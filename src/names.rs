use crate::{FloreumError, read_str, read_u64};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Names<C: AsRef<[u8]>>(C, usize);
pub struct NamesIter<'a, S: AsRef<[u8]>> {
    names: &'a Names<S>,
    byte: usize,
    index: usize,
    count: usize,
}
impl<'a, C: AsRef<[u8]>> Iterator for NamesIter<'a, C> {
    type Item = Result<&'a str, FloreumError>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            self.index += 1;
            let mut cursor = self
                .names
                .0
                .as_ref()
                .get(self.byte..)
                .map_or(Err(FloreumError::TruncatedNamesIter), |ok| Ok(ok))
                .unwrap();
            match read_u64(&mut cursor).map_or(Err(FloreumError::TruncatedNamesIter), |ok| {
                ok.try_into()
                    .ok()
                    .map_or(Err(FloreumError::LocalBitWidth), |ok| Ok(ok))
            }) {
                Ok(len) => Some(
                    cursor
                        .get(..len)
                        .map_or(Err(FloreumError::TruncatedNamesIter), |ok| {
                            str::from_utf8(ok).map_or(Err(FloreumError::Utf8), |ok| Ok(ok))
                        }),
                ),
                Err(error) => Some(Err(error)),
            }
        } else {
            None
        }
    }
}
impl<C: AsRef<[u8]>> Names<C> {
    pub fn bytes(&self) -> impl Iterator<Item = &u8> {
        self.0.as_ref().iter()
    }
    pub fn iter<'a>(&'a self) -> NamesIter<'a, C> {
        NamesIter {
            names: self,
            byte: 0,
            index: 0,
            count: self.1,
        }
    }
    pub fn len(&self) -> u64 {
        self.iter().count() as u64
    }
}
impl<C: AsRef<[u8]> + for<'a> From<&'a [u8]>> Names<C> {
    pub fn from_bytes(bytes: &mut &[u8]) -> Result<Self, FloreumError> {
        let original = *bytes;
        let mut cursor = original;
        let count = read_u64(&mut cursor)?.try_into().map_err(|_| FloreumError::LocalBitWidth)?;
        let mut byte_len = size_of::<u64>();
        for _ in 0..count {
            byte_len += size_of::<u64>() + read_str(&mut cursor)?.len();
        }
        if original.len() < byte_len {
            return Err(FloreumError::TruncatedContent { expected: byte_len as u64 });
        }
        let (data, rest) = original.split_at(byte_len);
        *bytes = rest;
        Ok(Self(data.into(), count))
    }
}
impl<C: AsRef<[u8]> + FromIterator<u8>> Names<C> {
    pub fn from_iter<'a, I: Iterator<Item = &'a str>>(iter: I) -> Result<Self, FloreumError> {
        let mut count = 0;
        let names = C::from_iter(iter.flat_map(|as_str| {
            count += 1;
            (as_str.len() as u64)
                .to_le_bytes()
                .into_iter()
                .chain(as_str.bytes().into_iter())
        }));
        Ok(Self(names, count))
    }
}
