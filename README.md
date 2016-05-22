# Notch [![Build Status](https://travis-ci.org/Reshurum/notch.svg?branch=master)](https://travis-ci.org/Reshurum/notch)

Notch is a CHIP-8 virtual machine written in Rust.

## Dependencies

Notch requires Rust and SDL2 development headers. Rust has only been
tested on Linux and Mac OS X, but should work on Windows as it does not use
any platform specific code.

### Linux

Installing SDL headers under most linux distributions is fairly simple and
available as a single package.

Ubuntu example:
> sudo apt-get install libsdl2-dev

Fedora example:
> sudo dnf install SDL2-devel

Arch Linux example:
> sudo pacman -S sdl2

### Mac OS X

Installing under Mac OS X is a little more involved as the headers won't be
installed under the default path. To install with homebrew run the following:
> brew install sdl2

Put this in your `.bashrc` or other file depending on your shell.
> export LIBRARY_PATH="$LIBRARY_PATH:/usr/local/lib"

### Windows (MinGW)

On Windows, make certain you are using the MinGW version of SDL; the native
version will crash on `sdl2::init`.

1. Download mingw development libraries from
http://www.libsdl.org/ (SDL2-devel-2.0.x-mingw.tar.gz).
2. Unpack to a folder of your choosing (You can delete it afterwards).
3. Copy all lib files from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\lib

    inside
    > C:\Rust\bin\rustlib\x86_64-pc-windows-gnu\lib

    For Multirust Users, this folder will be in
    > C:\Users\{Your Username}\AppData\Local\.multirust\toolchains\{current toolchain}\lib\rustlib\x86_64-pc-windows-gnu\lib

4. Copy SDL2.dll from
    > SDL2-devel-2.0.x-mingw\SDL2-2.0.x\x86_64-w64-mingw32\bin

    into your cargo project, right next to your Cargo.toml.

## Installation

Once SDL is setup, building as simple as invoking cargo:
> cargo build

The final binary should be under `target/debug/notch`. To use notch, pass it
the path to a rom as the argument.
> target/debug/notch <rom file>

## References

* [Mastering CHIP-8](http://mattmik.com/files/chip8/mastering/chip8.html)
* [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

## License

Notch is licensed under the permissive
[BSD 2 Clause Licesne](https://opensource.org/licenses/BSD-2-Clause).
