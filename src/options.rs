use crate::NextU64;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Forward,
    Backward,
    Start,
    End,
}
impl Cursor {
    const FORWARD: u64 = 0;
    const BACKWARD: u64 = 1;
    const START: u64 = 2;
    const END: u64 = 3;
    pub(crate) fn to_iter(&self) -> impl Iterator<Item = u8> {
        (match self {
            Self::Forward => Self::FORWARD,
            Self::Backward => Self::BACKWARD,
            Self::Start => Self::START,
            Self::End => Self::END,
        } as u64)
            .to_le_bytes()
            .into_iter()
    }
    pub(crate) fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(match iter.next_u64()? {
            Self::FORWARD => Self::Forward,
            Self::BACKWARD => Self::Backward,
            Self::START => Self::Start,
            Self::END => Self::End,
            _ => None?,
        })
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}
impl Direction {
    const FORWARD: u64 = 0;
    const BACKWARD: u64 = 1;
    pub(crate) fn to_iter(&self) -> impl Iterator<Item = u8> {
        (match self {
            Self::Forward => Self::FORWARD,
            Self::Backward => Self::BACKWARD,
        } as u64)
            .to_le_bytes()
            .into_iter()
    }
    pub(crate) fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(match iter.next_u64()? {
            Self::FORWARD => Self::Forward,
            Self::BACKWARD => Self::Backward,
            _ => None?,
        })
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Order {
    Before,
    After,
}
impl Order {
    const BEFORE: u64 = 0;
    const AFTER: u64 = 1;
    pub(crate) fn to_iter(&self) -> impl Iterator<Item = u8> {
        (match self {
            Self::Before => Self::BEFORE,
            Self::After => Self::AFTER,
        } as u64)
            .to_le_bytes()
            .into_iter()
    }
    pub(crate) fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        Some(match iter.next_u64()? {
            Self::BEFORE => Self::Before,
            Self::AFTER => Self::After,
            _ => None?,
        })
    }
}
