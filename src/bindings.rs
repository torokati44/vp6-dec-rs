// Much of this was copied from the (gigantic) bindings auto-generated by `bindgen`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// All structures are opaque to Rust, with no members being accessible.
// This is to keep the cross-language interface as simple as possible.
// The types are only declared for notation purposes, and to enforce type safety.
#[repr(C)]
pub struct AVCodec {
    private: [u8; 0],
}
#[repr(C)]
pub struct AVFrame {
    private: [u8; 0],
}
#[repr(C)]
pub struct AVPacket {
    private: [u8; 0],
}
#[repr(C)]
pub struct AVCodecContext {
    private: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    private: [u8; 0],
}
#[repr(C)]
pub struct SwsContext {
    private: [u8; 0],
}

// These are for the libav headers, mostly copied from the (gigantic) bindings auto-generated by `bindgen`.
extern "C" {
    pub fn avcodec_alloc_context3(codec: *const AVCodec) -> *mut AVCodecContext;
    pub fn avcodec_free_context(avctx: *mut *mut AVCodecContext);

    pub fn avcodec_open2(
        avctx: *mut AVCodecContext,
        codec: *const AVCodec,
        options: *mut *mut AVDictionary,
    ) -> ::std::os::raw::c_int;

    pub fn av_packet_alloc() -> *mut AVPacket;
    pub fn av_packet_free(pkt: *mut *mut AVPacket);

    pub fn av_frame_alloc() -> *mut AVFrame;
    pub fn av_frame_free(frame: *mut *mut AVFrame);

    pub fn avcodec_send_packet(
        avctx: *mut AVCodecContext,
        avpkt: *const AVPacket,
    ) -> ::std::os::raw::c_int;
    pub fn avcodec_receive_frame(
        avctx: *mut AVCodecContext,
        frame: *mut AVFrame,
    ) -> ::std::os::raw::c_int;

    pub fn sws_freeContext(context: *mut SwsContext);
}

// These are for our own helpers, mostly hand-crafted.
extern "C" {
    pub static mut ff_vp6f_decoder_ptr: *mut AVCodec;
    pub static mut ff_vp6a_decoder_ptr: *mut AVCodec;

    pub fn packet_set_size(pkt: *mut AVPacket, size: ::std::os::raw::c_int);
    pub fn packet_data(arg1: *mut AVPacket) -> *mut ::std::os::raw::c_uchar;

    pub fn frame_width(arg1: *mut AVFrame) -> ::std::os::raw::c_int;
    pub fn frame_height(arg1: *mut AVFrame) -> ::std::os::raw::c_int;

    pub fn make_converter_context(yuv_frame: *mut AVFrame) -> *mut SwsContext;
    pub fn convert_yuv_to_rgba(
        context: *mut SwsContext,
        yuv_frame: *mut AVFrame,
        rgba_data: *mut ::std::os::raw::c_uchar,
    );
}
