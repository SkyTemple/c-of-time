#![no_std]
#![feature(alloc_error_handler)]
#![feature(core_intrinsics)]
// This will be stable pretty soon:
#![feature(alloc_c_string)]
#![feature(core_c_str)]

extern crate alloc;
extern crate compiler_builtins_local;

#[macro_use] mod macros;
pub mod prelude;
pub mod ctypes;
mod allocation;
mod panic;
pub mod ffi;
// pub only for proc macros crate.
pub mod log_impl;
pub mod api;
pub mod string_util;
