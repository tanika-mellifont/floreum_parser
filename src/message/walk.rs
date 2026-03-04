use crate::{FloreumError, State, names::Names, read_state, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequestWalk<C: AsRef<[u8]>> {
    pub descriptor: u64,
    pub state: State,
    pub names: Names<C>,
}
impl<C: AsRef<[u8]>> RequestWalk<C> {
    pub const KIND_TAG: u64 = 30;
    pub fn new(descriptor: u64, state: State, names: Names<C>) -> Self {
        Self {
            descriptor,
            state,
            names,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.state.to_iter())
            .chain(self.names.bytes().copied())
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> RequestWalk<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let descriptor = read_u64(bytes)?;
        let state = read_state(bytes)?;
        let names = Names::from_bytes(bytes)?;
        Ok(Self::new(descriptor, state, names))
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResponseWalk {
    pub descriptor: u64,
}
impl ResponseWalk {
    pub const KIND_TAG: u64 = 31;
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
fn test_request_walk() {
    #[derive(PartialEq)]
    pub struct SizedBuffer([u8; 1024]);
    impl AsRef<[u8]> for SizedBuffer {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }
    impl<'a> From<&'a [u8]> for SizedBuffer {
        fn from(value: &'a [u8]) -> Self {
            let mut buffer = [0; 1024];
            for (to, from) in buffer.iter_mut().zip(value.iter()) {
                *to = *from;
            }
            Self(buffer)
        }
    }
    let mut strings_buffer = [0u8; 2048];
    let strings = ["test1", "test2", "test3"];
    for (to, from) in
        strings_buffer
            .iter_mut()
            .zip(
                TryInto::<u64>::try_into(strings.len()).unwrap()
                    .to_le_bytes()
                    .into_iter()
                    .chain(strings.iter().flat_map(|string| {
                        TryInto::<u64>::try_into(string.len()).unwrap()
                            .to_le_bytes()
                            .into_iter()
                            .chain(string.as_bytes().iter().copied())
                    })),
            )
    {
        *to = from;
    }
    let mut strings_cursor = &strings_buffer as &[u8];
    let before = RequestWalk::new(
        12345,
        State::default(),
        Names::<SizedBuffer>::from_bytes(&mut strings_cursor).unwrap(),
    );
    let mut buffer = [0; 4096];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = RequestWalk::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_walk() {
    let before = ResponseWalk::new(12345);
    let mut buffer = [0; 1024];
    for (to, from) in buffer.iter_mut().zip(before.to_iter()) {
        *to = from;
    }
    let mut cursor = &buffer as &[u8];
    let after = ResponseWalk::from_bytes(&mut cursor).unwrap();
    assert!(before == after);
}
