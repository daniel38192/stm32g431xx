use stm32g4::stm32g431;
use stm32g4::stm32g431::SPI1;
use crate::drivers::gpio;
const PB5_SPI3_MOSI: gpio::Gpio = gpio::Gpio {
    port: gpio::GPIOPORT::GPIOB,
    pin_number: 5
};

const PC10_SPI3_SCK: gpio::Gpio = gpio::Gpio {
    port: gpio::GPIOPORT::GPIOC,
    pin_number: 10
};

const PC11_SPI3_MISO: gpio::Gpio = gpio::Gpio {
    port: gpio::GPIOPORT::GPIOC,
    pin_number: 11
};

const SPI3_ALF_FUNC_SEL: u8 = 6;

const TYPE_SPI3_GPIO_PORT: gpio::GpioConfig = gpio::GpioConfig {
    moder: gpio::MODER::AlternateFunction,
    otyper: gpio::OTYPER::PushPull,
    ospeedr: gpio::OSPEEDR::VeryHigh,
    pupdr: gpio::PUPDR::None,
    alf_func_sel: Some(SPI3_ALF_FUNC_SEL)
};

struct SPI();

impl SPI {



    fn configure_pins() {
            // PB5 SPI3_MOSI  AF6
            PB5_SPI3_MOSI.configure(TYPE_SPI3_GPIO_PORT);
            // PC10 SPI3_SCK  AF6
            PC10_SPI3_SCK.configure(TYPE_SPI3_GPIO_PORT);
            // PC11 SPI3_MISO  AF6
            PC11_SPI3_MISO.configure(TYPE_SPI3_GPIO_PORT);
    }

    fn enable_rcc_clock() {
        unsafe {
            let rcc = &*stm32g431::RCC::ptr();
            rcc.apb1enr1.as_ptr().write(rcc.apb1enr1.as_ptr().read() | (1 << 15))
        }
    }
}