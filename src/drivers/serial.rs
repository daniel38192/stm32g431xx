#![allow(dead_code)]

use alloc::string::String;
use core::fmt::{Debug, Formatter};
use stm32g4::stm32g431;
use crate::system::APB2_FREQ;
use crate::drivers::gpio::{Gpio, GpioConfig, GPIOPORT, MODER, OSPEEDR, OTYPER, PUPDR};
use stm32g4::stm32g431::interrupt;

pub struct Serial {

}

#[derive(PartialEq)]
pub enum SerialError {
    NoDataFound,
    OverRun
}

impl Debug for SerialError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

const TYPE_SERIAL_USART1: GpioConfig = GpioConfig {
    moder: MODER::AlternateFunction,
    otyper: OTYPER::PushPull,
    ospeedr: OSPEEDR::VeryHigh,
    pupdr: PUPDR::None,
    alf_func_sel: Some(7),
};

const USART1_TX: Gpio = Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 9
};

const USART1_RX: Gpio = Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 10
};

impl Serial {
    pub fn configure(){
        Self::enable_rcc_clock();
        Self::configure_gpio();
        Self::enabled(false);
        Self::configure_basic_registers();
        Self::set_baudrate(115200);
        Self::enable_interrupts();
        Self::enabled(true);
    }

    fn enable_rcc_clock(){
        unsafe {
            let rcc = &*stm32g431::RCC::ptr();
            rcc.apb2enr.as_ptr().write(rcc.apb2enr.as_ptr().read() | (1 << 14))
        }
    }

    fn configure_basic_registers(){
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            //Enables transmitter an receiver
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | ((1 << 3)|(1 << 2)));
        }
    }

    fn configure_gpio(){
        USART1_TX.configure(TYPE_SERIAL_USART1);
        USART1_RX.configure(TYPE_SERIAL_USART1)
    }

    fn enable_interrupts(){
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            //RXNE Interrupt
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 5))
        }
    }

    fn disable_interrupts(){
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            //RXNE Interrupt
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() & !(1 << 5))
        }
    }

    pub fn set_baudrate(baudrate: u32){
        let usart_divider = APB2_FREQ / baudrate;
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            //Program baudrate in USART_BRR
            port.brr.as_ptr().write(usart_divider)
            //port.brr.as_ptr().write(1475)
        }
    }

    pub fn enabled(enable: bool){
        if enable {
           // Enable USART
            unsafe {
                let port = &*stm32g431::USART1::ptr();
                //Enables transmitter and receiver
                port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 0))
            }
        } else {
            // Disable USART
            unsafe {
                let port = &*stm32g431::USART1::ptr();
                //Disables transmitter and receiver
                port.cr1.as_ptr().write(port.cr1.as_ptr().read() & !(1 << 0))
            }
        }
    }

    pub fn print(d: &str){
        for &b in d.as_bytes() {
            Self::write_byte(b);
        }
    }

    pub fn println(d: &str){
        Self::print(d);
        Self::print("\n\r")
    }

    pub fn write_byte(byte: u8){
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            // Write byte into USART_TDR
            port.tdr.as_ptr().write(byte as u32);
            // Wait for TC bit in USART_ISR
            while port.isr.read().tc().bit_is_clear() { }
        }
    }

    pub fn read_byte() -> Result<u8, SerialError> {
        unsafe {
            let port = &*stm32g431::USART1::ptr();

            let port_isr = port.isr.read();

            // Check OVR bit flag in USART_ISR
            if port_isr.ore().bit_is_set() {
                return Err(SerialError::OverRun)
            }

            // Check RXNE bit in USART_ISR
            if port_isr.rxne().bit_is_clear() {
                return Err(SerialError::NoDataFound)
            }

            // if everything is fine ->
            Ok(port.rdr.as_ptr().read() as u8)
        }
    }

    pub fn clear_ore_flag() {
        unsafe {
            let port = &*stm32g431::USART1::ptr();
            // Write ORECF bit 3 in USART_ICR register in order to clear ORE flag in USART_ISR
            port.icr.as_ptr().write(port.icr.as_ptr().read() | (1 << 3))
        }
    }

    pub fn on_receive(f: fn()){
        Self::enabled(false);
        unsafe {
            DEFAULT_HANDLER = f;
        }
        Self::enabled(true);
    }

    pub fn read_input_text() -> String {
        Self::disable_interrupts();
        let mut read_buffer = String::new();

        loop {
            let data = Serial::read_byte();

            if data.is_ok() {

                if (data.as_ref().unwrap() == &0xA) || (data.as_ref().unwrap() == &0xD) {
                    // When the user press enter, that leads to a new line character or carriage return
                    break
                } else if data.as_ref().unwrap() == &0x7F {
                    // If user have made a mistake, and they want to delete a char.
                    Self::clear_last_char();
                    read_buffer.pop();
                } else {
                    // Clone the character (User friendly)
                    Serial::write_byte(*data.as_ref().unwrap());
                    read_buffer.push(data.unwrap() as char);
                }
            }
        }
        Serial::println("");
        Self::enable_interrupts();
        read_buffer

    }

    fn clear_last_char(){
        Serial::write_byte(0x8);
        Serial::write_byte(' ' as u8);
        Serial::print("\x1B[1D");
    }

}

static mut DEFAULT_HANDLER: fn() = default_handler;

fn default_handler(){
    /* ******* */
}

#[interrupt]
fn USART1() {
    unsafe {
        DEFAULT_HANDLER();
    }
}