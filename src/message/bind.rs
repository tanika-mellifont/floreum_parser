use crate::{FloreumError, Order, State, read_order, read_state, read_str, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestBind<N: AsRef<str>> {
    pub descriptor: u64,
    pub new: u64,
    pub order: Order,
    pub state: State,
    pub name: N,
}
impl<N: AsRef<str>> RequestBind<N> {
    pub const KIND_TAG: u64 = 160;
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
}
impl<'a, N: AsRef<str> + From<&'a str>> RequestBind<N> {
    pub fn from_bytes(bytes: &mut &'a [u8]) -> Result<Self, FloreumError> {
        Ok(Self::new(
            read_u64(bytes)?,
            read_u64(bytes)?,
            read_order(bytes)?,
            read_state(bytes)?,
            read_str(bytes)?.into(),
        ))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseBind {}
impl ResponseBind {
    pub const KIND_TAG: u64 = 161;
    pub fn new() -> Self {
        Self {}
    }
}
#[cfg(feature = "bind")]
#[test]
fn test_request_bind() {
    let before = RequestBind::new(
        12345,
        67890,
        Order::Before,
        State::default(),
        "test test test",
    );
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestBind::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
