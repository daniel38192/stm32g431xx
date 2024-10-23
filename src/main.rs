#![no_std]
#![no_main]

mod system;
mod drivers;
mod core;

use alloc::format;
use cortex_m_rt::entry;
use crate::core::delay::non_exact_time_delay;
use crate::drivers::gpio::{Gpio, GpioConfig, GPIOPORT, MODER, OSPEEDR, OTYPER, PUPDR};
use crate::drivers::serial::{Serial, SerialError};

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

extern crate alloc;

const LED: Gpio = Gpio {port: GPIOPORT::GPIOC, pin_number: 6};

#[entry]
fn main() -> ! {

    system::system_init();

    LED.configure(GpioConfig{
        moder: MODER::GeneralPurposeOutput,
        otyper: OTYPER::PushPull,
        ospeedr: OSPEEDR::Medium,
        pupdr: PUPDR::None,
        alf_func_sel: None
    });

    LED.high();

    Serial::println("Hello, World!");
    Serial::println("STM34G431 rust test");
    Serial::println("You are receiving this information over USART1");

    Serial::println("Type first number: ");
    let n1: f32 = Serial::read_input_text().parse().expect("Text is not a valid integer");

    Serial::println("Type second number: ");
    let n2: f32 = Serial::read_input_text().parse().expect("Text is not a valid integer");

    Serial::println("In total is: ");
    Serial::println(format!("{}", n1 + n2).as_str());

    Serial::on_receive(serial_runtime);

    loop {

        LED.low();
        non_exact_time_delay(500000);
        LED.high();
        non_exact_time_delay(500000);

    }
}


fn serial_runtime(){
    let data = Serial::read_byte();

    if data.is_err() {
        if data == Err(SerialError::OverRun) {
            Serial::clear_ore_flag();
            Serial::println("Over run detected!")
        }
    }

    if data.is_ok() {
        let d = data.unwrap();
        Serial::println(format!("Received: {}", d as char).as_str());
        Serial::println(format!("As Hex: {:X}", d).as_str());
    }

    non_exact_time_delay(4000);
}





