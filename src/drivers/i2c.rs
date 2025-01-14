mod portdef;
mod registers_init;
mod actions;

use alloc::format;
use stm32h5::stm32h562 as stm32;

use stm32::interrupt;

use crate::drivers::uart::Serial;


pub struct I2C;

pub struct I2CConfig {
    pub primary_address: Option<u16>,
    pub secondary_address: Option<u8>,

    pub fast_mode_plus_en: bool,
    pub enable_nostretch: bool,
    pub enable_slave_byte_control: bool,
    pub disable_analog_noise_filter: bool,

}

impl I2C {
    pub fn configure(config: &I2CConfig) {
        Self::enable_rcc_clock();
        Self::configure_pins();
        Self::configure_registers(config);
        Self::set_timingr();
        Self::enable_peripheral()
    }

    pub fn enable_peripheral(){
        let i2c = i2c1();
        i2c.cr1().modify(|_, w| w.pe().set_bit());
    }

    pub fn disable_peripheral(){
        let i2c = i2c1();
        i2c.cr1().modify(|_, w| w.pe().clear_bit());
    }

    fn enable_rcc_clock(){
        let rcc = unsafe { &(*stm32::RCC::ptr()) };
        rcc.apb1lenr().modify(|_, w | w.i2c1en().set_bit());
    }

}


#[interrupt]
fn I2C1_EV(){
    let i2c = i2c1();

    if i2c.isr().read().addr().bit_is_set() {
        //I2C adress matched event
        //Clear ADDR flag
        i2c.icr().write(|w| w.addrcf().set_bit());
        //Test
        Serial::println("I2C1 address matched!")
    };

    if i2c.isr().read().rxne().bit_is_set() {
        //I2C RXNE event
        let data_test = i2c.rxdr().read().rxdata().bits();
        Serial::println(format!("Received data: {:X}", data_test).as_str());
    }
}


fn i2c1() -> stm32::I2C1 {
    unsafe {
        stm32::I2C1::steal()
    }
}

