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
    #[cfg(feature = "bind")]
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
    #[cfg(feature = "bind")]
    ResponseBind(ResponseBind),
}
impl<N: AsRef<str>, C: AsRef<[u8]>> Message<N, C> {
    pub fn kind_tag(&self) -> u64 {
        match self {
            Message::RequestDrop(_) => RequestDrop::KIND_TAG,
            Message::RequestState(_) => RequestState::KIND_TAG,
            Message::RequestWalk(_) => RequestWalk::<C>::KIND_TAG,
            Message::RequestList(_) => RequestList::KIND_TAG,
            Message::RequestJump(_) => RequestJump::KIND_TAG,
            Message::RequestPoint(_) => RequestPoint::KIND_TAG,
            Message::RequestMake(_) => RequestMake::<N>::KIND_TAG,
            Message::RequestRemove(_) => RequestRemove::<N>::KIND_TAG,
            Message::RequestRename(_) => RequestRename::<N>::KIND_TAG,
            Message::RequestRead(_) => RequestRead::KIND_TAG,
            Message::RequestInsert(_) => RequestInsert::<C>::KIND_TAG,
            Message::RequestOverwrite(_) => RequestOverwrite::<C>::KIND_TAG,
            Message::RequestSeek(_) => RequestSeek::KIND_TAG,
            Message::RequestTell(_) => RequestTell::KIND_TAG,
            #[cfg(feature = "bind")]
            Message::RequestBind(_) => RequestBind::<N>::KIND_TAG,
            Message::ResponseError(_) => ResponseError::KIND_TAG,
            Message::ResponseDrop(_) => ResponseDrop::KIND_TAG,
            Message::ResponseState(_) => ResponseState::KIND_TAG,
            Message::ResponseWalk(_) => ResponseWalk::KIND_TAG,
            Message::ResponseList(_) => ResponseList::<C>::KIND_TAG,
            Message::ResponseJump(_) => ResponseJump::KIND_TAG,
            Message::ResponsePoint(_) => ResponsePoint::KIND_TAG,
            Message::ResponseMake(_) => ResponseMake::KIND_TAG,
            Message::ResponseRemove(_) => ResponseRemove::KIND_TAG,
            Message::ResponseRename(_) => ResponseRename::KIND_TAG,
            Message::ResponseRead(_) => ResponseRead::<C>::KIND_TAG,
            Message::ResponseInsert(_) => ResponseInsert::KIND_TAG,
            Message::ResponseOverwrite(_) => ResponseOverwrite::KIND_TAG,
            Message::ResponseSeek(_) => ResponseSeek::KIND_TAG,
            Message::ResponseTell(_) => ResponseTell::KIND_TAG,
            #[cfg(feature = "bind")]
            Message::ResponseBind(_) => ResponseBind::KIND_TAG,
        }
    }
}
impl<N: AsRef<str> + for<'a> From<&'a str>, C: AsRef<[u8]> + for<'a> From<&'a [u8]>> Message<N, C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        Ok(match read_u64(bytes)? {
            RequestDrop::KIND_TAG => Self::RequestDrop(RequestDrop::from_bytes(bytes)?),
            RequestState::KIND_TAG => Self::RequestState(RequestState::from_bytes(bytes)?),
            RequestWalk::<C>::KIND_TAG => Self::RequestWalk(RequestWalk::from_bytes(bytes)?),
            RequestList::KIND_TAG => Self::RequestList(RequestList::from_bytes(bytes)?),
            RequestJump::KIND_TAG => Self::RequestJump(RequestJump::from_bytes(bytes)?),
            RequestPoint::KIND_TAG => Self::RequestPoint(RequestPoint::from_bytes(bytes)?),
            RequestMake::<N>::KIND_TAG => Self::RequestMake(RequestMake::from_bytes(bytes)?),
            RequestRemove::<N>::KIND_TAG => Self::RequestRemove(RequestRemove::from_bytes(bytes)?),
            RequestRename::<N>::KIND_TAG => Self::RequestRename(RequestRename::from_bytes(bytes)?),
            RequestRead::KIND_TAG => Self::RequestRead(RequestRead::from_bytes(bytes)?),
            RequestInsert::<C>::KIND_TAG => Self::RequestInsert(RequestInsert::from_bytes(bytes)?),
            RequestOverwrite::<C>::KIND_TAG => Self::RequestOverwrite(RequestOverwrite::from_bytes(bytes)?),
            RequestSeek::KIND_TAG => Self::RequestSeek(RequestSeek::from_bytes(bytes)?),
            RequestTell::KIND_TAG => Self::RequestTell(RequestTell::from_bytes(bytes)?),
            #[cfg(feature = "bind")]
            RequestBind::<N>::KIND_TAG => Self::RequestBind(RequestBind::from_bytes(bytes)?),
            ResponseError::KIND_TAG => Self::ResponseError(ResponseError::new()),
            ResponseDrop::KIND_TAG => Self::ResponseDrop(ResponseDrop::new()),
            ResponseState::KIND_TAG => Self::ResponseState(ResponseState::from_bytes(bytes)?),
            ResponseWalk::KIND_TAG => Self::ResponseWalk(ResponseWalk::from_bytes(bytes)?),
            ResponseList::<C>::KIND_TAG => Self::ResponseList(ResponseList::from_bytes(bytes)?),
            ResponseJump::KIND_TAG => Self::ResponseJump(ResponseJump::new()),
            ResponsePoint::KIND_TAG => Self::ResponsePoint(ResponsePoint::new()),
            ResponseMake::KIND_TAG => Self::ResponseMake(ResponseMake::from_bytes(bytes)?),
            ResponseRemove::KIND_TAG => Self::ResponseRemove(ResponseRemove::new()),
            ResponseRename::KIND_TAG => Self::ResponseRename(ResponseRename::new()),
            ResponseRead::<C>::KIND_TAG => Self::ResponseRead(ResponseRead::from_bytes(bytes)?),
            ResponseInsert::KIND_TAG => Self::ResponseInsert(ResponseInsert::from_bytes(bytes)?),
            ResponseOverwrite::KIND_TAG => Self::ResponseOverwrite(ResponseOverwrite::from_bytes(bytes)?),
            ResponseSeek::KIND_TAG => Self::ResponseSeek(ResponseSeek::new()),
            ResponseTell::KIND_TAG => Self::ResponseTell(ResponseTell::from_bytes(bytes)?),
            #[cfg(feature = "bind")]
            ResponseBind::KIND_TAG => Self::ResponseBind(ResponseBind::new()),
            unknown => return Err(FloreumError::UnknownKind { kind: unknown }),
        })
    }
}