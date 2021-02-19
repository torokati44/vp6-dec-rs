# vp6-dec-rs

This Rust crate provides a tiny and very specific wrapper for the [VP6](https://en.wikipedia.org/wiki/VP6) decoder in [FFmpeg](https://ffmpeg.org/).

The primary purpose of it is to serve as an optional component of the [Ruffle](https://ruffle.rs/) Flash Player emulator.


## Two ways of using FFmpeg:

### Statically:
It is completely standalone, compiling in the necessary source files statically.

It also includes a tiny stub in place of a real libc - just enough to make the decoder compile and work on `wasm32-unknown-unknown` (without [Emscripten](https://emscripten.org/) and the like).

It comes with its own hand-tailored `config.h` to reduce its reliance on any external functions as much as possible.

### Dynamically:

Only on desktop, loads system libs. Uses pkg-config, only builds a tiny glue code, allows different licencing.