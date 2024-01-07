## Installing gcc and binutils for `arm-none-eabi`.

`c-of-time` and `Rust of Darkness` require a GCC & Binutils toolchain for `arm-none-eabi`.
You have several ways of installing it.

The following guides for each platform are sorted by what's recommended (based on what we think might be easiest). 
Follow any ONE of these guides.

After reading the guide for your platform, make sure to read the section `Updating PATH`,
since the tools need to be in your platform's `PATH`.

In addition to this guide, you will also need to make sure `make` is installed. Other common Unix build tools may also be needed.

### Windows
- Download it from ARM, see `All Platforms` below.
- Download it as part of devkitpro, see `All Platforms` below.
- Install it using the Windows Subsystem for Linux and following the `Linux` instructions below.
- Install it via Msys2's Mingw environment:
   1. Install [Msys2](https://www.msys2.org/wiki/MSYS2-installation/)
   2. Inside the MingW shell for your architecture (32bit or 64bit) install the package 
      [mingw-w64-arm-none-eabi-gcc](https://packages.msys2.org/base/mingw-w64-arm-none-eabi-gcc) for your MingW version.

### macOS
- Install it via brew: [gcc-arm-embedded cask](https://formulae.brew.sh/cask/gcc-arm-embedded).
- Download it from ARM, see `All Platforms` below.
- Download it as part of devkitpro, see `All Platforms` below.

Please note that this setup is currently untested on ARM64-based Macs.

### Linux
- Install it via your system's package manager:
  - Debian/Ubuntu: `sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi`
  - Arch Linux: `sudo pacman -Syu arm-none-eabi-gcc arm-none-eabi-binutils`
  - Fedora: `sudo dnf install arm-none-eabi-binutils-cs arm-none-eabi-gcc-cs`
- Download it from ARM, see `All Platforms` below.
- Download it as part of devkitpro, see `All Platforms` below.

### All platforms
- [Download it from ARM](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
- [Download devkitpro](https://devkitpro.org/wiki/Getting_Started)

### Updating PATH
You need to make sure the tools you just installed are in your system's PATH:

- If you installed it via brew under macOS, a package manager under Linux or Msys2 under Windows,
  you don't need to do anything except maybe restarting your shell.
- Otherwise you may need to add the directory which contains the
 `arm-none-eabi-gcc[.exe]` and `arm-none-eabi-binutils` binaries (`arm-none-eabi-objdump[.exe]` etc.) to your PATH:
  - Windows: https://www.computerhope.com/issues/ch000549.htm
  - MacOS: https://www.cyberciti.biz/faq/appleosx-bash-unix-change-set-path-environment-variable/
  - Linux: https://opensource.com/article/17/6/set-path-linux

After this you can confirm this worked by trying to run `arm-none-eabi-gcc` from any directory,
if you don't get an error that the command wasn't found, everything is working.
