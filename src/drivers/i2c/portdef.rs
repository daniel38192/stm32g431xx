use crate::drivers::gpio;

const PB7_I2C1_SDA: gpio::Gpio = gpio::Gpio::new(gpio::GPIOPORT::GPIOB, 7);
const PB8_I2C1_SCL: gpio::Gpio = gpio::Gpio::new(gpio::GPIOPORT::GPIOB, 8);

const TYPE_I2C1_PORT: gpio::GpioConfig = gpio::GpioConfig {
    moder: gpio::config::MODER::Alternate,
    otyper: gpio::config::OTYPER::OpenDrain,
    ospeedr: gpio::config::OSPEEDR::VeryHighSpeed,
    pupdr: gpio::config::PUPDR::None,
    alt_func_select: Some(4)
};

impl crate::drivers::i2c::I2C {
    pub(crate) fn configure_pins() {
        PB7_I2C1_SDA.configure(TYPE_I2C1_PORT);
        PB8_I2C1_SCL.configure(TYPE_I2C1_PORT);
    }
}