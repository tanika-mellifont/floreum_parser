use core::{error::Error, fmt::Display};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloreumError {
    Truncation,
    LocalBitWidth,
    Utf8,
    Domain,
}
impl Display for FloreumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for FloreumError {}
