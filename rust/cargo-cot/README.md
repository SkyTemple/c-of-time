# cargo-cot

Cargo extension to build Rust `c-of-time` projects.

Install via `cargo install --path .`.

More information can be found in the [parent README.md](https://github.com/tech-ticks/c-of-time/tree/main/rust) file.

## Commands
To be run from `/rust` inside this repository (the entrypoint crate).


### `cargo cot build`
```
Builds the project, including the C codebase. This is equivalent to `cargo build
-Zbuild-std=core,alloc --target ./armv5te-none-ndseoseabi-XX.json`, where XX is the region specified

USAGE:
    cargo cot build [OPTIONS] <REGION> [-- <CARGO_ARGS>...]

ARGS:
    <REGION>           The region to build for; `eu`, `na` or `jp`
    <CARGO_ARGS>...    Any additional argument after '--' will be forwarded to cargo build

OPTIONS:
    -h, --help       Print help information
    -r, --release    Build artifacts in release mode, with optimization


```

### `cargo cot burn`

```
Build the project and write the project to a ROM. Overlay 36 is patched and patches in ../patches
are applied to the game (including the glue code from the `patches!` macro)

USAGE:
    cargo cot burn [OPTIONS] <REGION> <ROM_PATH> <OUT_PATH> [-- <CARGO_ARGS>...]

ARGS:
    <REGION>           The region to build for; `eu`, `na` or `jp`
    <ROM_PATH>         Path to the input ROM to patch
    <OUT_PATH>         Path where the patched ROM should be saved
    <CARGO_ARGS>...    Any additional argument after '--' will be forwarded to cargo build

OPTIONS:
    -h, --help       Print help information
    -r, --release    Build artifacts in release mode, with optimization
```
