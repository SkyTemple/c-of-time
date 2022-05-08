//! Utilities for dealing with string conversion.

use alloc::ffi::CString;

use core::fmt::Debug;

#[inline]
/// Converts a Rust String to a CString.
pub fn str_to_cstring<S: AsRef<str> + Debug>(s: S) -> CString {
    CString::new(s.as_ref())
        .unwrap_or_else(|_| panic!("Was unable to convert {:?} to CString.", s.as_ref()))
}
