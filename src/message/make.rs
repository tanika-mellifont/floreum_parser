use crate::{FloreumError, State, read_state, read_str, read_u64};
#[derive(Clone, PartialEq, Eq)]
pub struct RequestMake<N: AsRef<str>> {
    pub descriptor: u64,
    pub state: State,
    pub name: N,
}
impl<N: AsRef<str>> RequestMake<N> {
    pub const KIND_TAG: u64 = 70;
    pub fn new(descriptor: u64, state: State, name: N) -> Self {
        Self {
            descriptor,
            state,
            name,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.state.to_iter())
            .chain(
                (self.name.as_ref().len() as u64)
                    .to_le_bytes()
                    .into_iter()
                    .chain(self.name.as_ref().as_bytes().iter().copied()),
            )
    }
}
impl<N: AsRef<str> + for<'a> From<&'a str>> RequestMake<N> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let state = read_state(bytes)?;
        let name = read_str(bytes)?.into();
        Ok(Self::new(descriptor, state, name))
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct ResponseMake {
    pub descriptor: u64,
}
impl ResponseMake {
    pub const KIND_TAG: u64 = 71;
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
#[test]
fn test_request_make() {
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
    let before = RequestMake::new(12345, State::default(), SizedString(test_array));
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestMake::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_make() {
    let before = ResponseMake::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseMake::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
