//! Handling of SIR0-wrapped files.

use crate::ffi;
use core::ptr;

/// Translates the offsets in a SIR0 file into NDS memory addresses, changes the magic number to
/// SirO (opened), and returns a pointer to the first pointer specified in the SIR0 header
/// (beginning of the data).
///
/// # Safety
/// The caller needs to make sure `src` points to a valid SIR0 file buffer.
pub unsafe fn translate_sir0(src: *mut u8) -> *mut u8 {
    let mut dst = ptr::null_mut();
    ffi::HandleSir0Translation((&mut dst) as *mut *mut u8, src);
    dst
}
