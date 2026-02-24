use crate::{Response, State};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestRename<N: AsRef<str>> {
    pub descriptor: u64,
    pub state: State,
    pub from: N,
    pub to: N,
}
impl<N: AsRef<str>> RequestRename<N> {
    pub const KIND_TAG: u64 = 90;
    pub fn new(descriptor: u64, state: State, from: N, to: N) -> Self {
        Self {
            descriptor,
            state,
            from,
            to,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.state.to_iter())
            .chain(
                (self.from.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.from.as_ref().as_bytes().iter().copied()),
            )
            .chain(
                (self.to.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.to.as_ref().as_bytes().iter().copied()),
            )
    }
}
#[cfg(feature = "alloc")]
impl RequestRename<String> {
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        use crate::NextU64;
        let descriptor = iter.next_u64()?;
        let state = State::from_iter(iter)?;
        let from_count = iter.next_u64()?.try_into().ok()?;
        let from = String::from_utf8(iter.take(from_count).collect()).ok()?;
        let to_count = iter.next_u64()?.try_into().ok()?;
        let to = String::from_utf8(iter.take(to_count).collect()).ok()?;
        Some(Self::new(descriptor, state, from, to))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseRename {}
impl ResponseRename {
    pub const KIND_TAG: u64 = 91;
    pub fn new() -> Self {
        Self {}
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        (0..0).into_iter()
    }
    pub fn from_iter(_iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self {})
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Rename(self)
    }
}
#[test]
fn test_request_rename() {
    let before = RequestRename::new(
        12345,
        State::default(),
        "test1".to_string(),
        "test2".to_string(),
    );
    let after = RequestRename::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_rename() {
    let before = ResponseRename::new();
    let after = ResponseRename::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
