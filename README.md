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
To build the project, run `make patch`. This command will build your code, inject it into an overlay in the provided ROM and apply the patches in the `patches` directory. The output ROM will be saved as `out.nds` by default.

If you want to check the generated assembly, run `make asmdump`. A file `out.asm` will be generated, which contains an assembly listing annotated with the corresponding source code lines.

## Usage
Patches can be added to `.cotpatch` files inside the `patches` directory. These patch files contain offsets into functions that should be patched and assembly instructions, which allow calling into custom code. See `src/main.c` and `patches/patches.cotpatch` for examples.

### Logging
You can use the logging macros `COT_LOG`, `COT_WARN` and `COT_ERROR`. To view the logs, open the ROM in the SkyTemple debugger and check "Game Internal" in the log window.

To disable logging globally and save some performance, change `RELEASE_CONFIG` in `Makefile`.

### Custom special processes
To create custom special processes, add them into the `switch` statement in `CustomScriptSpecialProcessCall`. This function is only called for special process ID 100 and greater for compatibility with existing patches.

## Updating symbol definitions and headers
To update symbol data from `pmdsky-debug`, run `git submodule foreach git pull origin master`,
then clean the build with `make clean`.

## Adding custom symbols
If you've found symbols that are currently missing, consider contributing them to [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug). You can find instructions in the repository's [contribution docs](https://github.com/UsernameFodder/pmdsky-debug/blob/master/docs/contributing.md).

For quick testing, you can also add symbols to `symbols/custom_[region].ld` (`symbols/generated_[region].ld` is auto-generated and should not be modified). You need to specify the file each symbol belongs to in comments:

```
/* !file arm9 */
MyCoolFunction = 0x200DABC;

/* !file overlay29 */
SomeDungeonFunction = 0x22DEABC;
SomeOtherDungeonFunction = 0x22DEEFF;
```

## Code size constraints

The built code gets injected into the custom overlay 36. The entire overlay is 228 KB big, most of which is reserved for common patches provided by SkyTemple. Your code will be placed in the last 32 KB, which are considered the "common area" . If the binary is larger than 32 KB, you will get the following linker error: 
```
error "section '.text' will not fit in region 'out'"
```

### Expanding the available space
To work around this issue, you can extend the space allocated in the overlay. **If you decide to extend the space, you do so at your own risk. Be careful since this space might be used by future patches!** Check the [list of assigned areas](https://docs.google.com/document/d/1Rs4icdYtiM6KYnWxMkdlw7jpWrH7qw5v6LOfDWIiYho) to find out if patches used in your ROM are affected.

To extend the allocated space, open `linker.ld` and edit the following line:
```
out     : ORIGIN = 0x23D7FF0, LENGTH = 0x8010
```

Subtract the amount of additional bytes you want to allocate from `ORIGIN` and add them to `LENGTH`. Next, open `patches/patch.py` and set `START_ADDRESS` of the top of the file to the same value as `ORIGIN` in the linker script.

### Optimizing for size
You can also change the compiler flags to optimize for size instead of speed. To do so, set `OPT_LEVEL := Os` in `Makefile`. Effectiveness varies per project.

## License
This repository is licensed under GPLv3. Review the file `LICENSE` for more information. 
