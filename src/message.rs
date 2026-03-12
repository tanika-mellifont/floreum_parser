use crate::{Create, Cursor, Entry, FileType, Metadata, Permit, Write};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseError {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestIdentify<N: AsRef<str>> {
    pub path: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseIdentify {
    pub file_type: FileType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestOpen<N: AsRef<str>> {
    pub expect: FileType,
    pub read: bool,
    pub write: Option<(Write, Create)>,
    pub path: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseOpen {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestClose {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseClose {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestMetadata {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub metadata: Metadata,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestSetmeta {
    pub descriptor: u64,
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
pub struct RequestMake<N: AsRef<str>> {
    pub descriptor: u64,
    pub file_type: FileType,
    pub permit: Permit,
    pub name: N,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseMake {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestRemove<N: AsRef<str>> {
    pub descriptor: u64,
    pub name: N,
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
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestWrite<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub content: C,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseWrite {
    pub length: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestSeek {
    pub descriptor: u64,
    pub cursor: Cursor,
    pub offset: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseSeek {}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestTell {
    pub descriptor: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResponseTell {
    pub offset: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestCopy {
    pub from: u64,
    pub to: u64,
    pub length: u64,
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
macro_rules! serde_base {
    ($message:ident) => {
        impl $message {
            pub fn to_slice<'a>(&self, buf: &'a mut [u8]) -> Result<&'a mut [u8], postcard::Error> {
                postcard::to_slice(self, buf)
            }
            pub fn from_bytes<'a>(s: &[u8]) -> Result<Self, postcard::Error> {
                postcard::from_bytes(s)
            }
        }
    };
}
macro_rules! serde_name {
    ($message:ident) => {
        impl<N: AsRef<str>> $message<N> {
            pub fn to_slice<'ser>(
                &self,
                buf: &'ser mut [u8],
            ) -> Result<&'ser mut [u8], postcard::Error>
            where
                N: Serialize,
            {
                postcard::to_slice(self, buf)
            }
            pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, postcard::Error>
            where
                N: Serialize,
            {
                postcard::to_extend(self, writer)
            }
            pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, postcard::Error>
            where
                N: Deserialize<'de>,
            {
                postcard::from_bytes(s)
            }
            pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), postcard::Error>
            where
                N: Deserialize<'de>,
            {
                postcard::take_from_bytes(s)
            }
        }
    };
}
macro_rules! serde_content {
    ($message:ident) => {
        impl<N: AsRef<[u8]>> $message<N> {
            pub fn to_slice<'ser>(
                &self,
                buf: &'ser mut [u8],
            ) -> Result<&'ser mut [u8], postcard::Error>
            where
                N: Serialize,
            {
                postcard::to_slice(self, buf)
            }
            pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, postcard::Error>
            where
                N: Serialize,
            {
                postcard::to_extend(self, writer)
            }
            pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, postcard::Error>
            where
                N: Deserialize<'de>,
            {
                postcard::from_bytes(s)
            }
            pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), postcard::Error>
            where
                N: Deserialize<'de>,
            {
                postcard::take_from_bytes(s)
            }
        }
    };
}
serde_base!(ResponseError);
serde_name!(RequestIdentify);
serde_base!(ResponseIdentify);
serde_name!(RequestOpen);
serde_base!(ResponseOpen);
serde_base!(RequestClose);
serde_base!(ResponseClose);
serde_base!(RequestMetadata);
serde_base!(ResponseMetadata);
serde_base!(RequestSetmeta);
serde_base!(ResponseSetmeta);
serde_base!(RequestList);
serde_name!(RequestMake);
serde_base!(ResponseMake);
serde_name!(RequestRemove);
serde_base!(ResponseRemove);
serde_base!(RequestRead);
serde_content!(ResponseRead);
serde_content!(RequestWrite);
serde_base!(ResponseWrite);
serde_base!(RequestSeek);
serde_base!(ResponseSeek);
serde_base!(RequestTell);
serde_base!(ResponseTell);
serde_base!(RequestCopy);
serde_base!(ResponseCopy);
serde_name!(RequestLink);
serde_base!(ResponseLink);
serde_base!(RequestDrop);
serde_base!(ResponseDrop);
impl<N: AsRef<str>, E: AsRef<[Entry<N>]>> ResponseList<N, E> {
    pub fn to_slice<'ser>(&self, buf: &'ser mut [u8]) -> Result<&'ser mut [u8], postcard::Error>
    where
        N: Serialize,
        E: Serialize,
    {
        postcard::to_slice(self, buf)
    }
    pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, postcard::Error>
    where
        N: Serialize,
        E: Serialize,
    {
        postcard::to_extend(self, writer)
    }
    pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, postcard::Error>
    where
        N: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::from_bytes(s)
    }
    pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), postcard::Error>
    where
        N: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::take_from_bytes(s)
    }
}
