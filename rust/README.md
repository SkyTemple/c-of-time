# c-of-time Rust subsystem

An environment for hooking and linking to Pokémon Mystery Dungeon: Explorers of Sky: Rust subsystem.

The Rust subsystem extends c-of-time with the ability to compile and link Rust code into
the game.

Everything you need to get started is in this repository.

## Project setup
1. Install a Rust nightly toolchain, preferably with [Rustup](https://rustup.rs/).
2. Install [Python](https://www.python.org/downloads/) and [devkitpro](https://devkitpro.org/wiki/Getting_Started).
    - If you're using Windows, follow the next steps within MSYS (refer to the installation guide for instructions on 
      how to launch it)
    - On Unix platforms, you might need to relaunch your terminal after the installation
3. After you've followed the devkitpro installation guide, add the Nintendo DS modules with `sudo dkp-pacman -S nds-dev`.
4. Clone this repository *recursively* with `git clone --recursive https://github.com/tech-ticks/c-of-time.git`. 
   Make sure that you enter the correct directory before continuing (e.g. `cd c-of-time/rust`).
5. Patch a Pokémon Mystery Dungeon: Explorers of Sky ROM with the
   [`ExtraSpace` patch by End45](https://github.com/End45/EoS-asm-hacks/blob/main/src/ExtraSpace.asm). You can apply the patch with [SkyTemple](https://skytemple.org):
    1. Open the ROM in SkyTemple
    2. Click *ASM Patches* and switch to the *Utility* tab
    3. Select the *ExtraSpace* patch and click *Apply*
6. Install the `cargo-cot` Cargo extension, by running `cargo install --path ./cargo-cot`.

## Building
To build the project, run `cargo cot build <region>`, where `<region>` is either `na`, `eu` or `jp`, matching your 
ROM's region.

To write the project to a ROM, run `cargo cot burn <region> <input_rom_path> <output_rom_path>`. Make sure the ROM
at `<input_rom_path>` is patched with the `ExtraSpace` patch. This command will also run `cargo cot build` before.

It is recommended for the final build of your ROM to use `release` mode. Pass `--release` to the `build` and `burn`
commands to do so.

Example: 
```bash
cargo cot burn na --release  input.nds output.nds
```

After running this command, the ROM at `input.nds` will be copied to `output.nds`, and `c-of-time` will be written
into overlay 36. This includes both the Rust code in `./src` and the C code in `../src`. 
`cotpatch` patches in `../patches` are also applied (see below).

## Documentation
Most of the logic of the Rust subsystem is in the `eos-rs` crate. The documentation for this crate is available
at <https://eosrs.pmdcollab.org/>.

Broadly speaking this crate provides two APIs:

- The "high-level" API: This is a "handwritten" Rust-idiomatic way of interacting with the game's functions and data.
  It is generally recommended to use this API.
- The "low-level API": Everything in the `eos_rs::ffi` crate. This will give you raw access to functions and symbols
  of the game.

## Stability
This entire subsystem is highly unstable, every update may entirely change the structure of the code. This is due
to the fact that we are still very much in the process of reverse-engineering the game's code.

Also note that a lot of code in the subsystem isn't actually fully tested yet. If you run into bugs, please report
an issue. If it's simple to fix, consider fixing it and opening a Pull Request.

## Usage
To get started, read the instructions and explanations in `src/main.rs`. This file will also contain a `patches!` macro
invocation. 

One thing to note about this is, that the last entry of this macro contains the string:
```
HasLowHealth+0:
  B has_high_health
```

This text will be written into the file `../patches/generated_by_rust.cotpatch` when you run `cargo cot burn` and 
will be burned into the game along with other `.cotpatch` files in that directory. These patch files contain the glue code that
makes existing code in the game jump to your patches.

This glue code is not neccesary for item or move effects or special processes as there is glue code for those
automatically generated, see the `main.rs` file and the documentation of the 
[`patches!` macro](https://eosrs.pmdcollab.org/armv5te-none-ndseoseabi-na/doc/eos_rs/macro.patches.html) for more 
information.

### Interop with C patches
This runtime will compile both the Rust code in  `./src` and the C code in `../src` into the game. They are linked
together and as such you can call Rust functions from C and vice-versa.

### Logging and assertions
You can use the normal Rust `assert!` etc. macros for assertions and for logging `eos_rs::prelude` exports the
common `info!`, `warn!` and `error!` macros of the `log` crate. Those are written to the game's debug log, which you
can, for example, view in the SkyTemple Script Engine Debugger.

### Rust and Cargo ecosystem
You have access to all the usual Rust / Cargo features. We encourage you to share code for patches as standalone crates,
that other people can use!

Not that the entire subsystem does not have `std` available.

### Removing example code
This repository contains a lot of example code. To remove it and getting started with your own patches:

- Remove all the example code from `src/main.rs`. At the bare minimum, you will need the crate attributes 
  (`#![no_std]`, etc.) and the `patches!` macro, where you can insert your own patch definitions.
- Remove the `../patches/patch.cotpatch` file. This is an example C patch glue code.
- Remove the contents of `../src/main.c`. This is an example C patch.
- From `../src/item_effects.c`, remove the `ItemElixir` function. This is an example item effect. Also remove it
  from the switch case in `CustomApplyItemEffect`. Leave everything else as it is.
- From `../src/move_effects.c`, remove the `MoveBodyPress` function. This is an example move effect. Also remove it
  from the switch case in `CustomApplyMoveEffect`. Leave everything else as it is.
- From `../src/move_effects.c`, remove the `SpChangeBorderColor` function. This is an example special process. 
  Also remove it from the switch case in `CustomScriptSpecialProcessCall`. Leave everything else as it is.


## Cleaning up
Sometimes it might be neccesary to clean build artifacts. For this you have two options:

1. Run `make clean` in the parent directory and `cargo clean` in this directory, or
2. clear the repository with Git (eg. `git clean -xdf`).

## Updating symbol definitions and headers
To update symbol data from `pmdsky-debug`, run `git submodule foreach git pull origin master`,
then clean the build with `make clean` in the parent directory and `cargo clean` in this directory.

Note that the `eos-rs` crate will also need to be updated. You can wait for an update to the "high-level API", or
simply regenerate the `eos_rs::ffi` module (which contains the "low-level" API / the raw C functions) by running
`./generate-bindings.sh` in the `eos-rs` directory. Note that this will only make new symbols available if they
were also added to the C header files in `pmdsky-debug`. If they weren't you can also just write your own `extern "C"`
blocks for custom/new symbols.

## Adding custom symbols
If you've found symbols that are currently missing, consider contributing them to 
[pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug). 
You can find instructions in the repository's 
[contribution docs](https://github.com/UsernameFodder/pmdsky-debug/blob/master/docs/contributing.md).

For quick testing, you can also add symbols to `../symbols/custom_[region].ld`, see the README.md in the parent 
directory for more information and the note above at `Updating symbol definitions and headers`.

## Code size constraints

The built code gets injected into the custom overlay 36. The entire overlay is 228 KB big, most of which is reserved 
for common patches provided by SkyTemple. Your code will be placed in the last 32 KB, which are considered the 
"common area".

To extend the space used by this patch, see the parent README.md, section "Code size constraints".


## License
The Rust subsystem is licensed under the MIT license, see the `LICENSE_MIT` file in the root directory for more information.

See the README.md in the root directory for more other licenses that apply to other parts of this repository.