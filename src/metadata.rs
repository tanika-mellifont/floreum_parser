use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileType {
    File,
    Dir,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Cursor {
    Forward,
    Backward,
    Start,
    End,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpenOptions {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub truncate: bool,
    pub create: bool,
    pub create_new: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Permit {
    pub read: bool,
    pub write: bool,
    pub append: bool,
    pub permit: bool,
    pub read_times: bool,
    pub write_times: bool,
    pub link: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Timestamp {
    pub secs: i64,
    pub nanos: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Metadata {
    pub permit: Permit,
    pub file_type: FileType,
    pub length: u64,
    pub accessed: Option<Timestamp>,
    pub modified: Option<Timestamp>,
    pub created: Option<Timestamp>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Entry<N: AsRef<str>> {
    pub metadata: Metadata,
    pub name: N,
}
