use crate::{NextU64, Response};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestDrop {
    pub descriptor: u64,
}
impl RequestDrop {
    pub const KIND_TAG: u64 = 10;
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
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseDrop {}
impl ResponseDrop {
    pub const KIND_TAG: u64 = 11;
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
        Response::Drop(self)
    }
}
#[test]
fn test_request_drop() {
    let before = RequestDrop::new(12345);
    let after = RequestDrop::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_drop() {
    let before = ResponseDrop::new();
    let after = ResponseDrop::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
