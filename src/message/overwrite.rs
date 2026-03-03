use crate::{Direction, FloreumError, Order, read_content, read_direction, read_order, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestOverwrite<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub order: Order,
    pub direction: Direction,
    pub content: C,
}
impl<C: AsRef<[u8]>> RequestOverwrite<C> {
    pub const KIND_TAG: u64 = 120;
    pub fn new(descriptor: u64, order: Order, direction: Direction, content: C) -> Self {
        Self {
            descriptor,
            order,
            direction,
            content,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.order.to_iter())
            .chain(self.direction.to_iter())
            .chain(
                (self.content.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.content.as_ref().iter().copied()),
            )
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> RequestOverwrite<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let order = read_order(bytes)?;
        let direction = read_direction(bytes)?;
        let content = read_content(bytes)?.into();
        Ok(Self::new(descriptor, order, direction, content))
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
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let count = read_u64(bytes)?;
        Ok(Self::new(count))
    }
}
#[test]
fn test_request_overwrite() {
    #[derive(PartialEq)]
    pub struct SizedBuffer([u8; 1024]);
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
    let mut test_array = [0; 1024];
    test_array.copy_from_slice(b"test test test");
    let before = RequestOverwrite::new(
        12345,
        Order::After,
        Direction::Forward,
        SizedBuffer(test_array),
    );
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestOverwrite::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_overwrite() {
    let before = ResponseOverwrite::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseOverwrite::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
