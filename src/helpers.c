// The wasm32-unknown-unknown ABI is not always in perfect match with the
// Rust one (even with it trying to be C-like), mostly regarding struct data layout.
// These little helpers are used to avoid having to pass structures by value across
// the FFI boundary, or having to access any C struct members from Rust.
// Trivial types like integers, floating point numbers, and pointers, should be fine.

#include "libavutil/common.h"
#include "libavcodec/avcodec.h"
#include "libswscale/swscale.h"

AVCodec *find_vp6_decoder(int with_alpha) {
    return avcodec_find_decoder(with_alpha ? AV_CODEC_ID_VP6A : AV_CODEC_ID_VP6F);
}

void packet_set_size(AVPacket *p, int size) {
    if (p->size < size) {
        av_grow_packet(p, size - p->size);
    }
    else if (p->size > size) {
        av_shrink_packet(p, size);
    }
}

unsigned char *packet_data(AVPacket *p) {
    return p->data;
}

int frame_width(AVFrame *f) {
    return f->width;
}
int frame_height(AVFrame *f) {
    return f->height;
}

typedef struct SwsContext SwsContext;

SwsContext *make_converter_context(AVFrame *yuv_frame) {
    return sws_getContext(
        yuv_frame->width, yuv_frame->height, yuv_frame->format,
        yuv_frame->width, yuv_frame->height, AV_PIX_FMT_RGBA,
        SWS_BILINEAR, NULL, NULL, NULL);
        // NOTE: this parameter is a great potential tuning point between image
        // quality and decoding speed. Some other reasonable values are:
        // SWS_FAST_BILINEAR, SWS_BICUBIC, SWS_SINC, SWS_LANCZOS, SWS_SPLINE
}

void convert_yuv_to_rgba(SwsContext *context, AVFrame *yuv_frame, uint8_t *rgba_data) {
    int linesize = yuv_frame->width * 4;
    sws_scale(context, yuv_frame->data, yuv_frame->linesize, 0,
        yuv_frame->height, &rgba_data, &linesize);
}


int avpriv_open(const char *filename, int flags, ...) {
    return -1;
}
