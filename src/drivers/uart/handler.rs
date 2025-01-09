use stm32h5::stm32h562 as stm32;

use stm32::interrupt;
use crate::drivers::uart::{Serial, SerialError};

pub static mut DEFAULT_RECEIVE_HANDLER: fn(received_data: u8) = default_receive_handler;
pub static mut DEFAULT_ERROR_HANDLER: fn(error_submitted: SerialError) = default_error_handler;

fn default_receive_handler(_received_data: u8){
    /* ******* */
}

fn default_error_handler(_error_submitted: SerialError){
    /* ******* */
}

#[interrupt]
fn USART1() {
    unsafe {
        let dat = Serial::read_byte();
        if dat.is_ok() {
            DEFAULT_RECEIVE_HANDLER(dat.unwrap());
        } else if dat.is_err() {
            DEFAULT_ERROR_HANDLER(dat.unwrap_err())
        }
    }
}

