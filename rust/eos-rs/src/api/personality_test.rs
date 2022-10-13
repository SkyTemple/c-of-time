//! Functions for the personality test.

use crate::api::overlay::OverlayLoadLease;
use crate::ffi;

/// Returns the personality obtained after answering all the questions.
///
/// The value to return is determined by checking the points obtained for each the personalities
/// and returning the one with the highest amount of points.
pub fn get_personality(_ov13: &OverlayLoadLease<13>) -> i32 {
    unsafe { ffi::GetPersonality() }
}
