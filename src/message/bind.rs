use crate::{Order, Request, Response, State};
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestBind<N: AsRef<str>> {
    pub descriptor: u64,
    pub new: u64,
    pub order: Order,
    pub state: State,
    pub name: N,
}
impl<N: AsRef<str>> RequestBind<N> {
    pub const KIND_TAG: u64 = 150;
    pub fn new(descriptor: u64, new: u64, order: Order, state: State, name: N) -> Self {
        Self {
            descriptor,
            new,
            order,
            state,
            name,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.new.to_le_bytes().into_iter())
            .chain(self.order.to_iter())
            .chain(self.state.to_iter())
            .chain(
                (self.name.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.name.as_ref().as_bytes().iter().copied()),
            )
    }
    pub fn to_request<P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Request<N, P, C> {
        Request::Bind(self)
    }
}
#[cfg(feature = "alloc")]
impl RequestBind<String> {
    pub fn string_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        use crate::NextU64;
        let descriptor = iter.next_u64()?;
        let new = iter.next_u64()?;
        let order = Order::from_iter(iter)?;
        let state = State::from_iter(iter)?;
        let name_count = iter.next_u64()?.try_into().ok()?;
        let name = String::from_utf8(iter.take(name_count).collect()).ok()?;
        Some(Self::new(descriptor, new, order, state, name))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseBind {}
impl ResponseBind {
    pub const KIND_TAG: u64 = 151;
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
        Response::Bind(self)
    }
}
#[test]
fn test_request_bind() {
    let before = RequestBind::new(
        12345,
        67890,
        Order::Before,
        State::default(),
        "test test test".to_string(),
    );
    let after = RequestBind::string_from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_bind() {
    let before = ResponseBind::new();
    let after = ResponseBind::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
