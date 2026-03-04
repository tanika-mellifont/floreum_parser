use crate::{FloreumError, Order, read_content, read_order, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestRead {
    pub descriptor: u64,
    pub order: Order,
    pub count: u64,
}
impl RequestRead {
    pub const KIND_TAG: u64 = 100;
    pub fn new(descriptor: u64, order: Order, count: u64) -> Self {
        Self {
            descriptor,
            order,
            count,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.order.to_iter())
            .chain(self.count.to_le_bytes().into_iter())
    }
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let order = read_order(bytes)?;
        let count = read_u64(bytes)?;
        Ok(Self::new(descriptor, order, count))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseRead<C: AsRef<[u8]>> {
    pub content: C,
}
impl<C: AsRef<[u8]>> ResponseRead<C> {
    pub const KIND_TAG: u64 = 101;
    pub fn new(content: C) -> Self {
        Self { content }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        (self.content.as_ref().len() as u64)
            .to_le_bytes()
            .into_iter()
            .chain(self.content.as_ref().iter().copied())
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> ResponseRead<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let content = read_content(bytes)?.into();
        Ok(Self::new(content))
    }
}
#[test]
fn test_request_read() {
    let before = RequestRead::new(12345, Order::Before, 67890);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestRead::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_read() {
    #[derive(PartialEq)]
    pub struct SizedBuffer([u8; 128]);
    impl AsRef<[u8]> for SizedBuffer {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }
    impl<'a> From<&'a [u8]> for SizedBuffer {
        fn from(value: &'a [u8]) -> Self {
            Self(value.as_array().unwrap().clone())
        }
    }
    let mut test_array = [0; 128];
    let test_bytes = b"test test test";
    test_array[..test_bytes.len()].copy_from_slice(test_bytes);
    let before = ResponseRead::new(SizedBuffer(test_array));
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseRead::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
