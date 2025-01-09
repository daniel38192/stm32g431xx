use crate::coreutils::Conversions;

pub enum MODER {
    Input,
    GeneralPurposeOutput,
    Alternate,
    Analog,
}

impl Conversions for MODER {
    fn as_u32(&self) -> u32 {
        match self {
            MODER::Input => 0,
            MODER::GeneralPurposeOutput => 1,
            MODER::Alternate => 2,
            MODER::Analog => 3,
        }
    }
}

pub enum OTYPER {
    PushPull,
    OpenDrain,
}

impl Conversions for OTYPER {
    fn as_u32(&self) -> u32 {
        match self {
            OTYPER::PushPull => 0,
            OTYPER::OpenDrain => 1,
        }
    }
}

pub enum OSPEEDR {
    LowSpeed,
    MediumSpeed,
    HighSpeed,
    VeryHighSpeed,
}

impl Conversions for OSPEEDR {
    fn as_u32(&self) -> u32 {
        match self {
            OSPEEDR::LowSpeed => 0,
            OSPEEDR::MediumSpeed => 1,
            OSPEEDR::HighSpeed => 2,
            OSPEEDR::VeryHighSpeed => 3,
        }
    }
}

pub enum PUPDR {
    None,
    PullUp,
    PullDown,
}

impl Conversions for PUPDR {
    fn as_u32(&self) -> u32 {
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
    pub alt_func_select: Option<u8>,
}
