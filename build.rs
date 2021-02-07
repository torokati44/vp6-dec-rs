extern crate cc;

fn main() {
    let mut build = cc::Build::new();

    build.files(&[
        "extern/libav/libavcodec/avpacket.c",
        "extern/libav/libavcodec/bitstream_filters.c",
        "extern/libav/libavcodec/bitstream.c",
        "extern/libav/libavcodec/bsf.c",
        "extern/libav/libavcodec/codec_desc.c",
        "extern/libav/libavcodec/decode.c",
        "extern/libav/libavcodec/h264chroma.c",
        "extern/libav/libavcodec/hpeldsp.c",
        "extern/libav/libavcodec/huffman.c",
        "extern/libav/libavcodec/mathtables.c",
        "extern/libav/libavcodec/me_cmp.c",
        "extern/libav/libavcodec/null_bsf.c",
        "extern/libav/libavcodec/options.c",
        "extern/libav/libavcodec/profiles.c",
        "extern/libav/libavcodec/simple_idct.c",
        "extern/libav/libavcodec/utils.c",
        "extern/libav/libavcodec/videodsp.c",
        "extern/libav/libavcodec/vp3dsp.c",
        "extern/libav/libavcodec/vp56.c",
        "extern/libav/libavcodec/vp56data.c",
        "extern/libav/libavcodec/vp56dsp.c",
        "extern/libav/libavcodec/vp56rac.c",
        "extern/libav/libavcodec/vp6.c",
        "extern/libav/libavcodec/vp6dsp.c",
        "extern/libav/libavutil/avstring.c",
        "extern/libav/libavutil/buffer.c",
        "extern/libav/libavutil/channel_layout.c",
        "extern/libav/libavutil/cpu.c",
        "extern/libav/libavutil/dict.c",
        "extern/libav/libavutil/eval.c",
        "extern/libav/libavutil/frame.c",
        "extern/libav/libavutil/hwcontext.c",
        "extern/libav/libavutil/imgutils.c",
        "extern/libav/libavutil/intmath.c",
        "extern/libav/libavutil/log.c",
        "extern/libav/libavutil/log2_tab.c",
        "extern/libav/libavutil/mathematics.c",
        "extern/libav/libavutil/mem.c",
        "extern/libav/libavutil/opt.c",
        "extern/libav/libavutil/pixdesc.c",
        "extern/libav/libavutil/rational.c",
        "extern/libav/libavutil/samplefmt.c",
        "extern/libav/libswscale/input.c",
        "extern/libav/libswscale/options.c",
        "extern/libav/libswscale/output.c",
        "extern/libav/libswscale/rgb2rgb.c",
        "extern/libav/libswscale/swscale_unscaled.c",
        "extern/libav/libswscale/swscale.c",
        "extern/libav/libswscale/utils.c",
        "extern/libav/libswscale/yuv2rgb.c",
        "src/helpers.c",
    ]);

    let target = std::env::var("TARGET").unwrap();
    if target == "wasm32-unknown-unknown" || target == "x86_64-pc-windows-msvc" {
        // relying on our fake libc fragment
        build
            .define("MALLOC_PREFIX", "vp6_custom_")
            .flag("-isystem")
            .flag("src/fakelibc")
            .file("src/fakelibc/impl.c")
            .file("src/fakelibc/qsort.c")
            .define("HAVE_ISINF", "0")
            .define("HAVE_ISNAN", "0");
    } else {
        // mostly relying on the system libc
        build.define("HAVE_ISINF", "1").define("HAVE_ISNAN", "1");
    }

    build
        .compiler("clang")
        .define("HAVE_AV_CONFIG_H", None)
        .includes(&[
            "extern/config",
            "extern/libav/",
            "extern/libav/libavutil",
            "extern/libav/libavcodec",
        ])
        .warnings(false)
        .extra_warnings(false)
        .flag("-Wno-attributes")
        .flag("-Wno-ignored-qualifiers")
        .flag("-Wno-deprecated-declarations")
        .flag("-Wno-switch")
        .flag("-Wno-parentheses")
        .flag("-Wno-implicit-const-int-float-conversion")
        .compile("vp6");
}
