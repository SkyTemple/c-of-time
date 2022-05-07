//! Utilities for dealing with string conversion.

use alloc::ffi::CString;
use alloc::format;
use core::fmt::Debug;

#[inline]
/// Converts a Rust String to a CString.
pub fn str_to_cstring<S: AsRef<str> + Debug>(s: S) -> CString {
    CString::new(s.as_ref())
        .expect(&format!("Was unable to convert {:?} to CString.", s.as_ref()))
}
