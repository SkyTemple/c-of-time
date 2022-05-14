//! Functions for generating random numbers.

use crate::ctypes::*;
use crate::ffi;
use core::ops::{Bound, RangeBounds};

/// Internal trait for test mocking.
pub(crate) trait Rng {
    fn rand16(&mut self) -> u16;
    fn rand32(&mut self) -> u32;
    fn rand_range32(&mut self, x: c_int, y: c_int) -> c_int;
}

struct GameRng;
impl Rng for GameRng {
    fn rand16(&mut self) -> u16 {
        unsafe { ffi::Rand16Bit() }
    }

    fn rand32(&mut self) -> u32 {
        unsafe { ffi::Rand32Bit() }
    }

    fn rand_range32(&mut self, x: c_int, y: c_int) -> c_int {
        unsafe { ffi::RandRange(x, y) }
    }
}

/// Gets the game's general PRNG seed.
pub fn get_seed() -> u16 {
    unsafe { ffi::GetRngSeed() }
}

/// Sets the games general PRNG seed.
pub fn set_seed(seed: u16) {
    unsafe { ffi::SetRngSeed(seed) }
}

/// Generates a random number between the beginning and end of the range.
/// If the range is unbounded, min and/or max values are bound to
/// 0 ([`u16::MIN`]) and [`u16::MAX`] respectively.
///
/// The range must contain at least one element, or this will panic.
/// Same if the start bound is excluded.
pub fn rand_u16<R: RangeBounds<u16>>(range: R) -> u16 {
    rand_u16_internal(&mut GameRng, range)
}

/// Generates a random number between the beginning and end of the range.
/// If the range is unbounded, min and/or max values are bound to
/// [`i32::MIN`] and [`i32::MAX`] respectively.
///
/// The range must contain at least one element, or this will panic.
/// Same if the start bound is excluded.
pub fn rand_i32<R: RangeBounds<i32>>(range: R) -> i32 {
    rand_i32_internal(&mut GameRng, range)
}

pub(crate) fn rand_u16_internal<T: Rng, R: RangeBounds<u16>>(rng: &mut T, range: R) -> u16 {
    <u16 as RangeCheckable>::check_range(&range);

    let (min, max) = match (range.start_bound(), range.end_bound()) {
        (Bound::Unbounded, Bound::Unbounded) => return rng.rand16(),
        (Bound::Unbounded, Bound::Included(u)) => (u16::MIN, *u),
        (Bound::Unbounded, Bound::Excluded(u)) => (u16::MIN, *u - 1),
        (Bound::Included(l), Bound::Unbounded) => (*l, u16::MAX),
        (Bound::Included(l), Bound::Included(u)) => (*l, *u),
        (Bound::Included(l), Bound::Excluded(u)) => (*l, *u - 1),
        (Bound::Excluded(_), _) => {
            panic!("Excluded start ranges not supported.")
        }
    };

    let range = 1 + max - min;
    let buckets = u16::MAX / range;
    let limit = buckets * range;

    loop {
        let r = rng.rand16();
        if r < limit {
            return min + (r / buckets);
        }
    }
}

pub(crate) fn rand_i32_internal<T: Rng, R: RangeBounds<i32>>(rng: &mut T, range: R) -> i32 {
    <i32 as RangeCheckable>::check_range(&range);
    match (range.start_bound(), range.end_bound()) {
        (Bound::Unbounded, Bound::Unbounded) => rng.rand32() as i32, // overflow is ok for us here.
        (Bound::Unbounded, Bound::Included(u)) => rng.rand_range32(i32::MIN, u + 1),
        (Bound::Unbounded, Bound::Excluded(u)) => rng.rand_range32(i32::MIN, *u),
        // Note, this will never roll i32::MAX!
        (Bound::Included(l), Bound::Unbounded) => rng.rand_range32(*l, i32::MAX),
        (Bound::Included(l), Bound::Included(u)) => rng.rand_range32(*l, u + 1),
        (Bound::Included(l), Bound::Excluded(u)) => rng.rand_range32(*l, *u),
        (Bound::Excluded(_), _) => {
            panic!("Excluded start ranges not supported.")
        }
    }
}

trait RangeCheckable {
    /// Doesn't bother checking for start bound excluded since that's not supported anyway.
    fn check_range<R: RangeBounds<Self>>(range: &R);
}

impl RangeCheckable for i32 {
    fn check_range<R: RangeBounds<Self>>(range: &R) {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Included(l), Bound::Included(u)) => assert!(l <= u),
            (Bound::Included(l), Bound::Excluded(u)) => assert!(l < u),
            _ => (),
        }
    }
}

