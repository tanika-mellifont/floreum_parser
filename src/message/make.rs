use crate::{NextU64, Response, State};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestMake<N: AsRef<str>> {
    pub descriptor: u64,
    pub state: State,
    pub name: N,
}
impl<N: AsRef<str>> RequestMake<N> {
    pub const KIND_TAG: u64 = 70;
    pub fn new(descriptor: u64, state: State, name: N) -> Self {
        Self {
            descriptor,
            state,
            name,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.state.to_iter())
            .chain(
                (self.name.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.name.as_ref().as_bytes().iter().copied()),
            )
    }
}
#[cfg(feature = "alloc")]
impl RequestMake<String> {
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        let state = State::from_iter(iter)?;
        let name_count = iter.next_u64()?.try_into().ok()?;
        let name = String::from_utf8(iter.take(name_count).collect()).ok()?;
        Some(Self::new(descriptor, state, name))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseMake {
    pub descriptor: u64,
}
impl ResponseMake {
    pub const KIND_TAG: u64 = 71;
    pub fn new(descriptor: u64) -> Self {
        Self { descriptor }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        Some(Self::new(descriptor))
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Make(self)
    }
}
#[test]
fn test_request_make() {
    let before = RequestMake::new(12345, State::default(), "test".to_string());
    let after = RequestMake::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_make() {
    let before = ResponseMake::new(12345);
    let after = ResponseMake::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
