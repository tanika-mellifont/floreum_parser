use crate::FloreumError;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Head {
    Forward,
    Backward,
    Start,
    End,
}
impl Head {
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
}
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    pub walk: bool,
    pub make: bool,
    pub remove: bool,
    pub rename: bool,
    pub read_peek: bool,
    pub read_seek: bool,
    pub insert_forward: bool,
    pub insert_backward: bool,
    pub overwrite_forward_peek: bool,
    pub overwrite_forward_seek: bool,
    pub overwrite_backward_peek: bool,
    pub overwrite_backward_seek: bool,
    pub truncate_forward: bool,
    pub truncate_backward: bool,
    pub seek_forward: bool,
    pub seek_backward: bool,
    pub seek_start: bool,
    pub seek_end: bool,
    pub tell: bool,
    #[cfg(feature = "bind")]
    pub bind_before: bool,
    #[cfg(not(feature = "bind"))]
    bind_before: bool,
    #[cfg(feature = "bind")]
    pub bind_after: bool,
    #[cfg(not(feature = "bind"))]
    bind_after: bool,
}
macro_rules! state_chain {
    ($setter:ident, $field:ident) => {
        impl State {
            pub fn $setter(mut self, value: bool) -> Self {
                self.$field = value;
                self
            }
        }
    };
}
impl State {
    pub(crate) fn to_iter(&self) -> impl Iterator<Item = u8> {
        let mut buffer = [0; 3 * size_of::<u64>()];
        for (index, value) in [
            self.walk,
            self.make,
            self.remove,
            self.rename,
            self.read_peek,
            self.read_seek,
            self.insert_forward,
            self.insert_backward,
            self.overwrite_forward_peek,
            self.overwrite_forward_seek,
            self.overwrite_backward_peek,
            self.overwrite_backward_seek,
            self.truncate_forward,
            self.truncate_backward,
            self.seek_forward,
            self.seek_backward,
            self.seek_start,
            self.seek_end,
            self.tell,
            self.bind_before,
            self.bind_after,
        ]
        .iter_mut()
        .enumerate()
        {
            *buffer.get_mut(index).unwrap() = if *value { 1 } else { 0 };
        }
        buffer.into_iter()
    }
}
state_chain!(with_walk, walk);
state_chain!(with_make, make);
state_chain!(with_remove, remove);
state_chain!(with_rename, rename);
state_chain!(with_read_peek, read_peek);
state_chain!(with_read_seek, read_seek);
state_chain!(with_insert_forward, insert_forward);
state_chain!(with_insert_backward, insert_backward);
state_chain!(with_overwrite_forward_peek, overwrite_forward_peek);
state_chain!(with_overwrite_forward_seek, overwrite_forward_seek);
state_chain!(with_overwrite_backward_peek, overwrite_backward_peek);
state_chain!(with_overwrite_backward_seek, overwrite_backward_seek);
state_chain!(with_truncate_forward, truncate_forward);
state_chain!(with_truncate_backward, truncate_backward);
state_chain!(with_seek_forward, seek_forward);
state_chain!(with_seek_backward, seek_backward);
state_chain!(with_seek_start, seek_start);
state_chain!(with_seek_end, seek_end);
state_chain!(with_tell, tell);
#[cfg(feature = "bind")]
state_chain!(with_bind_before, bind_before);
#[cfg(feature = "bind")]
state_chain!(with_bind_after, bind_after);
fn read<'a>(bytes: &mut &'a [u8], len: usize) -> Result<&'a [u8], FloreumError> {
    let (head, tail) = bytes
        .split_at_checked(len)
        .map_or(Err(FloreumError::Truncation), |ok| Ok(ok))?;
    *bytes = tail;
    Ok(head)
}
pub fn read_bool(bytes: &mut &[u8]) -> Result<bool, FloreumError> {
    Ok(*read(bytes, 1)?
        .get(0)
        .map_or(Err(FloreumError::Truncation), |ok| Ok(ok))?
        != 0)
}
pub fn read_u64(bytes: &mut &[u8]) -> Result<u64, FloreumError> {
    Ok(u64::from_le_bytes(
        read(bytes, 8)?
            .as_array()
            .map_or(Err(FloreumError::LocalBitWidth), |ok| Ok(ok))
            .copied()?,
    ))
}
pub fn read_head(bytes: &mut &[u8]) -> Result<Head, FloreumError> {
    Ok(match read_u64(bytes)? {
        Head::FORWARD => Head::Forward,
        Head::BACKWARD => Head::Backward,
        Head::START => Head::Start,
        Head::END => Head::End,
        _ => Err(FloreumError::Domain)?,
    })
}
pub fn read_direction(bytes: &mut &[u8]) -> Result<Direction, FloreumError> {
    Ok(match read_u64(bytes)? {
        Direction::FORWARD => Direction::Forward,
        Direction::BACKWARD => Direction::Backward,
        _ => Err(FloreumError::Domain)?,
    })
}
pub fn read_order(bytes: &mut &[u8]) -> Result<Order, FloreumError> {
    Ok(match read_u64(bytes)? {
        Order::BEFORE => Order::Before,
        Order::AFTER => Order::After,
        _ => Err(FloreumError::Domain)?,
    })
}
pub fn read_state(bytes: &mut &[u8]) -> Result<State, FloreumError> {
    Ok(State {
        walk: read_bool(bytes)?,
        make: read_bool(bytes)?,
        remove: read_bool(bytes)?,
        rename: read_bool(bytes)?,
        read_peek: read_bool(bytes)?,
        read_seek: read_bool(bytes)?,
        insert_forward: read_bool(bytes)?,
        insert_backward: read_bool(bytes)?,
        overwrite_forward_peek: read_bool(bytes)?,
        overwrite_forward_seek: read_bool(bytes)?,
        overwrite_backward_peek: read_bool(bytes)?,
        overwrite_backward_seek: read_bool(bytes)?,
        truncate_forward: read_bool(bytes)?,
        truncate_backward: read_bool(bytes)?,
        seek_forward: read_bool(bytes)?,
        seek_backward: read_bool(bytes)?,
        seek_start: read_bool(bytes)?,
        seek_end: read_bool(bytes)?,
        tell: read_bool(bytes)?,
        bind_before: read_bool(bytes)?,
        bind_after: read_bool(bytes)?,
    })
}
pub fn read_content<'a>(bytes: &mut &'a [u8]) -> Result<&'a [u8], FloreumError> {
    let len = read_u64(bytes)?;
    Ok(read(
        bytes,
        len.try_into().map_err(|_| FloreumError::LocalBitWidth)?,
    )?)
}
pub fn read_str<'a>(bytes: &mut &'a [u8]) -> Result<&'a str, FloreumError> {
    let len = read_u64(bytes).map_or(Err(FloreumError::Truncation), |len| Ok(len))?;
    str::from_utf8(read(
        bytes,
        len.try_into().map_err(|_| FloreumError::LocalBitWidth)?,
    )?)
    .map_err(|_| FloreumError::Utf8)
}
