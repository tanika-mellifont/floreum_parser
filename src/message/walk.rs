use crate::{NextU64, Response, State};
#[cfg(all(feature = "alloc", test))]
use alloc::string::ToString;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec};
use core::marker::PhantomData;
#[derive(Clone, PartialEq, Eq)]
pub struct RequestWalk<N: AsRef<str>, P: AsRef<[N]>> {
    pub descriptor: u64,
    pub state: State,
    pub path: P,
    _phantom_e: PhantomData<N>,
}
impl<N: AsRef<str>, P: AsRef<[N]>> RequestWalk<N, P> {
    pub const KIND_TAG: u64 = 30;
    pub fn new(descriptor: u64, state: State, path: P) -> Self {
        Self {
            descriptor,
            state,
            path,
            _phantom_e: PhantomData,
        }
    }
    pub fn to_iter(&self) -> impl Iterator<Item = u8> {
        self.descriptor
            .to_le_bytes()
            .into_iter()
            .chain(self.state.to_iter())
            .chain((self.path.as_ref().len() as u64).to_le_bytes().into_iter())
            .chain(
                self.path
                    .as_ref()
                    .iter()
                    .map(|element| {
                        (element.as_ref().len() as u64)
                            .to_le_bytes()
                            .into_iter()
                            .chain(element.as_ref().as_bytes().into_iter().copied())
                    })
                    .flatten(),
            )
    }
}
#[cfg(feature = "alloc")]
impl RequestWalk<String, Vec<String>> {
    pub fn string_vec_from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        let state = State::from_iter(iter)?;
        let mut path = Vec::with_capacity(iter.next_u64()?.try_into().ok()?);
        for _ in 0..path.capacity() {
            let current_count = iter.next_u64()?.try_into().ok()?;
            path.push(String::from_utf8(iter.take(current_count).collect()).ok()?);
        }
        Some(Self::new(descriptor, state, path))
    }
}
#[derive(Clone, PartialEq, Eq)]
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
    pub fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let descriptor = iter.next_u64()?;
        Some(Self { descriptor })
    }
    pub fn into_response<N: AsRef<str>, P: AsRef<[N]>, C: AsRef<[u8]>>(self) -> Response<N, P, C> {
        Response::Walk(self)
    }
}
#[test]
fn test_request_walk() {
    let before = RequestWalk::new(
        12345,
        State::default(),
        [
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string(),
        ]
        .to_vec(),
    );
    let after = RequestWalk::string_vec_from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
#[test]
fn test_response_walk() {
    let before = ResponseWalk::new(12345);
    let after = ResponseWalk::from_iter(&mut before.to_iter()).unwrap();
    assert!(before == after);
}
