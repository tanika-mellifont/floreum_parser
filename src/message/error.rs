use crate::Response;
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseError {}
impl ResponseError {
    pub const KIND_TAG: u64 = 1;
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
        Response::Error(self)
    }
}
#[test]
fn test_response_error() {
    let before = ResponseError::new();
    let after = ResponseError::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
