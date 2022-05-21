//! Dealing with [fixed-point numbers](https://en.wikipedia.org/wiki/Fixed-point_arithmetic)
//! used in the game.
//!
//! These types can have `Frac` fractional bits, where 0 ≤ `Frac` ≤ *n*
//! and *n* is the total number of bits. When `Frac` = 0, the fixed-point
//! number behaves like an *n*-bit integer. When `Frac` = *n*, the value
//! *x* lies in the range −0.5 ≤ *x* < 0.5 for signed numbers, and in the
//! range 0 ≤ *x* < 1 for unsigned numbers.
//!
//! Think of these similar to floats, but instead of having an arbitrary amount of
//! fractional digits/bits and arbitrary precision, fixed-point numbers have a set
//! amount of fractional digits/bits that are fully accurate.
//!
//! Commonly used fixed numbers:
//! - [`I24F8`]: 32-bit number that has 24 integer bits and eight fractional bits.
//!
//! You have several options to create a fixed-point number:
//!
//! ```
//! let n1 = I24F8::from_num(10);
//! assert_eq!(n1, 10.0);
//!
//! // This will round to the nearest possible fixed representation. In this case,
//! // this value will fit, since 2 fits in a 24-bit integer and 75 fits in an
//! // 8-bit integer.
//! let n2 = I24F8::from_num(2.75);
//! // Note that due to precision differences this assertion can fail with some values.
//! assert_eq!(n2, 2.75);
//!
//! // It's also possible (and probably faster) to directly use a number already encoded
//! // as a fixed number. This has the lower byte set to 0, which means the fractional
//! // bit will be 0, and the upper byte to 1, which means this is "1.0".
//! let n3 = I24F8::from_bits(0x01_00);
//! assert_eq!(n3, 1.0);
//! ```
//!
//!
pub use fixed::types::*;


// Since doctests don't work, we turn the doctest into a normal unit test here.
#[cfg(test)]
mod test {
    use super::I24F8;

    #[test]
    pub fn test_documentation() {
        let n1 = I24F8::from_num(10);
        assert_eq!(n1, 10.0);

        let n2 = I24F8::from_num(2.75);
        assert_eq!(n2, 2.75);

        let n3 = I24F8::from_bits(0x01_00);
        assert_eq!(n3, 1.0);
    }
}
