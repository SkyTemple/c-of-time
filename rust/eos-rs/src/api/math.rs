//! Math functions.

use crate::ffi;

/// Computes the Euclidean norm of a two-component integer array, sort of like hypotf(3).
pub fn euclidean_norm(vec2: &[i32; 2]) -> f32 {
    unsafe { ffi::EuclideanNorm(vec2.as_ptr() as *mut _) }
}

/// Clamps the absolute values in a two-component integer array.
///
/// Given an integer array [x, y] and a maximum absolute value M, clamps each element of the array
/// to M (`max`) such that the output array is [min(max(x, -M), M), min(max(y, -M), M)].
pub fn clamp_component_abs(vec2: &mut [i32; 2], max: i32) {
    unsafe { ffi::ClampComponentAbs(vec2.as_mut_ptr(), max) }
}
