#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
use crate::Response;
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
#[cfg(feature = "alloc")]
impl RequestRemove<String> {
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        use crate::NextU64;
        let descriptor = iter.next_u64()?;
        let name_count = iter.next_u64()?.try_into().ok()?;
        let name = String::from_utf8(iter.take(name_count).collect()).ok()?;
        Some(Self::new(descriptor, name))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseRemove {}
impl ResponseRemove {
    pub const KIND_TAG: u64 = 81;
    pub fn new() -> Self {
        Self {}
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        (0..0).into_iter()
    }
    pub fn from_iter(_iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self {})
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Remove(self)
    }
}
#[test]
fn test_request_remove() {
    let before = RequestRemove::new(12345, "test test".to_string());
    let after = RequestRemove::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_remove() {
    let before = ResponseRemove::new();
    let after = ResponseRemove::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
