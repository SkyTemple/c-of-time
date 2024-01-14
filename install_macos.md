# macOS setup guide

You can install c-of-time on macOS using the following methods. The steps detailed in this guide were tested on macOS Sonoma (14.2).

- [Method 1: Homebrew (recommended)](#method-1-homebrew-recommended)
- [Method 2: Downloading the toolchain from ARM](#method-2-downloading-the-toolchain-from-arm)
- [Other methods (advanced)](#other-methods-advanced)

## Method 1: Homebrew (recommended)

[Homebrew](https://brew.sh) is a package manager for macOS that allows you to install the required tools with a few commands. This is the recommended method for installing c-of-time on macOS.

1. Open the Terminal app. You can find it in the Launchpad in the "Other" folder.
2. Install the command line tools for Xcode by running `xcode-select --install` in the terminal. Confirm the installation by clicking "Install" in the popup window.
3. Once the installation is finished, you can proceed with installing Homebrew. Refer to the section "Install Homebrew" of the [Homebrew website](https://brew.sh) for instructions (you will have to execute a command in the terminal). **After the installation is finished, make sure to follow the instructions printed by Homebrew to add the Homebrew directory to your `PATH` environment variable.**
4. Install the ARM toolchain by running `brew install --cask gcc-arm-embedded` in the terminal. You will need to enter your password during the installation.
5. Install Python dependencies: `pip3 install pyyaml ndspy`
6. Navigate to the directory where you want to install c-of-time. You can use `cd` to change the directory and `ls` to list the contents of the current directory. For example, if you want to install c-of-time to `/Users/YourName/Documents/c-of-time`, run `cd /Users/YourName/Documents`.
    - **Note:** You can also use the Finder to navigate to the directory where you want to install c-of-time. Right-click the name of the directory, hold the Option key and select "Copy [directory name] as Pathname". Then, run `cd ` in the terminal and paste the path by pressing Command+V.
5. Download this repository by running `git clone --recursive https://github.com/SkyTemple/c-of-time.git` in the terminal. c-of-time will be downloaded in a folder called `c-of-time` inside the current directory.
6. Enter the `c-of-time` directory with `cd c-of-time`.
7. Copy the ROM you have prepared into the `c-of-time` directory and rename it to `rom.nds`.
    - **US ROM offsets are used by default.** If you're using a EU ROM, change the `REGION` variable in `Makefile` to `EU`.
8. Run `make headers` to add aliases and documentation comments to headers for increased compatibility.
9. Run `make patch` to build the project. The output ROM will be saved as `out.nds` by default.

### Troubleshooting

If you are encountering errors with armips, try the following:
- Navigate to the folder `c-of-time/bin/armips` in Finder
- Right-click `armips-mac-x64`, click "Open" and confirm

If you encounter the error message that says "Bad CPU type in executable" on an Apple Silicon Mac, install Rosetta 2 by running `softwareupdate --install-rosetta` in the terminal.

## Method 2: Downloading the toolchain from ARM

You can install c-of-time on macOS by downloading the toolchain from ARM's website.
This method is slightly more involved, but it's more lightweight and doesn't require you to install Homebrew.

1. Open the Terminal app. You can find it in the Launchpad in the "Other" folder.
2. Install the command line tools for Xcode by running `xcode-select --install` in the terminal. Confirm the installation by clicking "Install" in the popup window.
3. Once the installation is finished, you can proceed with downloading the toolchain. Download the latest version of the toolchain from [ARM's website](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads).
    - If you're using an Apple Silicon Mac, download the "Apple Silicon hosted" version. Otherwise, use the "x86_64 hosted" version.
    - You will need the "AArch32 bare-metal target" version, not the "AArch64 bare-metal target" version.
    - Download and open the `.pkg` file. Follow the instructions in the installer.
4. Run the command `export VERSION=` in the terminal, followed by the version number of the toolchain you just installed. This version number should match the folder name of the toolchain in `/Applications/ArmGNUToolchain`. For example, if you installed version 13.2.rel1, run `export VERSION=13.2.rel1`. Then, run `echo 'export PATH="/Applications/ArmGNUToolchain/$VERSION/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc` to add the toolchain to your `PATH` environment variable.
5. You can now continue with the steps 5-9 of the Homebrew method above.

## Other methods (advanced)

- [devkitpro](https://devkitpro.org/wiki/Getting_Started) provides a macOS installer for a custom toolchain. Follow the instructions on their website to install it and make sure to add the toolchain to your `PATH` environment variable.
