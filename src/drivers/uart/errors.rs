use core::fmt::{Debug, Formatter};

#[derive(PartialEq)]
pub enum SerialError {
    NoDataFound,
    OverRun
}

impl Debug for SerialError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}