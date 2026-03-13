use core::{error::Error, fmt::Display};
use serde::{Deserialize, Serialize};
#[repr(u64)]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FloreumError {
    Other = 0,
    NotFound = 1,
    PermissionDenied = 2,
    ConnectionRefused = 3,
    ConnectionReset = 4,
    HostUnreachable = 5,
    NetworkUnreachable = 6,
    ConnectionAborted = 7,
    NotConnected = 8,
    AddrInUse = 9,
    AddrNotAvailable = 10,
    NetworkDown = 11,
    BrokenPipe = 12,
    AlreadyExists = 13,
    WouldBlock = 14,
    NotADirectory = 15,
    IsADirectory = 16,
    DirectoryNotEmpty = 17,
    ReadOnlyFilesystem = 18,
    FilesystemLoop = 19,
    StaleNetworkFileHandle = 20,
    InvalidInput = 21,
    InvalidData = 22,
    TimedOut = 23,
    WriteZero = 24,
    StorageFull = 25,
    NotSeekable = 26,
    QuotaExceeded = 27,
    FileTooLarge = 28,
    ResourceBusy = 29,
    ExecutableFileBusy = 30,
    Deadlock = 31,
    CrossesDevices = 32,
    TooManyLinks = 33,
    InvalidFilename = 34,
    ArgumentListTooLong = 35,
    Interrupted = 36,
    Unsupported = 37,
    UnexpectedEof = 38,
    OutOfMemory = 39,
    InProgress = 40,
}
impl Display for FloreumError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Error for FloreumError {}
