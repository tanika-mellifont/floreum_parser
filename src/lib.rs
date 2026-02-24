#![no_std]
#![feature(iter_array_chunks)]
mod message;
mod options;
mod state;
#[cfg(any(feature = "alloc", test))]
extern crate alloc;
pub use message::*;
pub use options::*;
pub use state::*;
pub(crate) trait NextU64 {
    fn next_u64(&mut self) -> Option<u64>;
}
impl<T: Iterator<Item = u8>> NextU64 for T {
    fn next_u64(&mut self) -> Option<u64> {
        Some(u64::from_le_bytes(self.array_chunks().next()?))
    }
}
pub enum Message<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>> {
    RequestDrop(RequestDrop),
    RequestState(RequestState),
    RequestWalk(RequestWalk<N, P>),
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
    ResponseList(ResponseList<N, P>),
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
impl<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>> Message<N, P, C> {
    pub fn string_vec_vec_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let kind = iter.next_u64()?;
        todo!()
    }
}
pub enum Request<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>> {
    Drop(RequestDrop),
    State(RequestState),
    Walk(RequestWalk<N, P>),
    List(RequestList),
    Jump(RequestJump),
    Point(RequestPoint),
    Make(RequestMake<N>),
    Remove(RequestRemove<N>),
    Rename(RequestRename<N>),
    Read(RequestRead),
    Insert(RequestInsert<C>),
    Overwrite(RequestOverwrite<C>),
    Seek(RequestSeek),
    Tell(RequestTell),
    Bind(RequestBind<N>),
}
pub enum Response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>> {
    Error(ResponseError),
    Drop(ResponseDrop),
    State(ResponseState),
    Walk(ResponseWalk),
    List(ResponseList<N, P>),
    Jump(ResponseJump),
    Point(ResponsePoint),
    Make(ResponseMake),
    Remove(ResponseRemove),
    Rename(ResponseRename),
    Read(ResponseRead<C>),
    Insert(ResponseInsert),
    Overwrite(ResponseOverwrite),
    Seek(ResponseSeek),
    Tell(ResponseTell),
    Bind(ResponseBind),
}
