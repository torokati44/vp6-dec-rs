extern crate cc;
extern crate pkg_config;

fn main() {
    // Minimum versions according to:
    // https://github.com/ruffle-rs/ruffle/pull/3004#issuecomment-781735152
    const LIBAVUTIL_MIN: &str = "55.0.100";
    const LIBAVCODEC_MIN: &str = "57.0.100";

    let avutil = pkg_config::Config::new()
        .atleast_version(LIBAVUTIL_MIN)
        .probe("libavutil");
    let avcodec = pkg_config::Config::new()
        .atleast_version(LIBAVCODEC_MIN)
        .probe("libavcodec");

    let mut build = cc::Build::new();

    match (avutil, avcodec) {
        (Ok(avutil), Ok(avcodec)) => {
            // The happy case; All three necessary FFmpeg libraries were found on
            // the system, so the only thing we have to build is a bit of glue.
            build
                .includes(avutil.include_paths)
                .includes(avcodec.include_paths);
            // linking is already taken care of by `pkg_config`
        }
        _ => {
            // Resorting to building the VP6 decoder statically,
            // from the FFmpeg sources in the git submodule.
            if !cfg!(feature = "allow-lgpl") {
                panic!(concat!("Required libraries could not be found, and compiling in LGPL code was not allowed.\n",
                    "Either install libavutil >={:} and libavcodec >={:};\n",
                    "Or enable the feature `allow-lgpl`."), LIBAVUTIL_MIN, LIBAVCODEC_MIN);
            }

            let target = std::env::var("TARGET").unwrap();
            if target == "wasm32-unknown-unknown" {
                // relying on our fake libc fragment
                build
                    .define("MALLOC_PREFIX", "vp6_custom_")
                    .flag("-isystem")
                    .flag("src/fakelibc")
                    .file("src/fakelibc/impl.c")
                    .file("src/fakelibc/qsort.c")
                    .define("HAVE_ISINF", "0")
                    .define("HAVE_ISNAN", "0")
                    .define("HAVE_LLRINT", "0")
                    .define("HAVE_LLRINTF", "0")
                    .define("HAVE_LRINT", "0")
                    .define("HAVE_LRINTF", "0")
                    .define("HAVE_RINT", "0")
                    .define("HAVE_STRERROR_R", "1");
            } else {
                // mostly relying on the system libc
                build
                    .define("HAVE_ISINF", "1")
                    .define("HAVE_ISNAN", "1")
                    .define("HAVE_ISFINITE", "1")
                    .define("HAVE_HYPOT", "1")
                    .define("HAVE_ERF", "1")
                    .define("HAVE_COPYSIGN", "1")
                    .define("HAVE_CBRT", "1")
                    .define("HAVE_LLRINT", "1")
                    .define("HAVE_LLRINTF", "1")
                    .define("HAVE_LRINT", "1")
                    .define("HAVE_LRINTF", "1")
                    .define("HAVE_RINT", "1");

                if target == "x86_64-pc-windows-msvc" {
                    build
                        .define("HAVE_LIBC_MSVCRT", "1")
                        .define("HAVE_UNISTD_H", "0")
                        .define("HAVE_STRERROR_R", "0");
                } else {
                    build
                        .define("HAVE_LIBC_MSVCRT", "0")
                        .define("HAVE_UNISTD_H", "1")
                        .define("HAVE_STRERROR_R", "1");
                }
            }

            build
                .compiler("clang")
                .define("HAVE_AV_CONFIG_H", None)
                .includes(&["extern/config", "extern/ffmpeg/"])
                .warnings(false)
                .extra_warnings(false)
                .flag("-Wno-switch")
                .flag("-Wno-attributes")
                .flag("-Wno-ignored-qualifiers")
                .flag("-Wno-deprecated-declarations")
                .flag("-Wno-parentheses")
                .flag("-Wno-pointer-sign")
                .flag("-Wno-implicit-int-float-conversion");

            let mut avcodec = build.clone();
            avcodec
                .files(&[
                    "extern/ffmpeg/libavcodec/allcodecs.c",
                    "extern/ffmpeg/libavcodec/avpacket.c",
                    "extern/ffmpeg/libavcodec/bitstream_filters.c",
                    "extern/ffmpeg/libavcodec/bitstream.c",
                    "extern/ffmpeg/libavcodec/bsf.c",
                    "extern/ffmpeg/libavcodec/codec_desc.c",
                    "extern/ffmpeg/libavcodec/encode.c",
                    "extern/ffmpeg/libavcodec/decode.c",
                    "extern/ffmpeg/libavcodec/avcodec.c",
                    "extern/ffmpeg/libavcodec/codec_par.c",
                    "extern/ffmpeg/libavcodec/h264chroma.c",
                    "extern/ffmpeg/libavcodec/hpeldsp.c",
                    "extern/ffmpeg/libavcodec/huffman.c",
                    "extern/ffmpeg/libavcodec/mathtables.c",
                    "extern/ffmpeg/libavcodec/me_cmp.c",
                    "extern/ffmpeg/libavcodec/null_bsf.c",
                    "extern/ffmpeg/libavcodec/options.c",
                    "extern/ffmpeg/libavcodec/profiles.c",
                    "extern/ffmpeg/libavcodec/simple_idct.c",
                    "extern/ffmpeg/libavcodec/utils.c",
                    "extern/ffmpeg/libavcodec/videodsp.c",
                    "extern/ffmpeg/libavcodec/vp3dsp.c",
                    "extern/ffmpeg/libavcodec/vp56.c",
                    "extern/ffmpeg/libavcodec/vp56data.c",
                    "extern/ffmpeg/libavcodec/vp56dsp.c",
                    "extern/ffmpeg/libavcodec/vp56rac.c",
                    "extern/ffmpeg/libavcodec/vp6.c",
                    "extern/ffmpeg/libavcodec/vp6dsp.c",
                ])
                .compile("avcodec");

            let mut avutil = build.clone();
            avutil
                .files(&[
                    "extern/ffmpeg/libavutil/avstring.c",
                    "extern/ffmpeg/libavutil/bprint.c",
                    "extern/ffmpeg/libavutil/buffer.c",
                    "extern/ffmpeg/libavutil/channel_layout.c",
                    "extern/ffmpeg/libavutil/cpu.c",
                    "extern/ffmpeg/libavutil/dict.c",
                    "extern/ffmpeg/libavutil/error.c",
                    "extern/ffmpeg/libavutil/utils.c",
                    "extern/ffmpeg/libavutil/eval.c",
                    "extern/ffmpeg/libavutil/fifo.c",
                    "extern/ffmpeg/libavutil/frame.c",
                    "extern/ffmpeg/libavutil/hwcontext.c",
                    "extern/ffmpeg/libavutil/imgutils.c",
                    "extern/ffmpeg/libavutil/intmath.c",
                    "extern/ffmpeg/libavutil/log.c",
                    "extern/ffmpeg/libavutil/log2_tab.c",
                    "extern/ffmpeg/libavutil/mathematics.c",
                    "extern/ffmpeg/libavutil/mem.c",
                    "extern/ffmpeg/libavutil/opt.c",
                    "extern/ffmpeg/libavutil/parseutils.c",
                    "extern/ffmpeg/libavutil/pixdesc.c",
                    "extern/ffmpeg/libavutil/random_seed.c",
                    "extern/ffmpeg/libavutil/rational.c",
                    "extern/ffmpeg/libavutil/reverse.c",
                    "extern/ffmpeg/libavutil/samplefmt.c",
                    "extern/ffmpeg/libavutil/sha.c",
                    "extern/ffmpeg/libavutil/time.c",
                ])
                .compile("avutil");
        }
    }

    if cfg!(feature = "with-swscale") {
        const LIBSWSCALE_MIN: &str = "4.1.100";
        let swscale = pkg_config::Config::new()
            .atleast_version(LIBSWSCALE_MIN)
            .probe("libswscale");

        match swscale {
            Ok(swscale) => {
                // The happy case; All three necessary FFmpeg libraries were found on
                // the system, so the only thing we have to build is a bit of glue.
                build.includes(swscale.include_paths);
                // linking is already taken care of by `pkg_config`
            }
            _ => {
                // Resorting to building the VP6 decoder statically,
                // from the FFmpeg sources in the git submodule.
                if !cfg!(feature = "allow-lgpl") {
                    panic!(concat!("Required libraries could not be found, and compiling in LGPL code was not allowed.\n",
                        "Either install libswscale >={:};\n",
                        "Or enable the feature `allow-lgpl`."), LIBSWSCALE_MIN);
                }

                let mut swscale = build.clone();
                swscale
                    .files(&[
                        "extern/ffmpeg/libswscale/alphablend.c",
                        "extern/ffmpeg/libswscale/gamma.c",
                        "extern/ffmpeg/libswscale/hscale_fast_bilinear.c",
                        "extern/ffmpeg/libswscale/hscale.c",
                        "extern/ffmpeg/libswscale/input.c",
                        "extern/ffmpeg/libswscale/options.c",
                        "extern/ffmpeg/libswscale/output.c",
                        "extern/ffmpeg/libswscale/rgb2rgb.c",
                        "extern/ffmpeg/libswscale/slice.c",
                        "extern/ffmpeg/libswscale/swscale_unscaled.c",
                        "extern/ffmpeg/libswscale/swscale.c",
                        "extern/ffmpeg/libswscale/utils.c",
                        "extern/ffmpeg/libswscale/vscale.c",
                        "extern/ffmpeg/libswscale/yuv2rgb.c",
                    ])
                    .compile("swscale")
            }
        }
    }

    build.file("src/helpers.c").compile("helpers");
}
