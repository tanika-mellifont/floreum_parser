use crate::{FloreumError, Order, read_content, read_order, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestInsert<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub order: Order,
    pub content: C,
}
impl<C: AsRef<[u8]>> RequestInsert<C> {
    pub const KIND_TAG: u64 = 110;
    pub fn new(descriptor: u64, order: Order, content: C) -> Self {
        Self {
            descriptor,
            order,
            content,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.order.to_iter().into_iter())
            .chain(
                (self.content.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.content.as_ref().iter().cloned()),
            )
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> RequestInsert<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let order = read_order(bytes)?;
        let content = read_content(bytes)?.into();
        Ok(Self::new(descriptor, order, content))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    #[derive(PartialEq)]
    pub struct SizedBuffer([u8; 64]);
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
    let mut test_array = [0; 64];
    let test_bytes = b"test test test";
    test_array[..test_bytes.len()].copy_from_slice(test_bytes);
    let before = RequestInsert::new(12345, Order::Before, SizedBuffer(test_array));
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
