#![allow(dead_code)]

use alloc::format;
use stm32g4::stm32g431;
use stm32g4::stm32g431::I2C1;
use crate::drivers::gpio;
use crate::drivers::gpio::GPIOPORT;
use crate::drivers::serial::Serial;
use crate::core::Conversions;

pub struct I2C {

}

const TYPE_I2C1_GPIO: gpio::GpioConfig = gpio::GpioConfig {
    moder: gpio::MODER::AlternateFunction,
    otyper: gpio::OTYPER::OpenDrain,
    ospeedr: gpio::OSPEEDR::VeryHigh,
    pupdr: gpio::PUPDR::PullUp,
    alf_func_sel: Some(4)
};

const PB8_I2C1_SCL: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOB,
    pin_number: 8,
};

const PB9_I2C1_SDA: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOB,
    pin_number: 9,
};

pub enum I2cError {
    TimeOut,
    PECErrorInReception,
    OverrunUnderrun,
    ArbitrationLost,
    BusError,
    NotAcknowledgedReceived,
}

pub struct MasterConfig {
    pub slave_address_to_send: u16,
    pub address_10_bit_mode: bool,
    pub i2c_10_bit_reading_procedure: Option<I2cHeaderOnlyReadDirection10Bit>
}

impl I2C {

    pub fn begin() {
        Self::enable_peripheral_clock_in_rcc();
        Self::configure_peripheral();
        Self::enable_peripheral()
    }

    pub fn begin_transmission(config: MasterConfig) {
        Self::master_initialization_address_phase(config);
    }

    pub fn transmit(data: &[u8]){

        let number_of_bytes_to_transmit = data.len() as u32;

        let port;
        unsafe {
             port = &*I2C1::ptr();
        }

        //sets the desired number of bytes and transfer direction
        Self::master_set_number_of_bytes_and_transfer_direction(number_of_bytes_to_transmit);

        //send a start condition
        Self::start_condition();

        Serial::println(format!("I2C1_CR1: {:b}", port.cr1.read().bits()).as_str());
        Serial::println(format!("I2C1_CR2: {:b}", port.cr2.read().bits()).as_str());
        Serial::println(format!("I2C1_ISR: {:b}", port.isr.read().bits()).as_str());

        while port.isr.read().tc().bit_is_clear(){
            for data_slice in data{
                Serial::println("sending FIRST byte");
                Self::write_into_tr_register(*data_slice);
            }
        }

    }

    pub fn end_transmission(){
        Self::stop_condition()
    }

    pub fn check_error_flags(){

    }

    pub fn write_into_tr_register(data_slice: u8){
        unsafe {
            let port = &*I2C1::ptr();

            while port.isr.read().txe().bit_is_clear(){}

            port.txdr.as_ptr().write(data_slice as u32);

        }
    }

    fn master_set_number_of_bytes_and_transfer_direction(number_of_bytes_to_transmit: u32){
        unsafe {
            let port = &*I2C1::ptr();
            //set number of bytes to transmit
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (number_of_bytes_to_transmit << 16));
            //set transfer direction (request a write)
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() & !(1 << 10));


        }
    }

    fn master_initialization_address_phase(config: MasterConfig){
        // run this code before a start condition

        /*
        * Master communication initialization (address phase)
        * To initiate the communication with a slave to address, set the following bitfields of the I2C_CR2 register:
        * • ADD10: addressing mode (7-bit or 10-bit)
        * • SADD[9:0]: slave address to send
        * • RD_WRN: transfer direction
        * • HEAD10R: in case of 10-bit address read, this bit determines whether the header only (for direction change) or the complete address sequence is sent.
        * • NBYTES[7:0]: the number of bytes to transfer; if equal to or greater than 255 bytes, the bitfield must initially be set to 0xFF.
        * Note: Changing these bitfields is not allowed as long as the START bit is set.
        */

        unsafe {
            let port = &*I2C1::ptr();

            //clear previous configurations

            port.cr2.as_ptr().write(0);

            //set addressing mode.
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (config.address_10_bit_mode.as_u32() << 11));
            //Set slave address to send
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | ((config.slave_address_to_send as u32 & 0x3FF) << 0));
            //Set 10 bit address reading procedure
            if config.i2c_10_bit_reading_procedure.is_some() {
                port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (config.i2c_10_bit_reading_procedure.as_ref().unwrap().as_u32() << 12));
            }

        }
    }

    /* Private methods */

    fn configure_peripheral(){
        Self::configure_gpio();
        unsafe {
            let port = &*I2C1::ptr();
            port.timingr.as_ptr().write(0x40B285C2);
        }
    }

    fn enable_peripheral(){
        unsafe {
            let port = &*I2C1::ptr();
            // Enable I2C
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 0));
        }
    }

    fn enable_peripheral_clock_in_rcc(){
        unsafe {
            let rcc = &*stm32g431::RCC::ptr();
            //Enable I2C1 in rcc clock
            rcc.apb1enr1.as_ptr().write(rcc.apb1enr1.as_ptr().read() | (1 << 21))
        }
    }

    fn start_condition(){
        unsafe {
            let port = &*I2C1::ptr();
            // Generate start condition
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (1 << 13));
        }
    }

    fn stop_condition(){
        unsafe {
            let port = &*I2C1::ptr();
            // Generate stop condition
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (1 << 14));
        }
    }

    fn configure_gpio(){
        // PB8 I2C1_SCL AF4
        // PB9 I2C1_SDA AF4
        PB8_I2C1_SCL.configure(TYPE_I2C1_GPIO);
        PB9_I2C1_SDA.configure(TYPE_I2C1_GPIO)
    }

}



/*
pub enum I2cTransferDirection {
    MasterRequestAWrite,
    MasterRequestARead
}
 */

pub enum I2cHeaderOnlyReadDirection10Bit {
    CompleteSlaveAddress,
    SevenBitsFirst
}

/*
impl Conversions for I2cTransferDirection {
    fn as_u32(&self) -> u32 {
        match self {
            I2cTransferDirection::MasterRequestAWrite => 0,
            I2cTransferDirection::MasterRequestARead => 1
        }
    }
}
 */

impl Conversions for I2cHeaderOnlyReadDirection10Bit {
    fn as_u32(&self) -> u32 {
        match self {
            I2cHeaderOnlyReadDirection10Bit::CompleteSlaveAddress => 0,
            I2cHeaderOnlyReadDirection10Bit::SevenBitsFirst => 1,
        }
    }
}

