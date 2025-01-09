#![allow(dead_code)]

pub mod errors;
mod portdef;
mod handler;

use stm32h5::stm32h562 as stm32;

pub use errors::SerialError;

use portdef::{TYPE_SERIAL_USART1, USART1_RX, USART1_TX, RCC_APB2ENR_USART1EN};
use crate::system::PCLK1_FREQ as USART_K_FREQ;

use handler::{DEFAULT_RECEIVE_HANDLER, DEFAULT_ERROR_HANDLER};

pub struct Serial;

impl Serial {
    pub fn configure() {
        Self::enable_rcc_clock();
        Self::configure_gpio();
        Self::enabled(false);
        Self::configure_basic_registers();
        Self::set_baudrate(115200);
        Self::enable_interrupts();
        Self::enabled(true);
    }

    fn enable_rcc_clock() {
        unsafe {
            let rcc = &*stm32::RCC::ptr();
            *rcc.apb2enr().as_ptr() |= RCC_APB2ENR_USART1EN;
        }
    }

    fn configure_basic_registers() {
        unsafe {
            let port = &*stm32::USART1::ptr();
            //Enables transmitter an receiver
            *port.cr1().as_ptr() |= (1 << 3) | (1 << 2)
        }
    }

    fn configure_gpio() {
        USART1_TX.configure(TYPE_SERIAL_USART1);
        USART1_RX.configure(TYPE_SERIAL_USART1)
    }

    fn enable_interrupts() {
        unsafe {
            let port = &*stm32::USART1::ptr();
            //RXNE Interrupt
            *port.cr1().as_ptr() |= 1 << 5
        }
    }

    fn disable_interrupts() {
        unsafe {
            let port = &*stm32::USART1::ptr();
            //RXNE Interrupt
            *port.cr1().as_ptr() &= !(1 << 5)
        }
    }

    pub fn set_baudrate(baudrate: u32) {
        let usart_divider = USART_K_FREQ / baudrate;
        unsafe {
            let port = &*stm32::USART1::ptr();
            //Program baudrate in USART_BRR
            port.brr().as_ptr().write(usart_divider)
            //port.brr.as_ptr().write(1475)
        }
    }

    pub fn enabled(enable: bool) {
        if enable {
            // Enable USART
            unsafe {
                let port = &*stm32::USART1::ptr();
                //Enables transmitter and receiver
                *port.cr1().as_ptr() |= 1 << 0
            }
        } else {
            // Disable USART
            unsafe {
                let port = &*stm32::USART1::ptr();
                //Disables transmitter and receiver
                *port.cr1().as_ptr() &= !(1 << 0)
            }
        }
    }

    pub fn print(d: &str) {
        for &b in d.as_bytes() {
            Self::write_byte(b);
        }
    }

    pub fn println(d: &str) {
        Self::print(d);
        Self::print("\n\r")
    }

    pub fn write_byte(byte: u8) {
        unsafe {
            let port = &*stm32::USART1::ptr();
            // Write byte into USART_TDR
            port.tdr().as_ptr().write(byte as u32);
            // Wait for TC bit in USART_ISR
            while port.isr().read().tc().bit_is_clear() {}
        }
    }

    pub fn read_byte() -> Result<u8, SerialError> {
        unsafe {
            let port = &*stm32::USART1::ptr();

            let port_isr = port.isr().read();

            // Check OVR bit flag in USART_ISR
            if port_isr.ore().bit_is_set() {
                return Err(SerialError::OverRun)
            }

            // Check RXNE bit in USART_ISR
            if port_isr.rxfne().bit_is_clear() {
                return Err(SerialError::NoDataFound)
            }

            // if everything is fine ->
            Ok(port.rdr().as_ptr().read() as u8)
        }
    }

    pub fn clear_ore_flag() {
        unsafe {
            let port = &*stm32::USART1::ptr();
            // Write ORECF bit 3 in USART_ICR register in order to clear ORE flag in USART_ISR
            *port.icr().as_ptr() |= 1 << 3
        }
    }

    pub fn on_receive(receive_handler: fn(received_data: u8)){
        unsafe {
            DEFAULT_RECEIVE_HANDLER = receive_handler;
        }
    }
    pub fn on_error(error_handler: fn(error_submitted: SerialError)){
        unsafe {
            DEFAULT_ERROR_HANDLER = error_handler;
        }
    }
}
