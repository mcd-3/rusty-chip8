# Rusty CHIP-8

A CHIP-8 Interpreter written in Rust and SDL2.

## Building

To build this project, you will first need SDL2 on your machine.
To get SDL2, [follow the instructions here](https://crates.io/crates/sdl2#requirements).
Make sure to follow the correct instructions for the operating system you are using.

Once SDL2 has been set up on your machine and set up for this project, simply run `cargo run` to build and start the project.

## Bundling

If you'd like to bundle this program, you'll first need to install the `cargo-bundle` crate globally.
[You can download the cargo-bundle crate here.](https://crates.io/crates/cargo-bundle)

### MacOs
1. Create a `lib` directory at the root of the project
2. Download the MacOS version of the SDL2 framework manually
    1. It should be named `SDL2.framework` when downloaded
3. Place the `SDL2.framework` directory in the `lib` directory you created
4. Run `cargo bundle --release` (or `cargo bundle` if you want a debug build)

## Resources
These resources were invaluable to me while developing this interpreter.

- [Cowgod's "Chip-8 Technical Reference"](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Tobias V. Langhoff's "Guide to making a CHIP-8 emulator"](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
- [aquova's "An Introduction to Chip-8 Emulation using the Rust Programming
Language"](https://aquova.net/chip8/chip8.pdf)
- [starrhorne's Chip-8 Emulator](https://github.com/starrhorne/chip8-rust)
- [Timendus's Chip-8 Test Suite](https://github.com/Timendus/chip8-test-suite)

