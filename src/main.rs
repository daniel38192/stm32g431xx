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
use crate::drivers::I2C::{I2C, MasterConfig};

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

extern crate alloc;

const LED: Gpio = Gpio {port: GPIOPORT::GPIOC, pin_number: 6};

#[entry]
fn main() -> ! {

    system::system_init();

    let peripherals = stm32g4::stm32g431::Peripherals::take().unwrap();

    LED.configure(GpioConfig{
        moder: MODER::GeneralPurposeOutput,
        otyper: OTYPER::PushPull,
        ospeedr: OSPEEDR::Medium,
        pupdr: PUPDR::None,
        alf_func_sel: None
    });

    LED.high();

    Serial::println("I2c test");

    Serial::println(format!("I2C1_CR1: {:b}", peripherals.I2C1.cr1.read().bits()).as_str());
    Serial::println(format!("I2C1_CR2: {:b}", peripherals.I2C1.cr2.read().bits()).as_str());
    Serial::println(format!("I2C1_ISR: {:b}", peripherals.I2C1.isr.read().bits()).as_str());

    I2C::begin();

    Serial::println("I2c begin");

    I2C::begin_transmission(MasterConfig {
        slave_address_to_send: 0x4,
        address_10_bit_mode: false,
        i2c_10_bit_reading_procedure: None
    });

    Serial::println(format!("I2C1_CR1: {:b}", peripherals.I2C1.cr1.read().bits()).as_str());
    Serial::println(format!("I2C1_CR2: {:b}", peripherals.I2C1.cr2.read().bits()).as_str());
    Serial::println(format!("I2C1_ISR: {:b}", peripherals.I2C1.isr.read().bits()).as_str());

    let data_test = "hello";

    Serial::println("I2c transmitting");

    //1000000000000001

    I2C::transmit(data_test.as_bytes());

    I2C::end_transmission();

    Serial::println("I2c transition done");



    loop {

        LED.low();
        non_exact_time_delay(500000);
        LED.high();
        non_exact_time_delay(500000);

    }
}




