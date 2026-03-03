use crate::{FloreumError, Head, read_head, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestTell {
    pub descriptor: u64,
    pub head: Head,
    pub offset: u64,
}
impl RequestTell {
    pub const KIND_TAG: u64 = 50;
    pub fn new(descriptor: u64, head: Head, offset: u64) -> Self {
        Self {
            descriptor,
            head,
            offset,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.head.to_iter())
            .chain(self.offset.to_le_bytes().into_iter())
    }
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let head = read_head(bytes)?;
        let offset = read_u64(bytes)?;
        Ok(Self::new(descriptor, head, offset))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseTell {
    offset: u64,
}
impl ResponseTell {
    pub const KIND_TAG: u64 = 51;
    pub fn new(offset: u64) -> Self {
        Self { offset }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.offset.to_le_bytes().into_iter()
    }
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let offset = read_u64(bytes)?;
        Ok(Self::new(offset))
    }
}
#[test]
fn test_request_tell() {
    let before = RequestTell::new(12345, Head::End, 67890);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestTell::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_tell() {
    let before = ResponseTell::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseTell::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
