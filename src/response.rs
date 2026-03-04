use crate::{
    ResponseBind, ResponseDrop, ResponseError, ResponseInsert, ResponseJump, ResponseList,
    ResponseMake, ResponseOverwrite, ResponsePoint, ResponseRead, ResponseRemove, ResponseRename,
    ResponseSeek, ResponseState, ResponseTell, ResponseTruncate, ResponseWalk,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Response<C: AsRef<[u8]>> {
    Error(ResponseError),
    Drop(ResponseDrop),
    State(ResponseState),
    Walk(ResponseWalk),
    List(ResponseList<C>),
    Jump(ResponseJump),
    Point(ResponsePoint),
    Make(ResponseMake),
    Remove(ResponseRemove),
    Rename(ResponseRename),
    Read(ResponseRead<C>),
    Insert(ResponseInsert),
    Overwrite(ResponseOverwrite),
    Truncate(ResponseTruncate),
    Seek(ResponseSeek),
    Tell(ResponseTell),
    Bind(ResponseBind),
}
impl<C: AsRef<[u8]>> From<ResponseDrop> for Response<C> {
    fn from(value: ResponseDrop) -> Self {
        Self::Drop(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseState> for Response<C> {
    fn from(value: ResponseState) -> Self {
        Self::State(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseWalk> for Response<C> {
    fn from(value: ResponseWalk) -> Self {
        Self::Walk(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseList<C>> for Response<C> {
    fn from(value: ResponseList<C>) -> Self {
        Self::List(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseJump> for Response<C> {
    fn from(value: ResponseJump) -> Self {
        Self::Jump(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponsePoint> for Response<C> {
    fn from(value: ResponsePoint) -> Self {
        Self::Point(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseMake> for Response<C> {
    fn from(value: ResponseMake) -> Self {
        Self::Make(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseRemove> for Response<C> {
    fn from(value: ResponseRemove) -> Self {
        Self::Remove(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseRename> for Response<C> {
    fn from(value: ResponseRename) -> Self {
        Self::Rename(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseRead<C>> for Response<C> {
    fn from(value: ResponseRead<C>) -> Self {
        Self::Read(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseInsert> for Response<C> {
    fn from(value: ResponseInsert) -> Self {
        Self::Insert(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseOverwrite> for Response<C> {
    fn from(value: ResponseOverwrite) -> Self {
        Self::Overwrite(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseTruncate> for Response<C> {
    fn from(value: ResponseTruncate) -> Self {
        Response::Truncate(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseSeek> for Response<C> {
    fn from(value: ResponseSeek) -> Self {
        Self::Seek(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseTell> for Response<C> {
    fn from(value: ResponseTell) -> Self {
        Self::Tell(value)
    }
}
impl<C: AsRef<[u8]>> From<ResponseBind> for Response<C> {
    fn from(value: ResponseBind) -> Self {
        Self::Bind(value)
    }
}
