
use stm32g4::stm32g431::I2C1;
use crate::drivers::gpio;
use crate::drivers::gpio::GPIOPORT;

pub struct I2C {

}

const TYPE_I2C1_GPIO: gpio::GpioConfig = gpio::GpioConfig {
    moder: gpio::MODER::AlternateFunction,
    otyper: gpio::OTYPER::OpenDrain,
    ospeedr: gpio::OSPEEDR::VeryHigh,
    pupdr: gpio::PUPDR::PullUp,
    alf_func_sel: Some(4)
};

const PA13_I2C1_SDA: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 13,
};

const PA14_I2C1_SDA: gpio::Gpio = gpio::Gpio {
    port: GPIOPORT::GPIOA,
    pin_number: 14,
};

impl I2C {
    pub fn begin(){

    }

    pub fn transmit(){

    }

    pub fn configure_registers(){

    }

    pub fn configure_gpio(){
        // PA13 I2C1_SCL AF4
        // PA14 I2C1_SDA AF4
        PA13_I2C1_SDA.configure(TYPE_I2C1_GPIO);
        PA14_I2C1_SDA.configure(TYPE_I2C1_GPIO)
    }

    /* Private methods */

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

    fn start_condition(){
        unsafe {
            let port = &*I2C1::ptr();
            // Generate start condition
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (1 << 13));
            
        }
    }

    fn call_address(adress: u8){

    }

    fn stop_condition(){

    }


}