impl RangeCheckable for u16 {
    fn check_range<R: RangeBounds<Self>>(range: &R) {
        match (range.start_bound(), range.end_bound()) {
            (Bound::Included(l), Bound::Included(u)) => assert!(l <= u),
            (Bound::Included(l), Bound::Excluded(u)) => assert!(l < u),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ctypes::c_int;
    use alloc::vec::Vec;
    use core::ops::{Bound, RangeBounds};

    const RAND_16_RETURN: u16 = 2000;
    const RAND_32_RETURN: i32 = 1234;

    #[derive(Clone, Copy, Eq, PartialEq, Debug)]
    enum RngCall {
        Rand16,
        Rand32,
        RandRange32(c_int, c_int),
    }

    struct MockRng(Vec<RngCall>);

    impl Rng for MockRng {
        fn rand16(&mut self) -> u16 {
            self.0.push(RngCall::Rand16);
            RAND_16_RETURN
        }

        fn rand32(&mut self) -> u32 {
            self.0.push(RngCall::Rand32);
            RAND_32_RETURN as u32
        }

        fn rand_range32(&mut self, x: c_int, y: c_int) -> c_int {
            self.0.push(RngCall::RandRange32(x, y));
            RAND_32_RETURN
        }
    }

    /// For testing panic behaviour.
    struct ExcludedStartRange<'a, T> {
        end_bound: Bound<&'a T>,
        excluded_start: T,
    }

    impl<'a, T> RangeBounds<T> for ExcludedStartRange<'a, T> {
        fn start_bound(&self) -> Bound<&T> {
            Bound::Excluded(&self.excluded_start)
        }

        fn end_bound(&self) -> Bound<&T> {
            self.end_bound
        }
    }

    #[test]
    fn test_rand_i32_unbounded_unbounded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, ..);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand32);
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    fn test_rand_i32_unbounded_included() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, ..=10);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::RandRange32(i32::MIN, 11));
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    fn test_rand_i32_unbounded_excluded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, ..10);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::RandRange32(i32::MIN, 10));
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    fn test_rand_i32_included_unbounded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, 10..);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::RandRange32(10, i32::MAX));
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    fn test_rand_i32_included_included() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, 10..=20);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::RandRange32(10, 21));
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    fn test_rand_i32_included_excluded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_i32_internal(&mut mock, 10..20);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::RandRange32(10, 20));
        assert_eq!(result, RAND_32_RETURN);
    }

    #[test]
    #[should_panic]
    fn test_rand_i32_excluded_unbounded() {
        rand_i32_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Unbounded,
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[should_panic]
    fn test_rand_i32_excluded_included() {
        rand_i32_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Included(&0),
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[should_panic]
    fn test_rand_i32_excluded_excluded() {
        rand_i32_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Excluded(&0),
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_i32_reverse_range_excluded() {
        assert_eq!((3..2).into_iter().count(), 0);
        rand_i32_internal(&mut MockRng(Vec::new()), 3..2);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_i32_empty_range_excluded() {
        assert_eq!((3..3).into_iter().count(), 0);
        rand_i32_internal(&mut MockRng(Vec::new()), 3..3);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_i32_reverse_range_included() {
        assert_eq!((3..=1).into_iter().count(), 0);
        rand_i32_internal(&mut MockRng(Vec::new()), 3..=1);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_i32_empty_range_included_anti() {
        assert_eq!((3..=2).into_iter().count(), 0);
        rand_i32_internal(&mut MockRng(Vec::new()), 3..=2);
    }

    #[test]
    fn test_rand_u16_unbounded_unbounded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, ..);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, RAND_16_RETURN);
    }

    #[test]
    fn test_rand_u16_unbounded_included() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, ..=10);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, RAND_16_RETURN / (u16::MAX / 11));
    }

    #[test]
    fn test_rand_u16_unbounded_excluded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, ..10);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, RAND_16_RETURN % 10);
    }

    #[test]
    fn test_rand_u16_included_unbounded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, 10..);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, RAND_16_RETURN + 10);
    }

    #[test]
    fn test_rand_u16_included_included() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, 10..=20);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, 10 + RAND_16_RETURN / (u16::MAX / 11));
    }

    #[test]
    fn test_rand_u16_included_excluded() {
        let mut mock = MockRng(Vec::new());
        let result = rand_u16_internal(&mut mock, 10..20);
        assert_eq!(mock.0.len(), 1);
        assert_eq!(mock.0[0], RngCall::Rand16);
        assert_eq!(result, (RAND_16_RETURN % 10) + 10);
    }

    #[test]
    #[should_panic]
    fn test_rand_u16_excluded_unbounded() {
        rand_u16_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Unbounded,
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[should_panic]
    fn test_rand_u16_excluded_included() {
        rand_u16_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Included(&0),
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[should_panic]
    fn test_rand_u16_excluded_excluded() {
        rand_u16_internal(
            &mut MockRng(Vec::new()),
            ExcludedStartRange {
                end_bound: Bound::Excluded(&0),
                excluded_start: 0,
            },
        );
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_u16_reverse_range_excluded() {
        assert_eq!((3..2).into_iter().count(), 0);
        rand_u16_internal(&mut MockRng(Vec::new()), 3..2);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_u16_empty_range_excluded() {
        assert_eq!((3..3).into_iter().count(), 0);
        rand_u16_internal(&mut MockRng(Vec::new()), 3..3);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_u16_reverse_range_included() {
        assert_eq!((3..=1).into_iter().count(), 0);
        rand_u16_internal(&mut MockRng(Vec::new()), 3..=1);
    }

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    #[should_panic]
    fn test_rand_u16_empty_range_included_anti() {
        assert_eq!((3..=2).into_iter().count(), 0);
        rand_u16_internal(&mut MockRng(Vec::new()), 3..=2);
    }
}
