//! Functions for the personality test.

use crate::api::overlay::OverlayLoadLease;
use crate::ctypes::c_char;
use crate::ffi;

/// Returns the personality obtained after answering all the questions.
///
/// The value to return is determined by checking the points obtained for each the personalities
/// and returning the one with the highest amount of points.
pub fn get_personality(_ov13: &OverlayLoadLease<13>) -> i32 {
    unsafe { ffi::GetPersonality() }
}

/// No description available.
///
/// Note: unverified, ported from Irdkwia's notes.
///
/// # Safety
/// It's unknown what size the string passed in must be and whether the output string
/// pointer is the same string or not. Maybe it re-allocates a new string if the passed in output
/// string does not fit the string.
pub unsafe fn get_option_string_from_id(
    output: *mut c_char,
    option_id: i32,
    _ov13: &OverlayLoadLease<13>,
) -> *mut c_char {
    ffi::GetOptionStringFromID(output, option_id)
}

/// No description available.
///
/// Note: unverified, ported from Irdkwia's notes
pub fn wait_for_next_step(switch_case: i32, _ov13: &OverlayLoadLease<13>) {
    unsafe { ffi::WaitForNextStep(switch_case) }
}
