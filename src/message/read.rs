use crate::{NextU64, Order, Response};
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestRead {
    pub descriptor: u64,
    pub cursor: Order,
    pub count: u64,
}
impl RequestRead {
    pub const KIND_TAG: u64 = 100;
    pub fn new(descriptor: u64, cursor: Order, count: u64) -> Self {
        Self {
            descriptor,
            cursor,
            count,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.cursor.to_iter())
            .chain(self.count.to_le_bytes().into_iter())
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self::new(
            iter.next_u64()?,
            Order::from_iter(iter)?,
            iter.next_u64()?,
        ))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseRead<C: AsRef<[u8]>> {
    pub content: C,
}
impl<C: AsRef<[u8]>> ResponseRead<C> {
    pub const KIND_TAG: u64 = 101;
    pub fn new(content: C) -> Self {
        Self { content }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.content
            .as_ref()
            .len()
            .to_le_bytes()
            .into_iter()
            .chain(self.content.as_ref().iter().copied())
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>>(self) -> Response<N, P, C> {
        Response::Read(self)
    }
}
#[cfg(feature = "alloc")]
impl ResponseRead<Vec<u8>> {
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let content_count = iter.next_u64()?.try_into().ok()?;
        let content = iter.take(content_count).collect();
        Some(Self { content })
    }
}
#[test]
fn test_request_read() {
    let before = RequestRead::new(12345, Order::Before, 67890);
    let after = RequestRead::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_read() {
    let before = ResponseRead::new("test test".as_bytes().to_vec());
    let after = ResponseRead::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
