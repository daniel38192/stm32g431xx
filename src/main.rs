#![no_std]
#![no_main]

mod system;
mod drivers;
mod core;

use cortex_m_rt::entry;
use crate::core::delay::non_exact_time_delay;
use crate::drivers::gpio::{Gpio, GpioConfig, GPIOPORT, MODER, OSPEEDR, OTYPER, PUPDR};
use crate::drivers::serial::Serial;
use crate::drivers::SPI::SPI;
#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

extern crate alloc;

const LED: Gpio = Gpio {port: GPIOPORT::GPIOC, pin_number: 6};

#[entry]
fn main() -> ! {

    system::system_init();

    //let peripherals = stm32g4::stm32g431::Peripherals::take().unwrap();

    LED.configure(GpioConfig{
        moder: MODER::GeneralPurposeOutput,
        otyper: OTYPER::PushPull,
        ospeedr: OSPEEDR::Medium,
        pupdr: PUPDR::None,
        alf_func_sel: None
    });

    LED.high();

    let slave_select = Gpio {
        port: GPIOPORT::GPIOB,
        pin_number: 7,
    };

    slave_select.configure(GpioConfig {
        moder: MODER::GeneralPurposeOutput,
        otyper: OTYPER::PushPull,
        ospeedr: OSPEEDR::Medium,
        pupdr: PUPDR::None,
        alf_func_sel: None
    });

    Serial::println("Spi test");

    slave_select.high();

    SPI::begin();

    slave_select.low();

   // slave_select.high();





    loop {

        Serial::print("Type data to transmit: ");

        let char: i32 = Serial::read_input_text().parse().unwrap();

        SPI::transfer(Some(char as u8));


    }
}




