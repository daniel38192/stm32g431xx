#![allow(dead_code)]

use stm32g4::stm32g431;

pub enum MODER {
    Input,
    GeneralPurposeOutput,
    AlternateFunction,
    AnalogMode
}

impl MODER {
    pub fn to_u32(&self) -> u32{
        match self {
            MODER::Input => 0,
            MODER::GeneralPurposeOutput => 1,
            MODER::AlternateFunction => 2,
            MODER::AnalogMode => 3
        }
    }
}

pub enum OTYPER {
    PushPull,
    OpenDrain
}

impl OTYPER {
    pub fn to_u32(&self) -> u32 {
        match self {
            OTYPER::PushPull => 0,
            OTYPER::OpenDrain => 1
        }
    }
}

pub enum OSPEEDR {
    Low,
    Medium,
    High,
    VeryHigh
}

impl OSPEEDR {
    pub fn to_u32(&self) -> u32 {
        match self {
            OSPEEDR::Low => 0,
            OSPEEDR::Medium => 1,
            OSPEEDR::High => 2,
            OSPEEDR::VeryHigh => 3
        }
    }
}

pub enum PUPDR {
    None,
    PullUp,
    PullDown
}

impl PUPDR {
    pub fn to_u32(&self) -> u32 {
        match self {
            PUPDR::None => 0,
            PUPDR::PullUp => 1,
            PUPDR::PullDown => 2,
        }
    }
}

pub struct GpioConfig {
    pub moder: MODER,
    pub otyper: OTYPER,
    pub ospeedr: OSPEEDR,
    pub pupdr: PUPDR,
    pub alf_func_sel: Option<u8>
}

#[derive(Clone)]
pub enum GPIOPORT {
    GPIOA,
    GPIOB,
    GPIOC,
    GPIOD,
    GPIOE,
    GPIOF,
    GPIOG,
}

#[derive(Clone)]
pub struct Gpio {
    pub port: GPIOPORT,
    pub pin_number: u32
}

impl Gpio {
    pub fn high(&self){
        self.set(true)
    }

    pub fn low(&self){
        self.set(false)
    }

    pub fn set(&self, value: bool){
        match self.port {
            GPIOPORT::GPIOA => turn_in_gpio_a(self.pin_number, value),
            GPIOPORT::GPIOB => turn_in_gpio_b(self.pin_number, value),
            GPIOPORT::GPIOC => turn_in_gpio_c(self.pin_number, value),
            _ => {}
        }
    }

    pub fn get(&self) -> bool {
        match self.port {
            GPIOPORT::GPIOA => read_in_gpio_a(self.pin_number),
            GPIOPORT::GPIOB => read_in_gpio_b(self.pin_number),
            GPIOPORT::GPIOC => read_in_gpio_c(self.pin_number),
            _ => {false}
        }
    }

    pub fn configure(&self, config: GpioConfig){
        self.enable_in_rcc();
        match self.port {
            GPIOPORT::GPIOA => configure_in_gpio_a(config, self.pin_number),
            GPIOPORT::GPIOB => configure_in_gpio_b(config, self.pin_number),
            GPIOPORT::GPIOC => configure_in_gpio_c(config, self.pin_number),
            _ => {}
        }
    }

    fn enable_in_rcc(&self){
        unsafe  {
            let rcc = &*stm32g431::RCC::ptr();
            match self.port {
                GPIOPORT::GPIOA => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 0)),
                GPIOPORT::GPIOB => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 1)),
                GPIOPORT::GPIOC => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 2)),
                GPIOPORT::GPIOD => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 3)),
                GPIOPORT::GPIOE => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 4)),
                GPIOPORT::GPIOF => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 5)),
                GPIOPORT::GPIOG => rcc.ahb2enr.as_ptr().write(rcc.ahb2enr.as_ptr().read() | (1 << 6)),
            }
        }
    }
}

