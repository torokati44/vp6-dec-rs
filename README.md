# vp6-dec-rs

This Rust crate provides a tiny and very specific wrapper for the [VP6](https://en.wikipedia.org/wiki/VP6) decoder in [FFmpeg](https://ffmpeg.org/).

The primary purpose of it is to serve as an optional component of the [Ruffle](https://ruffle.rs/) Flash Player emulator.

This wrapper can utilize FFmpeg in two ways.
At build time, `pkg-config` is used to look for suitable versions of
`libavutil`, `libavcodec`, and `libswscale` installed on the system.
If they are all found, the "Dynamically Linked" scenario takes effect,
otherwise, the "Statically Linked" one - if allowed.

### Dynamically Linked

Works only on desktop, and loads system libraries at runtime.
Only builds a tiny piece of C code as glue.

The license can be MIT.

### Statically Linked

The built library is completely standalone, with the necessary source files compiled in.

It also includes a tiny stub in place of a real libc - just enough to make the decoder compile and work on `wasm32-unknown-unknown` (without [Emscripten](https://emscripten.org/) and the like).

It comes with its own hand-tailored `config.h` to reduce its reliance on any external functions as much as possible.

The license must be LGPL, so this case is only available if the `allow-lgpl`
feature is enabled.
