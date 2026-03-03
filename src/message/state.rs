use crate::{FloreumError, Request, State, read_state, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestState {
    pub descriptor: u64,
}
impl RequestState {
    pub const KIND_TAG: u64 = 50;
    pub fn new(descriptor: u64) -> Self {
        Self { descriptor }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor.to_le_bytes().into_iter()
    }
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        Ok(Self::new(descriptor))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseState {
    state: State,
}
impl ResponseState {
    pub const KIND_TAG: u64 = 51;
    pub fn new(state: State) -> Self {
        Self { state }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.state.to_iter()
    }
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let state = read_state(bytes)?;
        Ok(Self::new(state))
    }
}
#[test]
fn test_request_state() {
    let before = RequestState::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestState::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_state() {
    let before = ResponseState::new(State::default());
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseState::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
