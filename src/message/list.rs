use crate::{NextU64, Order, Response};
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestList {
    pub descriptor: u64,
    pub cursor: Order,
    pub count: u64,
}
impl RequestList {
    pub const KIND_TAG: u64 = 30;
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
        let descriptor = iter.next_u64()?;
        let cursor = Order::from_iter(iter)?;
        let count = iter.next_u64()?;
        Some(Self::new(descriptor, cursor, count))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseList<N: AsRef<str>, P: AsRef<[N]>> {
    pub names: P,
    _phantom_n: PhantomData<N>,
}
impl<N: AsRef<str>, P: AsRef<[N]>> ResponseList<N, P> {
    pub const KIND_TAG: u64 = 41;
    pub fn new(names: P) -> Self {
        Self {
            names,
            _phantom_n: PhantomData,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        let count = self.names.as_ref().len() as u64;
        count.to_le_bytes().into_iter().chain(
            self.names
                .as_ref()
                .iter()
                .map(|name| {
                    (name.as_ref().len() as u64)
                        .to_le_bytes()
                        .into_iter()
                        .chain(name.as_ref().as_bytes().into_iter().copied())
                })
                .flatten(),
        )
    }
    pub fn into_response<C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::List(self)
    }
}
#[cfg(feature = "alloc")]
impl ResponseList<String, Vec<String>> {
    pub fn string_vec_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let mut names = Vec::with_capacity(iter.next_u64()?.try_into().ok()?);
        for _ in 0..names.capacity() {
            let current_count = iter.next_u64()?.try_into().ok()?;
            names.push(String::from_utf8(iter.take(current_count).collect()).ok()?);
        }
        Some(Self::new(names))
    }
}
#[test]
fn test_request_list() {
    let before = RequestList::new(12345, Order::Before, 67890);
    let after = RequestList::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_list() {
    let before = ResponseList::new(
        [
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]
        .to_vec(),
    );
    let after = ResponseList::string_vec_from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
