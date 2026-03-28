use core::ops::{BitAnd, BitAndAssign};

use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileType {
    File,
    Dir,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
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
impl BitAnd for Permit {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            read: self.read & rhs.read,
            write: self.write & rhs.write,
            append: self.append & rhs.append,
            permit: self.permit & rhs.permit,
            read_times: self.read_times & rhs.read_times,
            write_times: self.write_times & rhs.write_times,
            link: self.link & rhs.link,
        }
    }
}
impl BitAndAssign for Permit {
    fn bitand_assign(&mut self, rhs: Self) {
        self.read &= rhs.read;
        self.write &= rhs.write;
        self.append &= rhs.append;
        self.permit &= rhs.permit;
        self.read_times &= rhs.read_times;
        self.write_times &= rhs.write_times;
        self.link &= rhs.link;
    }
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
