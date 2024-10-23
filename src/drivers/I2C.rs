
use stm32g4::stm32g431::I2C1;


pub struct I2C {

}

impl I2C {
    pub fn begin(){

    }

    pub fn configure_registers(){

    }

    pub fn configure_gpio(){
        // PA13 I2C1_SCL AF4
        // PA14 I2C1_SDA AF4
    }

    fn set_timing_prescaler(){
        // Set PRESC[3:0] bits in I2C_TIMINGR register.
        unsafe {
            let port = &*I2C1::ptr();
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (4 << 28));
        }
    }

    fn set_data_times(){
        unsafe {
            let port = &*I2C1::ptr();
            // set SCLDEL[3:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (11 << 20));
            // set SDADEL[3:0]: bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (2 << 16))
        }
    }

    fn set_master_mode_periods(){
        unsafe {
            let port = &*I2C1::ptr();
            // set SCLH[7:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (133 << 8));
            // set SCLL[7:0] bits in I2C_TIMINGR register.
            port.timingr.as_ptr().write(port.timingr.as_ptr().read() | (194 << 8));

            //0100 0000 1011 0010    10000101    11000010
        }
    }



}