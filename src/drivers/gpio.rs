#![allow(dead_code)]

pub mod config;
pub mod ports;

mod acts;
mod consts;

pub use config::GpioConfig;
pub use ports::GPIOPORT;
use stm32h5::stm32h562 as stm32;

pub struct Gpio {
    port: GPIOPORT,
    pin: u8,
}

impl Gpio {
    pub const fn new(port: GPIOPORT, pin: u8) -> Self {
        Gpio { port, pin }
    }

    pub fn configure(&self, config: GpioConfig) {
        self.enable_in_rcc();
        unsafe {
            match self.port {
                GPIOPORT::GPIOA => {
                    acts::configure_into_gpio(&*stm32::GPIOA::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOB => {
                    acts::configure_into_gpio(&*stm32::GPIOB::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOC => {
                    acts::configure_into_gpio(&*stm32::GPIOC::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOD => {
                    acts::configure_into_gpio(&*stm32::GPIOD::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOE => {
                    acts::configure_into_gpio(&*stm32::GPIOE::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOF => {
                    acts::configure_into_gpio(&*stm32::GPIOF::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOG => {
                    acts::configure_into_gpio(&*stm32::GPIOG::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOH => {
                    acts::configure_into_gpio(&*stm32::GPIOH::ptr(), self.pin, config)
                }
                GPIOPORT::GPIOI => {
                    acts::configure_into_gpio(&*stm32::GPIOI::ptr(), self.pin, config)
                }
            }
        }
    }

    pub fn high(&self){
        self.set(true)
    }

    pub fn low(&self){
        self.set(false)
    }

    pub fn set(&self, value: bool) {
        unsafe {
            match self.port {
                GPIOPORT::GPIOA => {
                    acts::set_into_gpio(&*stm32::GPIOA::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOB => {
                    acts::set_into_gpio(&*stm32::GPIOB::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOC => {
                    acts::set_into_gpio(&*stm32::GPIOC::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOD => {
                    acts::set_into_gpio(&*stm32::GPIOD::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOE => {
                    acts::set_into_gpio(&*stm32::GPIOE::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOF => {
                    acts::set_into_gpio(&*stm32::GPIOF::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOG => {
                    acts::set_into_gpio(&*stm32::GPIOG::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOH => {
                    acts::set_into_gpio(&*stm32::GPIOH::ptr(), self.pin, value)
                }
                GPIOPORT::GPIOI => {
                    acts::set_into_gpio(&*stm32::GPIOI::ptr(), self.pin, value)
                }
            }
        }
    }

    pub fn get(&self) -> bool {
        unsafe {
            match self.port {
                GPIOPORT::GPIOA => {
                    acts::read_into_gpio(&*stm32::GPIOA::ptr(), self.pin)
                }
                GPIOPORT::GPIOB => {
                    acts::read_into_gpio(&*stm32::GPIOB::ptr(), self.pin)
                }
                GPIOPORT::GPIOC => {
                    acts::read_into_gpio(&*stm32::GPIOC::ptr(), self.pin)
                }
                GPIOPORT::GPIOD => {
                    acts::read_into_gpio(&*stm32::GPIOD::ptr(), self.pin)
                }
                GPIOPORT::GPIOE => {
                    acts::read_into_gpio(&*stm32::GPIOE::ptr(), self.pin)
                }
                GPIOPORT::GPIOF => {
                    acts::read_into_gpio(&*stm32::GPIOF::ptr(), self.pin)
                }
                GPIOPORT::GPIOG => {
                    acts::read_into_gpio(&*stm32::GPIOG::ptr(), self.pin)
                }
                GPIOPORT::GPIOH => {
                    acts::read_into_gpio(&*stm32::GPIOH::ptr(), self.pin)
                }
                GPIOPORT::GPIOI => {
                    acts::read_into_gpio(&*stm32::GPIOI::ptr(), self.pin)
                }
            }
        }
    }

    fn enable_in_rcc(&self) {
        unsafe {
            let rcc = &*stm32::RCC::ptr();
            match self.port {
                GPIOPORT::GPIOA => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOAEN,
                GPIOPORT::GPIOB => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOBEN,
                GPIOPORT::GPIOC => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOCEN,
                GPIOPORT::GPIOD => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIODEN,
                GPIOPORT::GPIOE => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOEEN,
                GPIOPORT::GPIOF => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOFEN,
                GPIOPORT::GPIOG => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOGEN,
                GPIOPORT::GPIOH => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOHEN,
                GPIOPORT::GPIOI => *rcc.ahb2enr().as_ptr() |= consts::RCC_AHB2ENR_GPIOIEN
            }
        }
    }
}
