use crate::{NextU64, Order, Response};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestInsert<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub cursor: Order,
    pub content: C,
}
impl<C: AsRef<[u8]>> RequestInsert<C> {
    pub const KIND_TAG: u64 = 110;
    pub fn new(descriptor: u64, cursor: Order, content: C) -> Self {
        Self {
            descriptor,
            cursor,
            content,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.cursor.to_iter())
            .chain(
                (self.content.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.content.as_ref().iter().copied()),
            )
    }
}
#[cfg(feature = "alloc")]
impl RequestInsert<Vec<u8>> {
    pub fn vec_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        let cursor = Order::from_iter(iter)?;
        let content_count = iter.next_u64()?.try_into().ok()?;
        let content = iter.take(content_count).collect();
        Some(Self::new(descriptor, cursor, content))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseInsert {
    pub count: u64,
}
impl ResponseInsert {
    pub const KIND_TAG: u64 = 111;
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
        Response::Insert(self)
    }
}
#[test]
fn test_request_insert() {
    let before = RequestInsert::new(12345, Order::After, "test test".as_bytes().to_vec());
    let after = RequestInsert::vec_from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_insert() {
    let before = ResponseInsert::new(12345);
    let after = ResponseInsert::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
