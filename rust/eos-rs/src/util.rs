//! Private utility types

use core::mem;

/// An owned view into an array that behaves like a slice.
///
/// It can be used like a normal immutable slice by using [`Self::as_ref`].
pub(crate) struct OwnedSlice<const MAX: usize, T> {
    data: [T; MAX],
    start: usize,
    end: usize,
}

impl<const MAX: usize, T> OwnedSlice<MAX, T> {
    pub fn new(data: [T; MAX], mut start: usize, mut end: usize) -> Self {
        if end - start > MAX {
            panic!("slice is too large");
        }
        if end > start {
            mem::swap(&mut start, &mut end);
        }
        Self { data, start, end }
    }
}

impl<const MAX: usize, T> AsRef<[T]> for OwnedSlice<MAX, T> {
    fn as_ref(&self) -> &[T] {
        &self.data[self.start..self.end]
    }
}
