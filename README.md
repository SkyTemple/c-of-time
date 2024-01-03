# c-of-time

![c-of-time logo by Irdkwia](./cot-logo.png)
*Logo by [Irdkwia](https://github.com/irdkwia)*

An environment for hooking and linking to Pokémon Mystery Dungeon: Explorers of Sky.

## Credits
This project is loosely based on [EternalCode's template](https://github.com/EternalCode/Empty-Template). The build configuration is based on scripts provided by [devkitPro](https://devkitpro.org). The patch format was inspired by [Starlight](https://github.com/shadowninja108/Starlight).

Special thanks to [UsernameFodder](https://github.com/UsernameFodder) for the [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug) project, [End45](https://github.com/End45) for the *ExtraSpace* patch and irdkwia for their research on item, move and special process effects.

## Rust subsystem
**NOTE: The `main` branch does currently not contain the Rust subsystem anymore**, as it's support
for symbols for `pmdsky-debug` is outdated and we eventually want to split the Rust subsytem
off so we can keep `c-of-time` up-to-date with `pmdsky-debug` more easily. Use the `rust` branch
if you want to use the Rust subsystem.

c-of-time can also be used with Rust projects. If you want to use Rust (including mixed Rust + C projects),
continue reading the `README.md` in the `rust` directory.

If you want to build pure C projects, continue below.

## Project setup
1. Install [Python](https://www.python.org/downloads/).
2. Install GCC and Binutils for `arm-none-eabi`. See [install_gcc.md](install_gcc.md) for information on how to install it.
3. Clone this repository *recursively* with `git clone --recursive https://github.com/tech-ticks/c-of-time.git`. Make sure that you enter the correct directory before continuing (e.g. `cd c-of-time`).
  - If you don't have Git installed, click the green "Code" button on GitHub to download this repository, then do the same for [pmdsky-debug](https://github.com/UsernameFodder/pmdsky-debug). Finally, extract both .zip files and copy the `pmdsky-debug` folder into the `c-of-time` folder.
4. Install Python dependencies: `pip3 install pyyaml ndspy`
5. Patch a Pokémon Mystery Dungeon: Explorers of Sky ROM with the [`ExtraSpace` patch by End45](https://github.com/End45/EoS-asm-hacks/blob/main/src/ExtraSpace.asm). You can apply the patch with [SkyTemple](https://skytemple.org):
    1. Open the ROM in SkyTemple
    2. Click *ASM Patches* (*Patches > ASM* in SkyTemple 1.4+) and switch to the *Utility* tab
    3. Select the *ExtraSpace* patch and click *Apply*
6. Place the ROM in `[project root]/rom.nds`
    - **US ROM offsets are used by default.** If you're using a EU ROM, change the `REGION` variable in `Makefile` to `EU`.
7. Follow these steps depending on your operating system:
    - If you are using Linux, install [armips](https://github.com/Kingcom/armips) manually.
    - If you are encountering errors with armips on Windows, you might need to install the [Visual C++ Redistributable for Visual Studio 2015](https://www.microsoft.com/en-US/download/details.aspx?id=48145).
    - On macOS, you might need to do the following:
      - Navigate to the folder `bin/armips` in Finder
      - Right-click `armips-mac-x64`, click "Open" and confirm
8. (optional) Run `make header-comments` to generate documentation comments for IDEs.

## Building
To build the project, run `make patch`. This command will build your code, inject it into an overlay in the provided ROM and apply the patches in the `patches` directory. The output ROM will be saved as `out.nds` by default.

If you want to check the generated assembly, run `make asmdump`. A file `out.asm` will be generated, which contains an assembly listing annotated with the corresponding source code lines.

## Usage
Patches can be added to `.asm` files inside the `patches` directory. These patch files contain offsets into functions that should be patched and assembly instructions, which allow calling into custom code. See `src/main.c` and `patches/patches.asm` for examples.

### Logging and assertions
You can use the logging macros `COT_LOG`, `COT_WARN` and `COT_ERROR`. To view the logs, open the ROM in the SkyTemple debugger and check "Game Internal" in the log window. A macro for assertions `COT_ASSERT(expr)` is also available.

To disable assertions and logging globally and save some performance, change `RELEASE_CONFIG` in `Makefile`.

### Custom move/item effects and special processes
To create custom special processes, add them into the `switch` statement in `CustomScriptSpecialProcessCall`. This function is only called for special process ID 100 and greater for compatibility with existing patches.

You can add custom item or move effects in `CustomApplyItemEffect` and `CustomApplyMoveEffect`.

#### Compatiblity with existing patches
This project aims to keep compatibility with existing patches for move, item and special process effects to some degree. Special process effects using the `ExtractSpCode` patch can be reused without problems if they were imported with an ID lower than 100. Compatiblity with the `ExtractMoveCode` has not been thoroughly tested yet and might potentially cause issues with the *Metronome* move.

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

The value of `ORIGIN` must a multiple of 16 (end with 0 in hexadecimal). Therefore, the amount of bytes added to `LENGTH` must also be a multiple of 16.

To extend the allocated space, open `linker.ld` and edit the following line:
```
out     : ORIGIN = 0x23D7FF0, LENGTH = 0x8010
```

Subtract the amount of additional bytes you want to allocate from `ORIGIN` and add them to `LENGTH`. Next, open `patches/patch.py` and set `START_ADDRESS` of the top of the file to the same value as `ORIGIN` in the linker script.

### Optimizing for size
You can also change the compiler flags to optimize for size instead of speed. To do so, set `OPT_LEVEL := Os` in `Makefile`. Effectiveness varies per project.

## Licensing
- Build scripts (everything under the `tools`) are licensed under GPLv3. Review the file `LICENSE_GPLv3` for more information.
- All other code is licensed under MIT. Review the file `LICENSE_MIT` for more information.
