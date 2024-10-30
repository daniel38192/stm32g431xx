
use stm32g4::stm32g431::I2C1;
use crate::drivers::gpio;
use crate::drivers::gpio::GPIOPORT;

pub struct I2C {

}

const TYPE_I2C1_GPIO: gpio::GpioConfig = gpio::GpioConfig {
    moder: gpio::MODER::AlternateFunction,
    otyper: gpio::OTYPER::OpenDrain,
    ospeedr: gpio::OSPEEDR::VeryHigh,
    pupdr: gpio::PUPDR::PullUp,
    alf_func_sel: Some(4)
};

const PA13_I2C1_SDA: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 13,
};

const PA14_I2C1_SDA: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 14,
};

pub enum I2cError {
    BusBusy,
    TimeOut,
    PECErrorInReception,
    OverrunUnderrun,
    ArbitrationLost,
    BusError,
    NotAcknowledgedReceived,
}

struct MasterConfig {
    slave_address_to_send: u16,
    address_10_bit_mode: bool,
    i2c_10_bit_reading_procedure: Option<I2cHeaderOnlyReadDirection10Bit>
}

impl I2C {

    pub fn begin() {

    }

    pub fn transmit(){

    }

    fn master_initialization_address_phase(config: MasterConfig){
        // run this code before a start condition

        /**
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
            //Set transfer direction
            //port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (config.i2c_transfer_direction.as_u32() << 10));
            //Set 10 bit address reading procedure
            if config.i2c_10_bit_reading_procedure.is_some() {
                port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (config.i2c_10_bit_reading_procedure.as_ref().unwrap().as_u32() << 12));
            }
        }
    }

    fn configure_peripheral(){
        Self::configure_gpio();
        Self::set_timing_prescaler();
        Self::set_data_times();
        Self::set_master_mode_periods();
    }

    /* Private methods */
    fn start_condition(){
        unsafe {
            let port = &*I2C1::ptr();
            // Generate start condition
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (1 << 13));
        }
    }

    fn stop_condition(){

    }

    fn configure_gpio(){
        // PA13 I2C1_SCL AF4
        // PA14 I2C1_SDA AF4
        PA13_I2C1_SDA.configure(TYPE_I2C1_GPIO);
        PA14_I2C1_SDA.configure(TYPE_I2C1_GPIO)
    }

    fn set_timing_prescaler(){
        // Set PRESC[3:0] bits in I2C_TIMINGR register.
        unsafe {
            let port = &*I2C1::ptr();
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (4 << 28));
        }
    }

    fn set_data_times(){
        unsafe {
            let port = &*I2C1::ptr();
            // set SCLDEL[3:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (11 << 20));
            // set SDADEL[3:0]: bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (2 << 16))
        }
    }

    fn set_master_mode_periods(){
        unsafe {
            let port = &*I2C1::ptr();
            // set SCLH[7:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (133 << 8));
            // set SCLL[7:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (194 << 0));

            //0100 0000 1011 0010    10000101    11000010
        }
    }


}

trait Conversions {
    fn as_u32(&self) -> u32;
}

pub enum I2cTransferDirection {
    MasterRequestAWrite,
    MasterRequestARead
}

pub enum I2cHeaderOnlyReadDirection10Bit {
    CompleteSlaveAddress,
    SevenBitsFirst
}

impl Conversions for I2cTransferDirection {
    fn as_u32(&self) -> u32 {
        match self {
            I2cTransferDirection::MasterRequestAWrite => 0,
            I2cTransferDirection::MasterRequestARead => 1
        }
    }
}

impl Conversions for I2cHeaderOnlyReadDirection10Bit {
    fn as_u32(&self) -> u32 {
        match self {
            I2cHeaderOnlyReadDirection10Bit::CompleteSlaveAddress => 0,
            I2cHeaderOnlyReadDirection10Bit::SevenBitsFirst => 1,
        }
    }
}

impl Conversions for bool {
    fn as_u32(&self) -> u32 {
        match self {
            true  => 1,
            false => 0
        }
    }
}

