# c-of-time

An environment for hooking and linking to Pokémon Mystery Dungeon: Explorers of Sky.

## Credits
This project is loosely based on [EternalCode's template](https://github.com/EternalCode/Empty-Template). The build configuration is based on scripts provided by [devkitPro](https://devkitpro.org). The patch format was inspired by [Starlight](https://github.com/shadowninja108/Starlight).

Special thanks to [UsernameFodder](https://github.com/UsernameFodder) for the [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug) project and [End45](https://github.com/End45) for the *ExtraSpace* patch.

## Project setup
1. Install [Python](https://www.python.org/downloads/) and [devkitpro](https://devkitpro.org/wiki/Getting_Started).
    - If you're using Windows, follow the next steps within MSYS (refer to the installation guide for instructions on how to launch it)
    - On Unix platforms, you might need to relaunch your terminal after the installation
1. After you've followed the devkitpro installation guide, add the Nintendo DS modules with `sudo dkp-pacman -S nds-dev`.
1. Clone this repository *recursively* with `git clone --recursive https://github.com/tech-ticks/c-of-time.git`. Make sure that you enter the correct directory before continuing (e.g. `cd c-of-time`).
1. Install Python dependencies: `pip3 install pyyaml keystone-engine ndspy`
1. Patch a Pokémon Mystery Dungeon: Explorers of Sky ROM with the [`ExtraSpace` patch by End45](https://github.com/End45/EoS-asm-hacks/blob/main/src/ExtraSpace.asm). You can apply the patch with [SkyTemple](https://skytemple.org):
    1. Open the ROM in SkyTemple
    1. Click *ASM Patches* and switch to the *Utility* tab
    1. Select the *ExtraSpace* patch and click *Apply*
1. Place the ROM in `[project root]/rom.nds`
    - **US ROM offsets are used by default.** If you're using a EU ROM, change the `REGION` variable in `Makefile` to `EU`.

## Building
To build the project, run `make patch`. This command will build a custom overlay, inject it into the provided ROM and apply the patches in the `patches` directory. The output ROM will be saved as `out.nds` by default.

If you want to check the generated assembly, run `make asmdump`. A file `overlay_0036.asm` will be generated, which contains an assembly listing annotated with the corresponding source code lines.

## Usage
Patches can be added to `.cotpatch` files inside the `patches` directory. These patch files contain offsets into functions that should be patched and assembly instructions, which allow calling into custom code. See `src/main.c` and `patches/patches.cotpatch` for examples.

## Updating symbol definitions and headers
To update symbol data from `pmdsky-debug`, run `git submodule foreach git pull origin master`,
then clean the build with `make clean`.

## Adding custom symbols
If you've found symbols that are currently missing, consider contributing them to [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug). You can find instructions in the repository's [contribution docs](https://github.com/UsernameFodder/pmdsky-debug/blob/master/docs/contributing.md).

For quick testing, you can also add symbols to `symbols_custom.ld` (`symbols.ld` is auto-generated and should not be modified). You need to specify the file each symbol belongs to in comments:

```
/* !file arm9 */
MyCoolFunction = 0x200DABC;

/* !file overlay29 */
SomeDungeonFunction = 0x22DEABC;
SomeOtherDungeonFunction = 0x22DEEFF;
```

## License
This repository is licensed under GPLv3. Review the file `LICENSE` for more information. 
