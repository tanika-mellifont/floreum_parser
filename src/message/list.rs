use crate::{FloreumError, names::Names, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestList {
    pub descriptor: u64,
    pub count: u64,
}
impl RequestList {
    pub const KIND_TAG: u64 = 40;
    pub fn new(descriptor: u64, count: u64) -> Self {
        Self {
            descriptor,
            count,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.count.to_le_bytes().into_iter())
    }
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(&mut bytes)?;
        let count = read_u64(&mut bytes)?;
        Ok(Self::new(descriptor, count))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    let before = RequestList::new(12345, 67890);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let after = RequestList::from_bytes(&buffer).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_list() {
    const DATA_SIZE: usize = 8 + 8 + 14;
    #[derive(Debug, PartialEq)]
    struct ExactBuffer([u8; DATA_SIZE]);
    impl AsRef<[u8]> for ExactBuffer {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }
    impl<'a> From<&'a [u8]> for ExactBuffer {
        fn from(value: &'a [u8]) -> Self {
            let mut arr = [0; DATA_SIZE];
            arr.copy_from_slice(value);
            ExactBuffer(arr)
        }
    }
    let mut names_bytes = [0u8; DATA_SIZE];
    let mut pos = 0;
    names_bytes[pos..pos + 8].copy_from_slice(&1u64.to_le_bytes());
    pos += 8;
    names_bytes[pos..pos + 8].copy_from_slice(&14u64.to_le_bytes());
    pos += 8;
    names_bytes[pos..pos + 14].copy_from_slice(b"test test test");
    let mut cursor = &names_bytes[..];
    let names = Names::<ExactBuffer>::from_bytes(&mut cursor).unwrap();
    let before = ResponseList::new(names);
    let mut serialized = [0u8; DATA_SIZE];
    for (dst, src) in serialized.iter_mut().zip(before.to_iter()) {
        *dst = src;
    }
    let mut cursor = &serialized[..];
    let after = ResponseList::from_bytes(&mut cursor).unwrap();
    assert_eq!(before, after);
}