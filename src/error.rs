use core::{error::Error, fmt::Display};
use serde::{Deserialize, Serialize};
#[repr(u64)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloreumError {
    Other,
    ProtocolViolation,
    InvalidUtf8,
    HostUsize,
    ServerUsize,
    Deadlock,
    OutOfMemory,
    Unsupported,
    InvalidDescriptor,
    DoesExist,
    DoesNotExist,
    NotAFile,
    NotADirectory,
    DirectoryNotEmpty,
    CannotExtend,
    BrokenLink,
    DeviceRejected,
}
impl Display for FloreumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for FloreumError {}
impl From<postcard::Error> for FloreumError {
    fn from(value: postcard::Error) -> Self {
        match value {
            postcard::Error::DeserializeBadUtf8 => Self::InvalidUtf8,
            _ => Self::ProtocolViolation,
        }
    }
}