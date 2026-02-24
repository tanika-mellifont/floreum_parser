use crate::{NextU64, Response, State};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestState {
    pub descriptor: u64,
}
impl RequestState {
    pub const KIND_TAG: u64 = 20;
    pub fn new(descriptor: u64) -> Self {
        Self { descriptor }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(Self::new(iter.next_u64()?))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseState {
    pub state: State,
}
impl ResponseState {
    pub const KIND_TAG: u64 = 21;
    pub fn new(state: State) -> Self {
        Self { state }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.state.to_iter()
    }
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let state = State::from_iter(iter)?;
        Some(Self { state })
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::State(self)
    }
}
#[test]
fn test_request_state() {
    let before = RequestState::new(12345);
    let after = RequestState::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_state() {
    let before = ResponseState::new(State::default());
    let after = ResponseState::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
