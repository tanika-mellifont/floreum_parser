use crate::{Direction, NextU64, Order, Response};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestOverwrite<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub cursor: Order,
    pub location: Direction,
    pub content: C,
}
impl<C: AsRef<[u8]>> RequestOverwrite<C> {
    pub const KIND_TAG: u64 = 120;
    pub fn new(descriptor: u64, cursor: Order, location: Direction, content: C) -> Self {
        Self {
            descriptor,
            cursor,
            location,
            content,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.cursor.to_iter())
            .chain(self.location.to_iter())
            .chain(
                (self.content.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.content.as_ref().iter().copied()),
            )
    }
}
#[cfg(feature = "alloc")]
impl RequestOverwrite<Vec<u8>> {
    pub fn vec_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        let cursor = Order::from_iter(iter)?;
        let location = Direction::from_iter(iter)?;
        let content_count = iter.next_u64()?.try_into().ok()?;
        let content = iter.take(content_count).collect();
        Some(Self::new(descriptor, cursor, location, content))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseOverwrite {
    pub count: u64,
}
impl ResponseOverwrite {
    pub const KIND_TAG: u64 = 121;
    pub fn new(count: u64) -> Self {
        Self { count }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.count.to_le_bytes().into_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let count = iter.next_u64()?;
        Some(Self { count })
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Overwrite(self)
    }
}
#[test]
fn test_request_overwrite() {
    let before = RequestOverwrite::new(
        12345,
        Order::After,
        Direction::Forward,
        "test test".as_bytes().to_vec(),
    );
    let after = RequestOverwrite::vec_from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_overwrite() {
    let before = ResponseOverwrite::new(12345);
    let after = ResponseOverwrite::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