fn configure_in_gpio_a(config: GpioConfig, pin_number: u32){
    unsafe {
        let port = &*stm32g431::GPIOA::ptr();
        // Set MODER
        port.moder.as_ptr().write(port.moder.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.moder.as_ptr().write(port.moder.as_ptr().read() | (config.moder.to_u32() << (pin_number*2)));
        // Set OTYPER
        port.otyper.as_ptr().write(port.otyper.as_ptr().read() | (config.otyper.to_u32() << pin_number));
        // Set OSPEEDR
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() | (config.ospeedr.to_u32() << (pin_number*2)));
        // Set PUPDR
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() | (config.pupdr.to_u32() << (pin_number*2)));

        if config.alf_func_sel.is_some() {
            if pin_number > 7 {
                port.afrh.as_ptr().write(port.afrh.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << ((pin_number * 4) - 32)))
            } else {
                port.afrl.as_ptr().write(port.afrl.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << (pin_number * 4)))
            }
        }
    }
}

fn configure_in_gpio_b(config: GpioConfig, pin_number: u32){
    unsafe {
        let port = &*stm32g431::GPIOB::ptr();
        // Set MODER
        port.moder.as_ptr().write(port.moder.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.moder.as_ptr().write(port.moder.as_ptr().read() | (config.moder.to_u32() << (pin_number*2)));
        // Set OTYPER
        port.otyper.as_ptr().write(port.otyper.as_ptr().read() | (config.otyper.to_u32() << pin_number));
        // Set OSPEEDR
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() | (config.ospeedr.to_u32() << (pin_number*2)));
        // Set PUPDR
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() | (config.pupdr.to_u32() << (pin_number*2)));

        if config.alf_func_sel.is_some() {
            if pin_number > 7 {
                port.afrh.as_ptr().write(port.afrh.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << ((pin_number * 4) - 32)))
            } else {
                port.afrl.as_ptr().write(port.afrl.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << (pin_number * 4)))
            }
        }
    }
}

fn configure_in_gpio_c(config: GpioConfig, pin_number: u32){
    unsafe {
        let port = &*stm32g431::GPIOC::ptr();
        // Set MODER
        port.moder.as_ptr().write(port.moder.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.moder.as_ptr().write(port.moder.as_ptr().read() | (config.moder.to_u32() << (pin_number*2)));
        // Set OTYPER
        port.otyper.as_ptr().write(port.otyper.as_ptr().read() | (config.otyper.to_u32() << pin_number));
        // Set OSPEEDR
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.ospeedr.as_ptr().write(port.ospeedr.as_ptr().read() | (config.ospeedr.to_u32() << (pin_number*2)));
        // Set PUPDR
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() & !(0x3 << (pin_number*2)));
        port.pupdr.as_ptr().write(port.pupdr.as_ptr().read() | (config.pupdr.to_u32() << (pin_number*2)));

        if config.alf_func_sel.is_some() {
            if pin_number > 7 {
                port.afrh.as_ptr().write(port.afrh.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << ((pin_number * 4) - 32)))
            } else {
                port.afrl.as_ptr().write(port.afrl.as_ptr().read() | ((config.alf_func_sel.unwrap() as u32) << (pin_number * 4)))
            }
        }
    }
}

fn turn_in_gpio_a(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32g431::GPIOA::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}

fn turn_in_gpio_b(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32g431::GPIOB::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}
fn turn_in_gpio_c(pin_number: u32, value: bool){
    unsafe {
        let port = &*stm32g431::GPIOC::ptr();
        if value {
            port.bsrr.as_ptr().write(1 << pin_number)
        } else {
            port.bsrr.as_ptr().write(1 << pin_number + 16)
        }
    }
}

fn read_in_gpio_a(pin_number: u32) -> bool {
    unsafe {
        let port = &*stm32g431::GPIOA::ptr();
        (port.idr.as_ptr().read() & (1 << pin_number)) > 0
    }
}
fn read_in_gpio_b(pin_number: u32) -> bool {
    unsafe {
        let port = &*stm32g431::GPIOB::ptr();
        (port.idr.as_ptr().read() & (1 << pin_number)) > 0
    }
}

fn read_in_gpio_c(pin_number: u32) -> bool {
    unsafe {
        let port = &*stm32g431::GPIOC::ptr();
        (port.idr.as_ptr().read() & (1 << pin_number)) > 0
    }
}
