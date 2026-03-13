#![no_std]
extern crate alloc;
mod error;
mod message;
mod metadata;
use alloc::vec::Vec;
pub use error::*;
pub use message::*;
pub use metadata::*;
use postcard::experimental::serialized_size;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Message<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> {
    RequestOpen(RequestOpen<N>),
    RequestFlush(RequestFlush),
    RequestClose(RequestClose),
    RequestMetadata(RequestMetadata<N>),
    RequestSetmeta(RequestSetmeta<N>),
    RequestList(RequestList),
    RequestRemove(RequestRemove<N>),
    RequestRead(RequestRead),
    RequestWrite(RequestWrite<C>),
    RequestSeek(RequestSeek),
    RequestTell(RequestTell),
    RequestCopy(RequestCopy<N>),
    RequestLink(RequestLink<N>),
    RequestDrop(RequestDrop),
    ResponseError(ResponseError),
    ResponseOpen(ResponseOpen),
    ResponseFlush(ResponseFlush),
    ResponseClose(ResponseClose),
    ResponseMetadata(ResponseMetadata),
    ResponseSetmeta(ResponseSetmeta),
    ResponseList(ResponseList<N, E>),
    ResponseRemove(ResponseRemove),
    ResponseRead(ResponseRead<C>),
    ResponseWrite(ResponseWrite),
    ResponseSeek(ResponseSeek),
    ResponseTell(ResponseTell),
    ResponseCopy(ResponseCopy),
    ResponseLink(ResponseLink),
    ResponseDrop(ResponseDrop),
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> Message<N, C, E> {
    pub fn length(&self) -> Result<usize, FloreumError>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        serialized_size(self).map_err(|err| err.into())
    }
    pub fn from_bytes<'de>(s: &'de [u8]) -> Result<Self, FloreumError>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::from_bytes(s).map_err(|err| err.into())
    }
    pub fn take_from_bytes<'de>(s: &'de [u8]) -> Result<(Self, &'de [u8]), FloreumError>
    where
        N: Deserialize<'de>,
        C: Deserialize<'de>,
        E: Deserialize<'de>,
    {
        postcard::take_from_bytes(s).map_err(|err| err.into())
    }
    pub fn to_allocvec(&self) -> Result<Vec<u8>, FloreumError>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        postcard::to_allocvec(self).map_err(|err| err.into())
    }
    pub fn to_extend<W: Extend<u8>>(&self, writer: W) -> Result<W, FloreumError>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        postcard::to_extend(self, writer).map_err(|err| err.into())
    }
    pub fn to_slice<'ser>(&self, buf: &'ser mut [u8]) -> Result<&'ser mut [u8], FloreumError>
    where
        N: Serialize,
        C: Serialize,
        E: Serialize,
    {
        postcard::to_slice(self, buf).map_err(|err| err.into())
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Request<N: AsRef<str>, C: AsRef<[u8]>> {
    Open(RequestOpen<N>),
    Flush(RequestFlush),
    Close(RequestClose),
    Metadata(RequestMetadata<N>),
    Setmeta(RequestSetmeta<N>),
    List(RequestList),
    Remove(RequestRemove<N>),
    Read(RequestRead),
    Write(RequestWrite<C>),
    Seek(RequestSeek),
    Tell(RequestTell),
    Copy(RequestCopy<N>),
    Link(RequestLink<N>),
    Drop(RequestDrop),
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> From<Request<N, C>> for Message<N, C, E> {
    fn from(value: Request<N, C>) -> Self {
        match value {
            Request::Open(open) => Self::RequestOpen(open),
            Request::Flush(flush) => Self::RequestFlush(flush),
            Request::Close(close) => Self::RequestClose(close),
            Request::Metadata(metadata) => Self::RequestMetadata(metadata),
            Request::Setmeta(setmeta) => Self::RequestSetmeta(setmeta),
            Request::List(list) => Self::RequestList(list),
            Request::Remove(remove) => Self::RequestRemove(remove),
            Request::Read(read) => Self::RequestRead(read),
            Request::Write(write) => Self::RequestWrite(write),
            Request::Seek(seek) => Self::RequestSeek(seek),
            Request::Tell(tell) => Self::RequestTell(tell),
            Request::Copy(copy) => Self::RequestCopy(copy),
            Request::Link(link) => Self::RequestLink(link),
            Request::Drop(drop) => Self::RequestDrop(drop),
        }
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> TryFrom<Message<N, C, E>>
    for Request<N, C>
{
    type Error = Message<N, C, E>;
    fn try_from(value: Message<N, C, E>) -> Result<Self, Self::Error> {
        Ok(match value {
            Message::RequestOpen(open) => Self::Open(open),
            Message::RequestFlush(flush) => Self::Flush(flush),
            Message::RequestClose(close) => Self::Close(close),
            Message::RequestMetadata(metadata) => Self::Metadata(metadata),
            Message::RequestSetmeta(setmeta) => Self::Setmeta(setmeta),
            Message::RequestList(list) => Self::List(list),
            Message::RequestRemove(remove) => Self::Remove(remove),
            Message::RequestRead(read) => Self::Read(read),
            Message::RequestWrite(write) => Self::Write(write),
            Message::RequestSeek(seek) => Self::Seek(seek),
            Message::RequestTell(tell) => Self::Tell(tell),
            Message::RequestCopy(copy) => Self::Copy(copy),
            Message::RequestLink(link) => Self::Link(link),
            Message::RequestDrop(drop) => Self::Drop(drop),
            other => return Err(other),
        })
    }
}
macro_rules! request {
    ($request_variant:ident, $message_variant:ident, $request:ty) => {
        impl<N: AsRef<str>, C: AsRef<[u8]>> From<$request> for Request<N, C> {
            fn from(value: $request) -> Self {
                Self::$request_variant(value)
            }
        }
        impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> TryFrom<Message<N, C, E>>
            for $request
        {
            type Error = Message<N, C, E>;
            fn try_from(value: Message<N, C, E>) -> Result<Self, Self::Error> {
                match value {
                    Message::$message_variant(as_self) => Ok(as_self),
                    other => Err(other),
                }
            }
        }
    };
}
request!(Open, RequestOpen, RequestOpen<N>);
request!(Flush, RequestFlush, RequestFlush);
request!(Close, RequestClose, RequestClose);
request!(Metadata, RequestMetadata, RequestMetadata<N>);
request!(Setmeta, RequestSetmeta, RequestSetmeta<N>);
request!(List, RequestList, RequestList);
request!(Remove, RequestRemove, RequestRemove<N>);
request!(Read, RequestRead, RequestRead);
request!(Write, RequestWrite, RequestWrite<C>);
request!(Seek, RequestSeek, RequestSeek);
request!(Tell, RequestTell, RequestTell);
request!(Copy, RequestCopy, RequestCopy<N>);
request!(Link, RequestLink, RequestLink<N>);
request!(Drop, RequestDrop, RequestDrop);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Response<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> {
    Error(ResponseError),
    Open(ResponseOpen),
    Flush(ResponseFlush),
    Close(ResponseClose),
    Metadata(ResponseMetadata),
    Setmeta(ResponseSetmeta),
    List(ResponseList<N, E>),
    Remove(ResponseRemove),
    Read(ResponseRead<C>),
    Write(ResponseWrite),
    Seek(ResponseSeek),
    Tell(ResponseTell),
    Copy(ResponseCopy),
    Link(ResponseLink),
    Drop(ResponseDrop),
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> From<Response<N, C, E>>
    for Message<N, C, E>
{
    fn from(value: Response<N, C, E>) -> Self {
        match value {
            Response::Error(error) => Self::ResponseError(error),
            Response::Open(open) => Self::ResponseOpen(open),
            Response::Flush(flush) => Self::ResponseFlush(flush),
            Response::Close(close) => Self::ResponseClose(close),
            Response::Metadata(metadata) => Self::ResponseMetadata(metadata),
            Response::Setmeta(setmeta) => Self::ResponseSetmeta(setmeta),
            Response::List(list) => Self::ResponseList(list),
            Response::Remove(remove) => Self::ResponseRemove(remove),
            Response::Read(read) => Self::ResponseRead(read),
            Response::Write(write) => Self::ResponseWrite(write),
            Response::Seek(seek) => Self::ResponseSeek(seek),
            Response::Tell(tell) => Self::ResponseTell(tell),
            Response::Copy(copy) => Self::ResponseCopy(copy),
            Response::Link(link) => Self::ResponseLink(link),
            Response::Drop(drop) => Self::ResponseDrop(drop),
        }
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> TryFrom<Message<N, C, E>>
    for Response<N, C, E>
{
    type Error = Message<N, C, E>;
    fn try_from(
        value: Message<N, C, E>,
    ) -> Result<Self, <Response<N, C, E> as TryFrom<Message<N, C, E>>>::Error> {
        Ok(match value {
            Message::ResponseError(error) => Self::Error(error),
            Message::ResponseOpen(open) => Self::Open(open),
            Message::ResponseFlush(flush) => Self::Flush(flush),
            Message::ResponseClose(close) => Self::Close(close),
            Message::ResponseMetadata(metadata) => Self::Metadata(metadata),
            Message::ResponseSetmeta(setmeta) => Self::Setmeta(setmeta),
            Message::ResponseList(list) => Self::List(list),
            Message::ResponseRemove(remove) => Self::Remove(remove),
            Message::ResponseRead(read) => Self::Read(read),
            Message::ResponseWrite(write) => Self::Write(write),
            Message::ResponseSeek(seek) => Self::Seek(seek),
            Message::ResponseTell(tell) => Self::Tell(tell),
            Message::ResponseCopy(copy) => Self::Copy(copy),
            Message::ResponseLink(link) => Self::Link(link),
            Message::ResponseDrop(drop) => Self::Drop(drop),
            other => return Err(other),
        })
    }
}
macro_rules! response {
    ($response_variant:ident, $message_variant:ident, $response:ty) => {
        impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> From<$response>
            for Response<N, C, E>
        {
            fn from(value: $response) -> Self {
                Self::$response_variant(value)
            }
        }
        impl<N: AsRef<str>, C: AsRef<[u8]>, E: AsRef<[Entry<N>]>> TryFrom<Message<N, C, E>>
            for $response
        {
            type Error = Message<N, C, E>;
            fn try_from(value: Message<N, C, E>) -> Result<Self, Self::Error> {
                match value {
                    Message::$message_variant(as_self) => Ok(as_self),
                    other => Err(other),
                }
            }
        }
    };
}
response!(Error, ResponseError, ResponseError);
response!(Open, ResponseOpen, ResponseOpen);
response!(Flush, ResponseFlush, ResponseFlush);
response!(Close, ResponseClose, ResponseClose);
response!(Metadata, ResponseMetadata, ResponseMetadata);
response!(Setmeta, ResponseSetmeta, ResponseSetmeta);
response!(List, ResponseList, ResponseList<N, E>);
response!(Remove, ResponseRemove, ResponseRemove);
response!(Read, ResponseRead, ResponseRead<C>);
response!(Write, ResponseWrite, ResponseWrite);
response!(Seek, ResponseSeek, ResponseSeek);
response!(Tell, ResponseTell, ResponseTell);
response!(Copy, ResponseCopy, ResponseCopy);
response!(Link, ResponseLink, ResponseLink);
response!(Drop, ResponseDrop, ResponseDrop);
