use crate::{SeekFrom, Entry, FileType, Metadata, OpenOptions, Permit, error::FloreumError};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseError {
    pub error: FloreumError,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestOpen<N: AsRef<str>> {
    pub expect: FileType,
    pub options: OpenOptions,
    pub path: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseOpen {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestFlush {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseFlush {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestClose {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseClose {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestMetadata<N: AsRef<str>> {
    pub path: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub metadata: Metadata,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestSetmeta<N: AsRef<str>> {
    pub path: N,
    pub metadata: Metadata,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseSetmeta {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestList {
    pub descriptor: u64,
    pub length: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseList<N: AsRef<str>, E: AsRef<[Entry<N>]>> {
    pub entries: E,
    pub _phantom_n: PhantomData<N>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestRemove<N: AsRef<str>> {
    pub expect: FileType,
    pub all: bool,
    pub path: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseRemove {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestRead {
    pub descriptor: u64,
    pub length: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseRead<C: AsRef<[u8]>> {
    pub content: C,
    pub error: Option<FloreumError>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestWrite<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub content: C,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseWrite {
    pub error: Option<(u64, FloreumError)>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestSeek {
    pub descriptor: u64,
    pub from: SeekFrom,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseSeek {
    pub offset: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestCopy<N: AsRef<str>> {
    pub from: N,
    pub to: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseCopy {
    pub length: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestLink<N: AsRef<str>> {
    pub permit: Permit,
    pub above: bool,
    pub from: N,
    pub to: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseLink {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestDrop {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseDrop {}
