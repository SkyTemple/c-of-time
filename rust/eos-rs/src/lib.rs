//! API for working with Rust codebases embeded into the
//! ROM for Pokémon Mystery Dungeon Explorers of Sky*.
//!
//! This is designed to be used as part of the [Rust subsystem of `c-of-time`](https://github.com/tech-ticks/c-of-time/rust).
//!
//! This crate contains a high level Rust-idiomatic API in the [`api`] module and
//! a low level API in the [`ffi`] module (consisting of `bindgen` generated bindings to known
//! functions in the base game).
//!
//! Pulling in this crate will also configure the allocator to use the game's allocation functions
//! and sets up a panic handler.
//!
//! *: This is NOT an official project. This is is in NO WAY affiliated
//! with Nintendo, The Pokémon Company, Spike Chunsoft or any of their
//! affiliates. This is an unofficial fan project.

#![no_std]
#![feature(alloc_error_handler)]
// This will be stable pretty soon:
#![feature(alloc_c_string)]
#![feature(core_c_str)]

extern crate alloc;
extern crate compiler_builtins_local;

#[macro_use] mod macros;

pub mod prelude;
pub mod ctypes;
pub mod ffi;
pub mod log_impl;
pub mod api;
pub mod string_util;
pub mod allocation;

pub use eos_rs_proc::patches;

mod panic;
