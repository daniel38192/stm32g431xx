use stm32h5::stm32h562 as stm32;

use crate::drivers::i2c::I2CConfig;

impl crate::drivers::i2c::I2C {
    pub(crate) fn configure_registers(config: &I2CConfig) {
        let i2c = i2c1();

        if config.primary_address.is_some(){
            i2c.oar1().modify(|_, w | w.oa1().bits(config.primary_address.unwrap() << 1));

            i2c.oar1().modify(|_, w| w.oa1en().set_bit())
        }

        if config.secondary_address.is_some(){
            i2c.oar2().modify(|_, w | w.oa2().bits(config.secondary_address.unwrap() << 1));

            i2c.oar2().modify(|_, w| w.oa2en().set_bit())
        }

        //Set fast mode 20mA
        i2c.cr1().modify(|_, w| w.fmp().bit(config.fast_mode_plus_en));

        //Set clock no stretching
        i2c.cr1().modify(|_, w | w.nostretch().bit(config.enable_nostretch));

        //Set slave byte control
        i2c.cr1().modify(|_, w| w.sbc().bit(config.enable_slave_byte_control));

        //Set analog noise filter
        i2c.cr1().modify(|_, w| w.anfoff().bit(config.disable_analog_noise_filter));

        //Enable interrupts
        i2c.cr1().modify(|_, w | w.addrie().set_bit());

        i2c.cr1().modify(|_, w | w.rxie().set_bit());


    }

    pub(crate) fn set_timingr() {
        let i2c = i2c1();
        i2c.timingr().write(|w| unsafe { w.bits(0x10707DBC) });
    }
}

fn i2c1() -> stm32::I2C1 {
    unsafe {
        stm32::I2C1::steal()
    }
}