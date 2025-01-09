#![no_std]
#![no_main]

mod coreutils;
mod drivers;
mod system;

extern crate panic_halt;

use alloc::format;
use cortex_m_rt::entry;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: emballoc::Allocator<4096> = emballoc::Allocator::new();

use crate::system::system_init;

use drivers::gpio::{Gpio, GPIOPORT, GpioConfig, config::{MODER, OTYPER, OSPEEDR, PUPDR}};
use drivers::uart::Serial;
use drivers::uart::SerialError;

use core::arch::asm;

use drivers::adc;


#[entry]
fn main() -> ! {

    system_init();

    let peripherals = stm32h5::stm32h562::Peripherals::take().unwrap();

    let led = Gpio::new(GPIOPORT::GPIOB, 2);

    led.configure(GpioConfig {
        moder: MODER::GeneralPurposeOutput,
        otyper: OTYPER::PushPull,
        ospeedr: OSPEEDR::LowSpeed,
        pupdr: PUPDR::None,
        alt_func_select: None
    });

    Serial::configure();

    Serial::on_receive(usart_handler);
    Serial::on_error(usart_error_handler);

    Serial::println("uart test");

    adc::adc1_test_configure();

    Serial::println(format!("ADC_ISR: {:X}", peripherals.ADC1.isr().read().bits()).as_str());
    Serial::println(format!("ADC_CR: {:X}", peripherals.ADC1.cr().read().bits()).as_str());
    Serial::println(format!("ADC_SQR1: {:X}", peripherals.ADC1.sqr1().read().bits()).as_str());
    Serial::println(format!("GPIOB_MODER: {:X}", peripherals.GPIOB.moder().read().bits()).as_str());
    Serial::println(format!("RCC_AHB2ENR: {:X}", peripherals.RCC.ahb2enr().read().bits()).as_str());

    loop {
        let receivedval = adc::adc1_get_value();

        Serial::println(format!("Received val: {}", receivedval).as_str());

        wait()
    }
}

fn wait(){
    unsafe {
        for _i in 1..100000 {
            asm!("nop");
        }
    }
}

fn usart_handler(received_data: u8){
    Serial::print("Received: ");
    Serial::write_byte(received_data);
    Serial::println("");
}

fn usart_error_handler(error: SerialError){
    if error == SerialError::OverRun {
        Serial::clear_ore_flag()
    }
}

//rustc --target thumbv8m.main-none-eabihf -- -C link-arg=-Tlink.x
