extern crate cc;
extern crate pkg_config;
use autotools;
use autotools::Config;
use std::env;

fn main() {
    // Minimum versions according to:
    // https://github.com/ruffle-rs/ruffle/pull/3004#issuecomment-781735152
    const LIBAVUTIL_MIN: &str = "575.0.100";
    const LIBAVCODEC_MIN: &str = "572.0.100";
    const LIBSWSCALE_MIN: &str = "47.1.100";

    let avutil = pkg_config::Config::new()
        .atleast_version(LIBAVUTIL_MIN)
        .probe("libavutil");
    let avcodec = pkg_config::Config::new()
        .atleast_version(LIBAVCODEC_MIN)
        .probe("libavcodec");
    let swscale = pkg_config::Config::new()
        .atleast_version(LIBSWSCALE_MIN)
        .probe("libswscale");

    let mut build = cc::Build::new();

    match (avutil, avcodec, swscale) {
        (Ok(avutil), Ok(avcodec), Ok(swscale)) => {
            // The happy case; All three necessary FFmpeg libraries were found on
            // the system, so the only thing we have to build is a bit of glue.
            build
                .includes(avutil.include_paths)
                .includes(avcodec.include_paths)
                .includes(swscale.include_paths);
            // linking is already taken care of by `pkg_config`
        }
        _ => {


            let dst = Config::new("extern/ffmpeg")
            .enable_static()
            .disable_shared()
            .disable("everything", None)
                //.disable("autodetect", None)
            .disable("swresample", None)
            .disable("avformat", None)
            .disable("avdevice", None)
            .disable("avfilter", None)
            .enable("decoder", Some("vp6f,vp6a"))
            .disable("iconv", None)
            .disable("doc", None)
            //  --cc="clang"
            .build();


            // Resorting to building the VP6 decoder statically,
            // from the FFmpeg sources in the git submodule.
            if !cfg!(feature = "allow-lgpl") {
                panic!(concat!("Required libraries could not be found, and compiling in LGPL code was not allowed.\n",
                    "Either install libavutil >={:}, libavcodec >={:}, and libswscale >={:};\n",
                    "Or enable the feature `allow-lgpl`."), LIBAVUTIL_MIN, LIBAVCODEC_MIN, LIBSWSCALE_MIN);
            }

/*
            build
                //.compiler("clang")
                .define("HAVE_AV_CONFIG_H", None)
                .includes(&["extern/config", "extern/ffmpeg/"])
                .include("extern/ffmpeg/compat/atomics/win32")
                .warnings(false)
                .extra_warnings(false)
                //.flag("-Wno-switch")
                /*
                .flag("-Wno-attributes")
                .flag("-Wno-ignored-qualifiers")
                .flag("-Wno-deprecated-declarations")
                .flag("-Wno-parentheses")
                .flag("-Wno-implicit-const-int-float-conversion")
                */
                ;
                */
        }
    }

    let mut out_dir = env::var("OUT_DIR").unwrap();
    out_dir.push_str("/include");

    build
    //.compiler("clang")
    //.define("HAVE_AV_CONFIG_H", None)
    .includes(&["extern/ffmpeg/", &out_dir])
    ;

    build.file("src/helpers.c").compile("vp6");
}
