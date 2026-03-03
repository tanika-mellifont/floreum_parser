use crate::{FloreumError, Order, Request, Response, read_content, read_order, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestInsert<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub order: Order,
    pub length: u64,
    pub content: C,
}
impl<C: AsRef<[u8]>> RequestInsert<C> {
    pub const KIND_TAG: u64 = 110;
    pub fn new(descriptor: u64, order: Order, length: u64, content: C) -> Self {
        Self {
            descriptor,
            order,
            length,
            content,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.order.to_iter().into_iter())
            .chain(
                (self.length as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.content.as_ref().iter().cloned()),
            )
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> RequestInsert<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        Ok(Self::new(
            read_u64(bytes)?,
            read_order(bytes)?,
            read_u64(bytes)?,
            read_content(bytes)?.into(),
        ))
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
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        Ok(Self::new(read_u64(bytes)?))
    }
}
#[test]
fn test_request_insert() {
    extern crate alloc;
    use alloc::{vec, vec::Vec};
    let before: RequestInsert<Vec<u8>> =
        RequestInsert::new(12345, Order::After, 5, vec![1, 2, 3, 4, 5]);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestInsert::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_insert() {
    let before = ResponseInsert::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseInsert::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
