use crate::drivers::gpio::config::{MODER, OSPEEDR, OTYPER, PUPDR};
use crate::drivers::gpio::{Gpio, GpioConfig, GPIOPORT};

pub const RCC_APB2ENR_USART1EN: u32 = 1 << 14;
pub const APB2_FREQ: u32 = 32000000;

pub const TYPE_SERIAL_USART1: GpioConfig = GpioConfig {
    moder: MODER::Alternate,
    otyper: OTYPER::PushPull,
    ospeedr: OSPEEDR::VeryHighSpeed,
    pupdr: PUPDR::None,
    alt_func_select: Some(7),
};

pub const USART1_TX: Gpio = Gpio::new(GPIOPORT::GPIOA, 9);

pub const USART1_RX: Gpio = Gpio::new(GPIOPORT::GPIOA, 10);