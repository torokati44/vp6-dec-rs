use std::ptr::slice_from_raw_parts_mut;

use crate::bindings::*;

pub struct Vp6State {
    context: *mut AvCodecContext,
    packet: *mut AvPacket,
    yuv_frame: *mut AvFrame,
    #[cfg(feature = "with-swscale")]
    sws_context: *mut SwsContext,
}

impl Vp6State {
    pub fn new(with_alpha: bool) -> Self {
        unsafe {
            let codec: *mut AvCodec = find_vp6_decoder(if with_alpha { 1 } else { 0 });
            let context: *mut AvCodecContext = avcodec_alloc_context3(codec);

            avcodec_open2(context, codec, std::ptr::null_mut::<*mut AvDictionary>());

            let packet: *mut AvPacket = av_packet_alloc();
            let yuv_frame: *mut AvFrame = av_frame_alloc();

            Self {
                context,
                packet,
                yuv_frame,
                #[cfg(feature = "with-swscale")]
                sws_context: std::ptr::null_mut(),
            }
        }
    }

    pub fn decode(&mut self, encoded_frame: &[u8]) -> (Vec<u8>, (usize, usize)) {
        unsafe {
            packet_set_size(self.packet, encoded_frame.len() as i32);

            let data = packet_data(self.packet);
            for (i, e) in encoded_frame.iter().enumerate() {
                (*slice_from_raw_parts_mut(data, encoded_frame.len()))[i] = *e;
            }

            let _ret = avcodec_send_packet(self.context, self.packet);
            let _ret = avcodec_receive_frame(self.context, self.yuv_frame);
            // TODO: check for return values (errors) everywhere, with proper cleanup!

            #[cfg(feature = "with-swscale")]
            if self.sws_context.is_null() {
                self.sws_context = make_converter_context(self.yuv_frame);
            }

            let w = frame_width(self.yuv_frame) as usize;
            let h = frame_height(self.yuv_frame) as usize;

            #[cfg(feature = "with-swscale")]
            {
                let num_pixels = w * h;
                let mut rgba_data = vec![0; num_pixels * 4];
                convert_yuv_to_rgba(self.sws_context, self.yuv_frame, rgba_data.as_mut_ptr());
                (rgba_data, (w, h))
            }

            #[cfg(not(feature = "with-swscale"))]
            {
                let cbcr_w = (w+1)/2;
                let cbcr_h = (h+1)/2;

                let mut y_data = vec![0; w * h];
                let mut cb_data = vec![0; cbcr_w * cbcr_h];
                let mut cr_data = vec![0; cbcr_w * cbcr_h];

                get_yuv_frame(self.yuv_frame, y_data.as_mut_ptr(), cb_data.as_mut_ptr(), cr_data.as_mut_ptr());

                let rgba = h263_rs_yuv::bt601::yuv420_to_rgba(&y_data, &cb_data, &cr_data, w, cbcr_w);
                (rgba, (w, h))
            }
        }
    }
}

impl Drop for Vp6State {
    fn drop(&mut self) {
        unsafe {
            #[cfg(feature = "with-swscale")]
            sws_freeContext(self.sws_context);
            av_frame_free(&mut self.yuv_frame);
            av_packet_free(&mut self.packet);
            avcodec_free_context(&mut self.context);
        }
    }
}
