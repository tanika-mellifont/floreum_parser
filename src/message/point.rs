use crate::{NextU64, Response};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestPoint {
    pub descriptor: u64,
}
impl RequestPoint {
    pub const KIND_TAG: u64 = 60;
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
pub struct ResponsePoint {
    pub offset: u64,
}
impl ResponsePoint {
    pub const KIND_TAG: u64 = 61;
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
        Response::Point(self)
    }
}
#[test]
fn test_request_point() {
    let before = RequestPoint::new(12345);
    let after = RequestPoint::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_point() {
    let before = ResponsePoint::new(12345);
    let after = ResponsePoint::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
