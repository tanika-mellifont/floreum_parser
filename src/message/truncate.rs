use crate::{Direction, FloreumError, read_direction, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestTruncate {
    pub descriptor: u64,
    pub direction: Direction,
}
impl RequestTruncate {
    pub const KIND_TAG: u64 = 130;
    pub fn new(descriptor: u64, direction: Direction) -> Self {
        Self {
            descriptor,
            direction,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.direction.to_iter())
    }
}
impl RequestTruncate {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let direction = read_direction(bytes)?;
        Ok(Self::new(descriptor, direction))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseTruncate {
    pub count: u64,
}
impl ResponseTruncate {
    pub const KIND_TAG: u64 = 131;
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
fn test_request_truncate() {
    let before = RequestTruncate::new(12345, Direction::Forward);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestTruncate::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
