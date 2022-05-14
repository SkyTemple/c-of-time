# eos-rs

API for working with Rust codebases embeded into the
ROM for Pokémon Mystery Dungeon Explorers of Sky.

This is meant to be used as part of the `c-of-time` repository, with the crate in the parent directory.

Documentation: https://eosrs.pmdcollab.org/ (via [PMDCollab/c-of-time-eos-rs-doc](https://github.com/PMDCollab/c-of-time-eos-rs-doc))

More information can be found in the [parent README.md](https://github.com/tech-ticks/c-of-time/tree/main/rust) file.

## Sub-crates
This crate has a few internal sub-crates:

- `build/`: `eos-rs-build` - Build scripts for use with the `build.rs` of the entrypoint crate (see [../build.rs](https://github.com/tech-ticks/c-of-time/blob/main/rust/build.rs)).
- `patches-def/`: `eos-rs-patches-def` - `syn` crate parsing structs for the `patches!` proc marco.
- `proc/`: `eos-rs-proc` - Proc macro crate for the `patches!` macro (this is re-exported by `eos-rs`).
