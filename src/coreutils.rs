pub trait Conversions {
    fn as_u32(&self) -> u32;
}

impl Conversions for bool {
    fn as_u32(&self) -> u32 {
        match self {
            true => 1,
            false => 0,
        }
    }
}
