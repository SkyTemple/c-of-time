use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use eos_rs_build::*;

// To build, use cargo cot (eg. `cargo cot build na --release`),
// or cargo directly
// (eg. `cargo build -Zbuild-std=core,alloc --release --target ./armv5te-none-ndseoseabi-na.json`).

fn main() {
    let this_dir = PathBuf::from_str(
        env::var("CARGO_MANIFEST_DIR").unwrap().as_str()
    ).unwrap();
    let parent_dir = this_dir.parent().unwrap();

    // This compiles the C code in ../src.
    compile_c_code(parent_dir.join("Makefile").as_path());

    // This generates the pmdsky-debug symbols for the linker.
    // Python 3 must be in the PATH.
    generate_symbols_for_linker(parent_dir);

    // This collects the glue code from the `patches!` macro and dumps it into a .cotpatch file
    generate_cotpatch(parent_dir.join("patches/generated_by_rust.cotpatch").as_path());

    // To patch the ROM, use cargo-cot (eg. `cargo cot burn na rom.nds out.nds`).
}
