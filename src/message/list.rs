use crate::{FloreumError, Order, names::Names, read_order, read_u64};
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
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(&mut bytes)?;
        let cursor = read_order(&mut bytes)?;
        let count = read_u64(&mut bytes)?;
        Ok(Self::new(descriptor, cursor, count))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseList<C: AsRef<[u8]>> {
    pub names: Names<C>,
}
impl<C: AsRef<[u8]>> ResponseList<C> {
    pub const KIND_TAG: u64 = 41;
    pub fn new(names: Names<C>) -> Self {
        Self { names }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.names.bytes().copied()
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> ResponseList<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let names = Names::from_bytes(bytes)?;
        Ok(Self::new(names))
    }
}
#[test]
fn test_request_list() {
    let before = RequestList::new(12345, Order::Before, 67890);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let after = RequestList::from_bytes(&buffer).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_list() {
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
    let mut strings = [0u8; 1024];
    strings[0..8].copy_from_slice(&3u64.to_le_bytes());
    strings[8..16].copy_from_slice(&5u64.to_le_bytes());
    strings[16..21].copy_from_slice("test1".as_bytes());
    strings[21..29].copy_from_slice(&5u64.to_le_bytes());
    strings[29..34].copy_from_slice("test2".as_bytes());
    strings[34..42].copy_from_slice(&5u64.to_le_bytes());
    strings[42..47].copy_from_slice("test3".as_bytes());
    let mut strings_cursor = &strings as &[u8];
    let before = ResponseList::new(Names::<SizedBuffer>::from_bytes(&mut strings_cursor).unwrap());
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseList::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
