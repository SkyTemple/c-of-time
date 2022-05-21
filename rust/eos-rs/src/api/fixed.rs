//! Dealing with [fixed-point numbers](https://en.wikipedia.org/wiki/Fixed-point_arithmetic)
//! used in the game.
//!
//! Note that this module currently only deals with binary fixed-point representations.
//! The game also sometimes uses decimal representations of fixed-point numbers
//! (eg. 0x64 -> 100 -> '01.00').
//!
//! This pulls in parts of the [`fixed`](https://docs.rs/fixed/latest/fixed/index.html) crate,
//! which describes these numbers as follows:
//!
//! > An <i>n</i>-bit fixed-point number has <i>f</i>&nbsp;=&nbsp;`Frac` fractional
//! > bits where 0&nbsp;≤&nbsp;<i>f</i>&nbsp;≤&nbsp;<i>n</i>, and
//! > <i>n</i>&nbsp;&minus;&nbsp;<i>f</i> integer bits. For example,
//! > <code>[FixedI32]\<[U24]></code> is a 32-bit signed fixed-point number with
//! > <i>n</i>&nbsp;=&nbsp;32 total bits, <i>f</i>&nbsp;=&nbsp;24 fractional bits, and
//! > <i>n</i>&nbsp;&minus;&nbsp;<i>f</i>&nbsp;=&nbsp;8 integer bits.
//! > <code>[FixedI32]\<[U0]></code> behaves like [`i32`], and
//! > <code>[FixedU32]\<[U0]></code> behaves like [`u32`].
//! >
//! > The difference between any two successive representable numbers is constant
//! > throughout the possible range for a fixed-point number:
//! > <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>f</i></sup>. When <i>f</i>&nbsp;=&nbsp;0, like
//! > in <code>[FixedI32]\<[U0]></code>, <i>Δ</i>&nbsp;=&nbsp;1 because representable
//! > numbers are integers, and the difference between two successive integers is 1.
//! > When <i>f</i>&nbsp;=&nbsp;<i>n</i>, <i>Δ</i>&nbsp;=&nbsp;1/2<sup><i>n</i></sup>
//! > and the value lies in the range &minus;0.5&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;0.5
//! > for signed numbers like <code>[FixedI32]\<[U32]></code>, and in the range
//! > 0&nbsp;≤&nbsp;<i>x</i>&nbsp;<&nbsp;1 for unsigned numbers like
//! > <code>[FixedU32]\<[U32]></code>.
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
//! [U0]: fixed::types::extra::U0
//! [U24]: fixed::types::extra::U24
//! [U32]: fixed::types::extra::U32
pub use fixed::{FixedU8, FixedI8, FixedU16, FixedI16, FixedU32, FixedI32};
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
