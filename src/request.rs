use crate::{
    RequestBind, RequestDrop, RequestInsert, RequestJump, RequestList, RequestMake,
    RequestOverwrite, RequestPoint, RequestRead, RequestRemove, RequestRename, RequestSeek,
    RequestState, RequestTell, RequestTruncate, RequestWalk,
};
pub enum Request<N: AsRef<str>, C: AsRef<[u8]>> {
    Drop(RequestDrop),
    State(RequestState),
    Walk(RequestWalk<C>),
    List(RequestList),
    Jump(RequestJump),
    Point(RequestPoint),
    Make(RequestMake<N>),
    Remove(RequestRemove<N>),
    Rename(RequestRename<N>),
    Read(RequestRead),
    Insert(RequestInsert<C>),
    Overwrite(RequestOverwrite<C>),
    Truncate(RequestTruncate),
    Seek(RequestSeek),
    Tell(RequestTell),
    Bind(RequestBind<N>),
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestDrop> for Request<N, C> {
    fn from(value: RequestDrop) -> Self {
        Request::Drop(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestState> for Request<N, C> {
    fn from(value: RequestState) -> Self {
        Self::State(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestWalk<C>> for Request<N, C> {
    fn from(value: RequestWalk<C>) -> Self {
        Self::Walk(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestList> for Request<N, C> {
    fn from(value: RequestList) -> Self {
        Self::List(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestJump> for Request<N, C> {
    fn from(value: RequestJump) -> Self {
        Self::Jump(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestPoint> for Request<N, C> {
    fn from(value: RequestPoint) -> Self {
        Self::Point(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestMake<N>> for Request<N, C> {
    fn from(value: RequestMake<N>) -> Self {
        Self::Make(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestRemove<N>> for Request<N, C> {
    fn from(value: RequestRemove<N>) -> Self {
        Self::Remove(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestRename<N>> for Request<N, C> {
    fn from(value: RequestRename<N>) -> Self {
        Self::Rename(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestRead> for Request<N, C> {
    fn from(value: RequestRead) -> Self {
        Self::Read(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestInsert<C>> for Request<N, C> {
    fn from(value: RequestInsert<C>) -> Self {
        Self::Insert(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestOverwrite<C>> for Request<N, C> {
    fn from(value: RequestOverwrite<C>) -> Self {
        Self::Overwrite(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestTruncate> for Request<N, C> {
    fn from(value: RequestTruncate) -> Self {
        Self::Truncate(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestSeek> for Request<N, C> {
    fn from(value: RequestSeek) -> Self {
        Self::Seek(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestTell> for Request<N, C> {
    fn from(value: RequestTell) -> Self {
        Self::Tell(value)
    }
}
impl<N: AsRef<str>, C: AsRef<[u8]>> From<RequestBind<N>> for Request<N, C> {
    fn from(value: RequestBind<N>) -> Self {
        Self::Bind(value)
    }
}
