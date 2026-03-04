use core::{error::Error, fmt::Display};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloreumError {
    TruncatedBool,
    TruncatedU64,
    TruncatedHead,
    TruncatedDirection,
    TruncatedOrder,
    TruncatedState,
    TruncatedContentLen,
    TruncatedContent { expected: u64 },
    TruncatedStrLen,
    TruncatedStr { expected: u64 },
    TruncatedNamesIter,
    DomainBool { received: u8 },
    DomainHead { received: u64 },
    DomainDirection { received: u64 },
    DomainOrder { received: u64 },
    LocalBitWidth,
    Utf8,
    UnknownKind {kind: u64},
}
impl Display for FloreumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for FloreumError {}
