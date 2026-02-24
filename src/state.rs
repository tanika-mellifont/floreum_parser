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
        pub fn $setter(mut self, value: bool) -> Self {
            self.$field = value;
            self
        }
    };
}
impl State {
    pub(crate) fn from_iter(iter: &mut impl Iterator<Item = u8>) -> Option<Self> {
        let mut buffer = [0; 3 * size_of::<u64>()];
        for byte in buffer.iter_mut() {
            *byte = iter.next()?;
        }
        let mut aligned = buffer.into_iter();
        Some(Self {
            walk: aligned.next()? != 0,
            make: aligned.next()? != 0,
            remove: aligned.next()? != 0,
            rename: aligned.next()? != 0,
            read_peek: aligned.next()? != 0,
            read_seek: aligned.next()? != 0,
            insert_forward: aligned.next()? != 0,
            insert_backward: aligned.next()? != 0,
            overwrite_forward_peek: aligned.next()? != 0,
            overwrite_forward_seek: aligned.next()? != 0,
            overwrite_backward_peek: aligned.next()? != 0,
            overwrite_backward_seek: aligned.next()? != 0,
            truncate_forward: aligned.next()? != 0,
            truncate_backward: aligned.next()? != 0,
            seek_forward: aligned.next()? != 0,
            seek_backward: aligned.next()? != 0,
            seek_start: aligned.next()? != 0,
            seek_end: aligned.next()? != 0,
            tell: aligned.next()? != 0,
            bind_before: aligned.next()? != 0,
            bind_after: aligned.next()? != 0,
        })
    }
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
}
