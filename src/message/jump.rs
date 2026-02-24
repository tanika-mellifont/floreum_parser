use crate::{Cursor, NextU64, Response};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestJump {
    pub descriptor: u64,
    pub cursor: Cursor,
    pub offset: u64,
}
impl RequestJump {
    pub const KIND_TAG: u64 = 50;
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
        let descriptor = iter.next_u64()?;
        let cursor = Cursor::from_iter(iter)?;
        let offset = iter.next_u64()?;
        Some(Self::new(descriptor, cursor, offset))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseJump {}
impl ResponseJump {
    pub const KIND_TAG: u64 = 51;
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
        Response::Jump(self)
    }
}
#[test]
fn test_request_jump() {
    let before = RequestJump::new(12345, Cursor::End, 67890);
    let after = RequestJump::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_jump() {
    let before = ResponseJump::new();
    let after = ResponseJump::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
