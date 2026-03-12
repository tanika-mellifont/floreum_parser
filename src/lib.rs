#![no_std]
mod message;
mod metadata;
mod test;
pub use message::*;
pub use metadata::*;
pub use postcard::Error;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Request<N: AsRef<str>, C: AsRef<[u8]>> {
    Identify(RequestIdentify<N>),
    Open(RequestOpen<N>),
    Close(RequestClose),
    Metadata(RequestMetadata),
    Setmeta(RequestSetmeta),
    List(RequestList),
    Make(RequestMake<N>),
    Remove(RequestRemove<N>),
    Read(RequestRead),
    Write(RequestWrite<C>),
    Seek(RequestSeek),
    Tell(RequestTell),
    Copy(RequestCopy),
    Link(RequestLink<N>),
    Drop(RequestDrop),
}
macro_rules! request {
    ($variant:ident, $request:ty) => {
        impl<N: AsRef<str>, C: AsRef<[u8]>> From<$request> for Request<N, C> {
            fn from(value: $request) -> Self {
                Self::$variant(value)
            }
        }
        impl<N: AsRef<str>, C: AsRef<[u8]>> TryFrom<Request<N, C>> for $request {
            type Error = ();
            fn try_from(value: Request<N, C>) -> Result<Self, Self::Error> {
                match value {
                    Request::$variant(as_self) => Ok(as_self),
                    _ => Err(()),
                }
            }
        }
    };
}
impl<N: AsRef<str>, C: AsRef<[u8]>> Request<N, C> {
    pub fn to_slice<'ser>(&self, buf: &'ser mut [u8]) -> Result<&'ser mut [u8], postcard::Error>
    where
        N: Serialize,
        C: Serialize,
    {
        postcard::to_slice(self, buf)
    }
    pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, postcard::Error>
    where
        N: Serialize,
        C: Serialize,
    {
        postcard::to_extend(self, writer)
    }
    pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, postcard::Error>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
    {
        postcard::from_bytes(s)
    }
    pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), postcard::Error>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
    {
        postcard::take_from_bytes(s)
    }
}
request!(Identify, RequestIdentify<N>);
request!(Open, RequestOpen<N>);
request!(Close, RequestClose);
request!(Metadata, RequestMetadata);
request!(Setmeta, RequestSetmeta);
request!(List, RequestList);
request!(Make, RequestMake<N>);
request!(Remove, RequestRemove<N>);
request!(Read, RequestRead);
request!(Write, RequestWrite<C>);
request!(Seek, RequestSeek);
request!(Tell, RequestTell);
request!(Copy, RequestCopy);
request!(Link, RequestLink<N>);
request!(Drop, RequestDrop);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Response<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> {
    Identify(ResponseIdentify),
    Open(ResponseOpen),
    Close(ResponseClose),
    Metadata(ResponseMetadata),
    Setmeta(ResponseSetmeta),
    List(ResponseList<N, E>),
    Make(ResponseMake),
    Remove(ResponseRemove),
    Read(ResponseRead<C>),
    Write(ResponseWrite),
    Seek(ResponseSeek),
    Tell(ResponseTell),
    Copy(ResponseCopy),
    Link(ResponseLink),
    Drop(ResponseDrop),
}
macro_rules! response {
    ($variant:ident, $response:ty) => {
        impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> From<$response>
            for Response<N, C, E>
        {
            fn from(value: $response) -> Self {
                Self::$variant(value)
            }
        }
        impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> TryFrom<Response<N, C, E>>
            for $response
        {
            type Error = ();
            fn try_from(value: Response<N, C, E>) -> Result<Self, Self::Error> {
                match value {
                    Response::$variant(as_self) => Ok(as_self),
                    _ => Err(()),
                }
            }
        }
    };
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> Response<N, C, E> {
    pub fn to_slice<'ser>(&self, buf: &'ser mut [u8]) -> Result<&'ser mut [u8], postcard::Error>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        postcard::to_slice(self, buf)
    }
    pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, postcard::Error>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        postcard::to_extend(self, writer)
    }
    pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, postcard::Error>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::from_bytes(s)
    }
    pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), postcard::Error>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::take_from_bytes(s)
    }
}
response!(Identify, ResponseIdentify);
response!(Open, ResponseOpen);
response!(Close, ResponseClose);
response!(Metadata, ResponseMetadata);
response!(Setmeta, ResponseSetmeta);
response!(List, ResponseList<N, E>);
response!(Make, ResponseMake);
response!(Remove, ResponseRemove);
response!(Read, ResponseRead<C>);
response!(Write, ResponseWrite);
response!(Seek, ResponseSeek);
response!(Tell, ResponseTell);
response!(Copy, ResponseCopy);
response!(Link, ResponseLink);
response!(Drop, ResponseDrop);
