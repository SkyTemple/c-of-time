//! Functions for generating random numbers.

use core::ops::{Bound, RangeBounds};
use crate::ffi;

pub fn get_seed() -> u16 {
    unsafe { ffi::GetRngSeed() }
}

pub fn set_seed(seed: u16) {
    unsafe { ffi::SetRngSeed(seed) }
}

// TODO UNIT TESTS
/// Generates a random number between the beginning and end of the range.
/// If the range is unbounded, min and/or max values are bound to 0 and the
/// maximum 16-bit unsigned integer value respectively.
pub fn rand_u16<R: RangeBounds<u16>>(_range: R) -> u16 {
    todo!()
}

// TODO UNIT TESTS
/// Generates a random number between the beginning and end of the range.
/// If the range is unbounded, min and/or max values are bound to 0 and the
/// maximum 32-bit signed integer values respectively.
/// https://docs.rs/mockall/latest/mockall/
pub fn rand_i32<R: RangeBounds<i32>>(range: R) -> i32 {
    unsafe {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Unbounded, Bound::Unbounded) => ffi::Rand32Bit() as i32,  // overflow is ok for us here.
            (Bound::Unbounded, Bound::Included(u)) => ffi::RandRange(0, u + 1),
            (Bound::Unbounded, Bound::Excluded(u)) => ffi::RandRange(0, *u),
            // Note, this will never roll i32::MAX!
            (Bound::Included(l), Bound::Unbounded) => ffi::RandRange(*l, i32::MAX),
            (Bound::Included(l), Bound::Included(u)) => ffi::RandRange(*l, u + 1),
            (Bound::Included(l), Bound::Excluded(u)) => ffi::RandRange(*l, *u),
            // Bounds in Rust can't start with an excluded bound.
            (Bound::Excluded(_), _) => { unreachable!() }
        }
    }
}

// uint16_t Rand16Bit(void);
// uint32_t RandInt(uint32_t n);
// int RandRange(int x, int y);
// uint32_t Rand32Bit(void);
// uint32_t RandIntSafe(uint32_t n);
// int RandRangeSafe(int x, int y);
