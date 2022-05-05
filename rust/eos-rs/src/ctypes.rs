#![allow(non_camel_case_types)]

pub type c_char = i8;
pub type c_short = i16;
pub type c_int = i32;
pub type c_long = i32;
pub type c_longlong = i64;

pub type c_uchar = u8;
pub type c_ushort = u16;
pub type c_uint = u32;
pub type c_ulong = u32;
pub type c_ulonglong = u64;

/// Instances of this enum can never actually exist.
/// This is only meant to be used as a type for pointers.
/// Do not use the variants. See notes for libc::c_void!
#[repr(u8)]
pub enum c_void {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}
