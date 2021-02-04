# vp6-dec-rs

This Rust crate provides a tiny and very specific wrapper for the [VP6](https://en.wikipedia.org/wiki/VP6) decoder in [libav](libav.org).
It is completely standalone, compiling in the necessary source files statically.

It also includes a tiny stub in place of a real libc - just enough to make the decoder compile and work on `wasm32-unknown-unknown` (without [Emscripten](https://emscripten.org/) and the like).

It comes with its own hand-tailored `config.h` to reduce its reliance on any external functions as much as possible.

The primary purpose of this crate is to serve as an optional component of the [Ruffle](https://ruffle.rs/) Flash Player emulator.
