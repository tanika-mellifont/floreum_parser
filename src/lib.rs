#![no_std]
mod data;
mod error;
mod message;
mod names;
mod request;
mod response;
pub use data::*;
pub use error::*;
pub use message::*;
pub use request::*;
pub use response::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Message<N: AsRef<str>, C: AsRef<[u8]>> {
    RequestDrop(RequestDrop),
    RequestState(RequestState),
    RequestWalk(RequestWalk<C>),
    RequestList(RequestList),
    RequestJump(RequestJump),
    RequestPoint(RequestPoint),
    RequestMake(RequestMake<N>),
    RequestRemove(RequestRemove<N>),
    RequestRename(RequestRename<N>),
    RequestRead(RequestRead),
    RequestInsert(RequestInsert<C>),
    RequestOverwrite(RequestOverwrite<C>),
    RequestSeek(RequestSeek),
    RequestTell(RequestTell),
    RequestBind(RequestBind<N>),
    ResponseError(ResponseError),
    ResponseDrop(ResponseDrop),
    ResponseState(ResponseState),
    ResponseWalk(ResponseWalk),
    ResponseList(ResponseList<C>),
    ResponseJump(ResponseJump),
    ResponsePoint(ResponsePoint),
    ResponseMake(ResponseMake),
    ResponseRemove(ResponseRemove),
    ResponseRename(ResponseRename),
    ResponseRead(ResponseRead<C>),
    ResponseInsert(ResponseInsert),
    ResponseOverwrite(ResponseOverwrite),
    ResponseSeek(ResponseSeek),
    ResponseTell(ResponseTell),
    ResponseBind(ResponseBind),
}
