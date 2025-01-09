use crate::drivers::gpio::config::GpioConfig;

use crate::coreutils::Conversions;

use stm32h5::stm32h562::{gpioa, gpiob, gpioc, gpioh, gpioi};

pub trait ModifiableGpioBus {
    fn configure_into(&self, pin_number: u8, gpio_config: GpioConfig);
    fn set_into(&self, pin_number: u8, state: bool);
    fn read_into(&self, pin_number: u8) -> bool;
}

pub fn configure_into_gpio(port: &impl ModifiableGpioBus, pin_number: u8, gpio_config: GpioConfig) {
    port.configure_into(pin_number, gpio_config);
}

pub fn set_into_gpio(port: &impl ModifiableGpioBus, pin_number: u8, state: bool) {
    port.set_into(pin_number, state);
}

pub fn read_into_gpio(port: &impl ModifiableGpioBus, pin_number: u8) -> bool {
    port.read_into(pin_number)
}

macro_rules! gpio_modificator {
    (gpioi) => {
        impl ModifiableGpioBus for gpioi::RegisterBlock {
            fn configure_into(&self, pin_number: u8, gpio_config: GpioConfig) {
                unsafe {
                    //configure gpio MODER, GPIOx_MODER register
                    *self.moder().as_ptr() &= !(0x3 << (2 * (pin_number as u32)));
                    *self.moder().as_ptr() |=
                    (gpio_config.moder.as_u32() << (2 * (pin_number as u32)));

                    //configure gpio OTYPER, GPIOx_OTYPER
                    *self.otyper().as_ptr() |= (gpio_config.otyper.as_u32() << (pin_number as u32));

                    //configure gpio OSPEEDR, GPIOx_OSPEEDR
                    *self.ospeedr().as_ptr() &= !(0x3 << (2 * pin_number as u32));
                    *self.ospeedr().as_ptr() |=
                        (gpio_config.ospeedr.as_u32() << (2 * (pin_number as u32)));

                    //configure gpio PUPDR, GPIOx_PUPDR
                    *self.pupdr().as_ptr() &= !(0x3 << (2 * pin_number as u32));
                    *self.pupdr().as_ptr() |=
                        (gpio_config.pupdr.as_u32() << (2 * (pin_number as u32)));

                    //in case of alt_func_sel
                    if gpio_config.alt_func_select.is_some() {
                        *self.afrl().as_ptr() |= ((gpio_config.alt_func_select.unwrap() as u32)
                            << ((pin_number as u32) * 4));
                    }
                }
            }

            fn set_into(&self, pin_number: u8, state: bool) {
                unsafe {
                    if state {
                        self.bsrr()
                            .as_ptr()
                            .write_volatile(1 << (pin_number as u32));
                    } else {
                        self.bsrr()
                            .as_ptr()
                            .write_volatile(1 << ((pin_number as u32) + 16));
                    }
                }
            }

            fn read_into(&self, pin_number: u8) -> bool {
                (self.idr().read().bits() & (1 << (pin_number as u32))) > 0
            }
        }
    };

    ($port:ident) => {
        impl ModifiableGpioBus for $port::RegisterBlock {
            fn configure_into(&self, pin_number: u8, gpio_config: GpioConfig) {
                unsafe {
                    //configure gpio MODER, GPIOx_MODER register
                    *self.moder().as_ptr() &= !(0x3 << (2 * (pin_number as u32)));
                    *self.moder().as_ptr() |=
                        (gpio_config.moder.as_u32() << (2 * (pin_number as u32)));

                    //configure gpio OTYPER, GPIOx_OTYPER
                    *self.otyper().as_ptr() |= (gpio_config.otyper.as_u32() << (pin_number as u32));

                    //configure gpio OSPEEDR, GPIOx_OSPEEDR
                    *self.ospeedr().as_ptr() &= !(0x3 << (2 * (pin_number as u32)));
                    *self.ospeedr().as_ptr() |=
                        (gpio_config.ospeedr.as_u32() << (2 * (pin_number as u32)));

                    //configure gpio PUPDR, GPIOx_PUPDR
                    *self.pupdr().as_ptr() &= !(0x3 << (2 * (pin_number as u32)));
                    *self.pupdr().as_ptr() |=
                        (gpio_config.pupdr.as_u32() << (2 * (pin_number as u32)));

                    //in case of alt_func_sel

                    if gpio_config.alt_func_select.is_some() {
                        if pin_number <= 7 {
                            *self.afrl().as_ptr() |= ((gpio_config.alt_func_select.unwrap()
                                as u32)
                                << ((pin_number as u32) * 4));
                        } else {
                            *self.afrh().as_ptr() |= ((gpio_config.alt_func_select.unwrap()
                                as u32)
                                << (((pin_number as u32) * 4) - 32));
                        }
                    }
                }
            }

            fn set_into(&self, pin_number: u8, state: bool) {
                unsafe {
                    if state {
                        self.bsrr()
                            .as_ptr()
                            .write_volatile(1 << (pin_number as u32));
                    } else {
                        self.bsrr()
                            .as_ptr()
                            .write_volatile(1 << ((pin_number as u32) + 16));
                    }
                }
            }

            fn read_into(&self, pin_number: u8) -> bool {
                (self.idr().read().bits() & (1 << pin_number as u32)) > 0
            }
        }
    };
}

gpio_modificator!(gpioa);
gpio_modificator!(gpiob);
gpio_modificator!(gpioc);
gpio_modificator!(gpioh);
gpio_modificator!(gpioi);
