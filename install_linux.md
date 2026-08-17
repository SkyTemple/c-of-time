# Linux setup guide

You can install c-of-time on Linux using the following methods. Instructions vary based on your Linux distribution.

- [Ubuntu/Debian](#ubuntu-debian)
- [Fedora](#fedora)
- [Arch Linux](#arch-linux)
- [Other methods (advanced)](#other-methods-advanced)

## Ubuntu/Debian

You can install c-of-time on Ubuntu or Debian using the following methods. The steps detailed in this guide were tested on Ubuntu 24.04.
> [!WARNING]
>
> Link-time optimization is known to fail on older versions of gcc-arm-none-eabi, such as 10.3.1,
> the last version published on Ubuntu Universe for Ubuntu 22 (Jammy). As such, Ubuntu 24 or later
> is recommended, but if using this version is not possible, link-time optimization can be disabled
> by removing the `-flto` flag from `CFLAGS` in `Makefile` to circumvent the error. 

1. Open the Terminal app in your Applications menu. The exact steps might vary based on your desktop environment.
2. Run the following command to install the required tools: `sudo apt install build-essential cmake git python3-pip gcc-arm-none-eabi binutils-arm-none-eabi`. You will need to enter your password during the installation.
3. Compile `armips`:
    1. Run `git clone --recursive https://github.com/Kingcom/armips.git` to download the source code.
    2. Run `cd armips` to enter the directory.
    3. Run this command to compile the project: `mkdir build && cd build && cmake -DCMAKE_BUILD_TYPE=Release .. && cmake --build .`
    4. Run `sudo cp armips /usr/local/bin` to install `armips`.
    5. Run `cd ../..` to return to the previous directory.
4. Navigate to the directory where you want to install c-of-time. You can use `cd` to change the directory and `ls` to list the contents of the current directory. For example, if you want to install c-of-time to `/home/YourName/Documents/c-of-time`, run `cd /home/YourName/Documents`.
    - **Note:** You can also use the file manager to navigate to the directory where you want to install c-of-time. Right-click the name of the directory and select "Open in Terminal".
5. Download this repository by running `git clone --recursive https://github.com/SkyTemple/c-of-time.git` in the terminal. c-of-time will be downloaded in a folder called `c-of-time` inside the current directory.
6. Enter the `c-of-time` directory with `cd c-of-time`.
7. Copy the ROM you have prepared into the `c-of-time` directory and rename it to `rom.nds`. You can open the file manager in the current directory by running `xdg-open .` in the terminal.
    - **US ROM offsets are used by default.** If you're using a EU or JP ROM, change the `REGION` variable in `Makefile` to `EU` or `JP` accordingly.
8. Create a Python venv and install dependencies: `python3 -m venv .venv && .venv/bin/pip3 install pyyaml ndspy`
9. Activate the Python venv with `source .venv/bin/activate` (use `activate.fish` for fish shell or `activate.csh` for C shell.)
10. Run `make headers` to add aliases and documentation comments to headers for increased compatibility.
11. Run `make patch` to build the project. The output ROM will be saved as `out.nds` by default.

## Fedora

1. Open the Terminal app in your Applications menu. The exact steps might vary based on your desktop environment.
2. Run the following command to install the required tools: `sudo dnf install @development-tools gcc-c++ cmake git python3-pip arm-none-eabi-binutils-cs arm-none-eabi-gcc-cs`. You will need to enter your password during the installation.
3. You can now continue with the steps 3-11 of the Ubuntu/Debian method above.

## Arch Linux

1. Open the Terminal app. The exact steps vary based on your desktop environment.
2. Run the following command to install the required tools: `sudo pacman -Syu base-devel git python python-pip arm-none-eabi-gcc arm-none-eabi-binutils`. You will need to enter your password during the installation.
3. Install the [armips package](https://aur.archlinux.org/packages/armips) via the Arch User Repository (AUR). Please refer to the [Arch Wiki](https://wiki.archlinux.org/title/Arch_User_Repository) for instructions.
4. You can now continue with the steps 4-11 of the Ubuntu/Debian method above.

## Future builds (all distros)

1. In the Terminal app, `cd` to the `c-of-time` directory.
2. Activate the Python venv with `source .venv/bin/activate` (use `activate.fish` for fish shell or `activate.csh` for C shell.)
3. Run `make patch` to rebuild the project.

## Other methods (advanced)

- You can use the official [ARM toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) to build c-of-time. Make sure to add the toolchain to your `PATH` environment variable. You will need to install `make` and other Unix build tools as well.
- [devkitpro](https://devkitpro.org/wiki/Getting_Started) provides an installer for a custom toolchain. Follow the instructions on their website to install it and make sure to add the toolchain to your `PATH` environment variable.
