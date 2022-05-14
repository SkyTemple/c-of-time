//! API for working with Rust codebases embeded into the
//! ROM for Pokémon Mystery Dungeon Explorers of Sky*.
//!
//! This is designed to be used as part of the [Rust subsystem of `c-of-time`](https://github.com/tech-ticks/c-of-time/rust).
//!
//! This crate contains a high level Rust-idiomatic API in the [`api`] module and
//! a low level API in the [`ffi`] module (consisting of `bindgen` generated bindings to known
//! functions in the base game).
//!
//! This crate also contains a replacement implementation of `std::io` inside [`api::io`].
//!
//! Pulling in this crate will also configure the allocator to use the game's allocation functions
//! and sets up a panic handler.
//!
//! Please note that this entire crate relies on reverse-engineering efforts of the
//! [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug),
//! [c-of-time](https://github.com/tech-ticks/c-of-time) and
//! [SkyTemple](https://skytemple.org)
//! teams. There is going to be a lot of things missing, some functions
//! could have unintended side effects or be unsafe to call in some situations, even if marked safe,
//! and lastly both the low-level and high-level APIs are not stable, they WILL change when we find
//! out new things.
//!
//! Please open issues and/or Pull Requests on `pmdsky-debug` and/or `c-of-time` if you discover
//! issues or new insights.
//!
//! *: This is NOT an official project. This is is in NO WAY affiliated
//! with Nintendo, The Pokémon Company, Spike Chunsoft or any of their
//! affiliates. This is an unofficial fan project.

// Allow std for tests. Make sure to NOT cross-compile when using tests.
#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
// This will be stable pretty soon:
#![feature(alloc_c_string)]
#![feature(core_c_str)]
#![feature(allocator_api)]
#![feature(nonnull_slice_from_raw_parts)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;
extern crate compiler_builtins_local;

#[macro_use]
mod macros;

pub mod api;
pub mod ctypes;
pub mod ffi;
pub mod log_impl;
pub mod prelude;
pub mod string_util;
mod util;

pub mod allocation;

pub use eos_rs_proc::patches;

#[cfg(not(test))]
mod panic;
