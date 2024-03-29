// Much of this was copied from the (gigantic) bindings auto-generated by `bindgen`.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// All structures are opaque to Rust, with no members being accessible.
// This is to keep the cross-language interface as simple as possible.
// The types are only declared for notation purposes, and to enforce type safety.
#[repr(C)]
pub struct AvCodec {
    private: [u8; 0],
}
#[repr(C)]
pub struct AvFrame {
    private: [u8; 0],
}
#[repr(C)]
pub struct AvPacket {
    private: [u8; 0],
}
#[repr(C)]
pub struct AvCodecContext {
    private: [u8; 0],
}
#[repr(C)]
pub struct AvDictionary {
    private: [u8; 0],
}
#[repr(C)]
pub struct SwsContext {
    private: [u8; 0],
}

// These are for the FFmpeg headers, mostly copied from the (gigantic) bindings auto-generated by `bindgen`.
extern "C" {
    pub fn avcodec_alloc_context3(codec: *const AvCodec) -> *mut AvCodecContext;
    pub fn avcodec_free_context(avctx: *mut *mut AvCodecContext);

    pub fn avcodec_open2(
        avctx: *mut AvCodecContext,
        codec: *const AvCodec,
        options: *mut *mut AvDictionary,
    ) -> ::std::os::raw::c_int;

    pub fn av_packet_alloc() -> *mut AvPacket;
    pub fn av_packet_free(pkt: *mut *mut AvPacket);

    pub fn av_frame_alloc() -> *mut AvFrame;
    pub fn av_frame_free(frame: *mut *mut AvFrame);

    pub fn avcodec_send_packet(
        avctx: *mut AvCodecContext,
        avpkt: *const AvPacket,
    ) -> ::std::os::raw::c_int;
    pub fn avcodec_receive_frame(
        avctx: *mut AvCodecContext,
        frame: *mut AvFrame,
    ) -> ::std::os::raw::c_int;

    pub fn sws_freeContext(context: *mut SwsContext);
}

// These are for our own helpers, mostly hand-crafted.
extern "C" {
    pub fn find_vp6_decoder(with_alpha: ::std::os::raw::c_int) -> *mut AvCodec;

    pub fn packet_set_size(pkt: *mut AvPacket, size: ::std::os::raw::c_int);
    pub fn packet_data(arg1: *mut AvPacket) -> *mut ::std::os::raw::c_uchar;

    pub fn frame_width(arg1: *mut AvFrame) -> ::std::os::raw::c_int;
    pub fn frame_height(arg1: *mut AvFrame) -> ::std::os::raw::c_int;

    pub fn make_converter_context(yuv_frame: *mut AvFrame) -> *mut SwsContext;
    pub fn convert_yuv_to_rgba(
        context: *mut SwsContext,
        yuv_frame: *mut AvFrame,
        rgba_data: *mut ::std::os::raw::c_uchar,
    );
}
