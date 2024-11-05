#![allow(dead_code)]

use stm32g4::stm32g431;
use stm32g4::stm32g431::interrupt;
use crate::core::Conversions;
use crate::drivers::gpio;

pub enum SpiMode {
    Mode0,
    Mode1,
    Mode2,
    Mode3,
}

pub enum SpiFrameFormat {
    MSBFirst,
    LSBFirst,
}

pub enum SpiClockDivider{
    DivideBy2,
    DivideBy4,
    DivideBy8,
    DivideBy16,
    DivideBy32,
    DivideBy64,
    DivideBy128,
    DivideBy256,
}

pub enum SpiDataSize {
    Bit4,
    Bit5,
    Bit6,
    Bit7,
    Bit8,
    Bit9,
    Bit10,
    Bit11,
    Bit12,
    Bit13,
    Bit14,
    Bit15,
    Bit16,
}

pub struct SpiSettings {
    spi_mode: SpiMode,
    spi_clock_divider: SpiClockDivider,
    spi_frame_format: SpiFrameFormat,
    spi_data_size: SpiDataSize
}

pub struct SPI();

impl SPI {
    pub fn begin(){
        Self::enable_rcc_clock();
        Self::configure_pins();
        Self::configure_specific_registers();
        Self::set_settings(SpiSettings {
            spi_mode: SpiMode::Mode0,
            spi_clock_divider: SpiClockDivider::DivideBy256,
            spi_frame_format: SpiFrameFormat::MSBFirst,
            spi_data_size: SpiDataSize::Bit8
        });
        Self::master_selection(true);
        Self::enable_peripheral()
    }

    pub fn transmit(data: Option<u16>) -> Option<u16> {
        unsafe {
            let port = &*stm32g431::SPI3::ptr();

            if data.is_some() {
                port.dr.as_ptr().write_volatile(data.unwrap() as u32);

                while port.sr.read().txe().bit_is_clear() { };
            }

            if port.sr.read().rxne().bit_is_set() {
                Some(port.dr.as_ptr().read() as u16)
            } else {
                None
            }

        }
    }

    pub fn set_settings(spi_settings: SpiSettings){
        Self::set_clock_divider(spi_settings.spi_clock_divider);
        Self::set_frame_format(spi_settings.spi_frame_format);
        Self::set_spi_mode(spi_settings.spi_mode);
        Self::set_data_size(spi_settings.spi_data_size)
    }

    pub fn on_receive(handler: fn(u16)){
        unsafe {
            ON_RECEIVE_HANDLER = handler;
        }
    }

}

impl SPI {
    fn set_clock_divider(div: SpiClockDivider) {
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (div.as_u32() << 3));
        }
    }

    fn set_frame_format(fmt: SpiFrameFormat) {
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (fmt.as_u32() << 7));
        }
    }

    fn set_spi_mode(mode: SpiMode) {
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (mode.as_u32() << 0));
        }
    }

    fn set_data_size(size: SpiDataSize) {
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr2.as_ptr().write(port.cr2.as_ptr().read() | (size.as_u32() << 8));
        }
    }

    fn master_selection(master_mode: bool){
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (master_mode.as_u32() << 2))
        }
    }

    fn enable_peripheral(){
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 6))
        }
    }

    fn configure_specific_registers(){
        //Set software NSS management SMM = 1 in SPI_CR1 register
        unsafe {
            let port = &*stm32g431::SPI3::ptr();
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 9));
            port.cr1.as_ptr().write(port.cr1.as_ptr().read() | (1 << 8))
        }
    }

    fn enable_rcc_clock() {
        unsafe {
            let rcc = &*stm32g431::RCC::ptr();
            rcc.apb1enr1.as_ptr().write(rcc.apb1enr1.as_ptr().read() | (1 << 15))
        }
    }

    fn configure_pins() {
        // PB5 SPI3_MOSI  AF6
        PB5_SPI3_MOSI.configure(TYPE_SPI3_GPIO_PORT);
        // PC10 SPI3_SCK  AF6
        PC10_SPI3_SCK.configure(TYPE_SPI3_GPIO_PORT);
        // PC11 SPI3_MISO  AF6
        PC11_SPI3_MISO.configure(TYPE_SPI3_GPIO_PORT);
    }
}

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

impl Conversions for SpiFrameFormat {
    fn as_u32(&self) -> u32 {
        match self {
            SpiFrameFormat::MSBFirst => 0,
            SpiFrameFormat::LSBFirst => 1,
        }
    }
}

impl Conversions for SpiMode {
    fn as_u32(&self) -> u32 {
        match self {
            SpiMode::Mode0 => 0,
            SpiMode::Mode1 => 1,
            SpiMode::Mode2 => 2,
            SpiMode::Mode3 => 3,
        }
    }
}

impl Conversions for SpiClockDivider {
    fn as_u32(&self) -> u32 {
        match self {
            SpiClockDivider::DivideBy2 => 0,
            SpiClockDivider::DivideBy4 => 1,
            SpiClockDivider::DivideBy8 => 2,
            SpiClockDivider::DivideBy16 => 3,
            SpiClockDivider::DivideBy32 => 4,
            SpiClockDivider::DivideBy64 => 5,
            SpiClockDivider::DivideBy128 => 6,
            SpiClockDivider::DivideBy256 => 7,
        }
    }
}

impl Conversions for SpiDataSize {
    fn as_u32(&self) -> u32 {
        match self {
            SpiDataSize::Bit4 => 3,
            SpiDataSize::Bit5 => 4,
            SpiDataSize::Bit6 => 5,
            SpiDataSize::Bit7 => 6,
            SpiDataSize::Bit8 => 7,
            SpiDataSize::Bit9 => 8,
            SpiDataSize::Bit10 => 9,
            SpiDataSize::Bit11 => 10,
            SpiDataSize::Bit12 => 11,
            SpiDataSize::Bit13 => 12,
            SpiDataSize::Bit14 => 13,
            SpiDataSize::Bit15 => 14,
            SpiDataSize::Bit16 => 15
        }
    }
}

static mut ON_RECEIVE_HANDLER: fn(u16) = spi3_on_receive_default_handler;

fn spi3_on_receive_default_handler(_received_data: u16) { }

#[interrupt]
fn SPI3() {
    let port = unsafe { &*stm32g431::SPI3::ptr() };

    if port.sr.read().rxne().bit_is_set() {
        let received_data = SPI::transmit(None).unwrap();
        unsafe {ON_RECEIVE_HANDLER(received_data)}
    }

}