use crate::{FloreumError, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestDrop {
    pub descriptor: u64,
}
impl RequestDrop {
    pub const KIND_TAG: u64 = 10;
    pub fn new(descriptor: u64) -> Self {
        Self { descriptor }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter()
    }
    pub fn from_bytes<'a>(bytes: &mut &'a [u8]) -> Result<Self, FloreumError> {
        Ok(Self::new(read_u64(bytes)?))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseDrop {}
impl ResponseDrop {
    pub const KIND_TAG: u64 = 11;
    pub fn new() -> Self {
        Self {}
    }
}
#[test]
fn test_request_drop() {
    let before = RequestDrop::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestDrop::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
