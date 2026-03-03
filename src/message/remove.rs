use crate::{FloreumError, read_str, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestRemove<N: AsRef<str>> {
    pub descriptor: u64,
    pub name: N,
}
impl<N: AsRef<str>> RequestRemove<N> {
    pub const KIND_TAG: u64 = 80;
    pub fn new(descriptor: u64, name: N) -> Self {
        Self { descriptor, name }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter().chain(
            (self.name.as_ref().len() as u64)
                .to_le_bytes()
                .into_iter()
                .chain(self.name.as_ref().as_bytes().iter().copied()),
        )
    }
}
impl<N: AsRef<str> + for<'a> From<&'a str>> RequestRemove<N> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let name = read_str(bytes)?.into();
        Ok(Self::new(descriptor, name))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseRemove {}
impl ResponseRemove {
    pub const KIND_TAG: u64 = 81;
    pub fn new() -> Self {
        Self {}
    }
}
#[test]
fn test_request_remove() {
    #[derive(PartialEq)]
    pub struct SizedString([u8; 1024]);
    impl AsRef<str> for SizedString {
        fn as_ref(&self) -> &str {
            str::from_utf8(&self.0).unwrap()
        }
    }
    impl<'a> From<&'a str> for SizedString {
        fn from(value: &'a str) -> Self {
            Self(value.as_bytes().as_array().unwrap().clone())
        }
    }
    let mut test_array = [0; 1024];
    test_array.copy_from_slice(b"test test test");
    let before = RequestRemove::new(12345, SizedString(test_array));
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestRemove::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
