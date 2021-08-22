The files in this folder are a barebones fraction of a libc.
Just enough present in every header to make the included parts
of FFmpeg compile and work for what we need it. Some functions
are actually just stubs, but those were deemed not necessary.
All of this is only necessary when compiling to WASM, because
on Desktop targets, we rely on the complete libc available on
the system by default. And we're not using Emscripten or other
frameworks to provide a libc for us when running on WASM.