use crate::{Cursor, NextU64, Response};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestSeek {
    pub descriptor: u64,
    pub cursor: Cursor,
    pub offset: u64,
}
impl RequestSeek {
    pub const KIND_TAG: u64 = 130;
    pub fn new(descriptor: u64, cursor: Cursor, offset: u64) -> Self {
        Self {
            descriptor,
            cursor,
            offset,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.cursor.to_iter())
            .chain(self.offset.to_le_bytes().into_iter())
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self::new(
            iter.next_u64()?,
            Cursor::from_iter(iter)?,
            iter.next_u64()?,
        ))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseSeek {}
impl ResponseSeek {
    pub const KIND_TAG: u64 = 131;
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
        Response::Seek(self)
    }
}
#[test]
fn test_request_seek() {
    let before = RequestSeek::new(12345, Cursor::End, 67890);
    let after = RequestSeek::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_seek() {
    let before = ResponseSeek::new();
    let after = ResponseSeek::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
