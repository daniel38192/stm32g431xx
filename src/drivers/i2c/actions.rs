use stm32h5::stm32h562 as stm32;

fn i2c1() -> stm32::I2C1 {
    unsafe {
        stm32::I2C1::steal()
    }
}

impl crate::drivers::i2c::I2C {
    //Master communication initialization (address phase)
    /*
    •ADD10: addressing mode (7-bit or 10-bit)
    •SADD[9:0]: slave address to send
    •RD_WRN: transfer direction
    •HEAD10R: in case of 10-bit address read, this bit determines whether the header only
    (for direction change) or the complete address sequence is sent.
    •NBYTES[7:0]: the number of bytes to transfer; if equal to or greater than 255 bytes, the
    bitfield must initially be set to 0xFF.
     */
    pub(crate) fn master_transmit(dev_address: u16, buffer: &[u8]) {
        let i2c = i2c1();
        let nbytes = buffer.len() as u8;

        //Set transfer direction (Write)
        i2c.cr2().modify(|_, w| w.rd_wrn().clear_bit());

        //Set slave address
        i2c.cr2().modify(|_, w| w.sadd().bits(dev_address << 1));

        //Set nbytes
        i2c.cr2().modify(|_, w| w.nbytes().bits(nbytes));

        //Enable autoend
        i2c.cr2().modify(|_, w| w.autoend().set_bit());

        //Set start condition
        i2c.cr2().modify(|_, w| w.start().set_bit());

        let mut index = 0;

        while i2c.isr().read().tc().bit_is_clear() {
            while i2c.isr().read().txis().bit_is_clear() {};

            i2c.txdr().write(|w| w.txdata().bits(buffer[index]));

            index += 1;
        }
    }
}