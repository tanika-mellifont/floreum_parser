use crate::{NextU64, Response};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestTell {
    pub descriptor: u64,
}
impl RequestTell {
    pub const KIND_TAG: u64 = 140;
    pub fn new(descriptor: u64) -> Self {
        Self { descriptor }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self::new(iter.next_u64()?))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseTell {
    pub offset: u64,
}
impl ResponseTell {
    pub const KIND_TAG: u64 = 141;
    pub fn new(offset: u64) -> Self {
        Self { offset }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.offset.to_le_bytes().into_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let offset = iter.next_u64()?;
        Some(Self { offset })
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Tell(self)
    }
}
#[test]
fn test_request_tell() {
    let before = RequestTell::new(12345);
    let after = RequestTell::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_tell() {
    let before = ResponseTell::new(12345);
    let after = ResponseTell::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
