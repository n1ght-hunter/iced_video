
pub trait VideoFunctions {
    fn data(&self) -> [*mut u8; 8];
    fn linesize(&self) -> [i32; 8];
    fn extended_data(&self) -> *mut *mut u8;
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn nb_samples(&self) -> i32;
    fn format(&self) -> ffmpeg::ffi::AVPixelFormat;
    fn key_frame(&self) -> i32;
    fn pict_type(&self) -> ffmpeg::ffi::AVPictureType;
    fn sample_aspect_ratio(&self) -> ffmpeg::ffi::AVRational;
    fn pts(&self) -> i64;
    fn pkt_dts(&self) -> i64;
    fn time_base(&self) -> ffmpeg::ffi::AVRational;
    fn coded_picture_number(&self) -> i32;
    fn display_picture_number(&self) -> i32;
    fn quality(&self) -> i32;
    fn opaque(&self) -> *mut std::ffi::c_void;
    fn repeat_pict(&self) -> i32;
    fn interlaced_frame(&self) -> i32;
    fn top_field_first(&self) -> i32;
    fn palette_has_changed(&self) -> i32;
    fn reordered_opaque(&self) -> i64;
    fn sample_rate(&self) -> i32;
    fn channel_layout(&self) -> u64;
    fn buf(&self) -> [*mut ffmpeg::ffi::AVBufferRef; 8];
    fn extended_buf(&self) -> *mut *mut ffmpeg::ffi::AVBufferRef;
    fn nb_extended_buf(&self) -> i32;
    fn side_data(&self) -> *mut *mut ffmpeg::ffi::AVFrameSideData;
    fn nb_side_data(&self) -> i32;
    fn flags(&self) -> i32;
    fn color_range(&self) -> ffmpeg::ffi::AVColorRange;
    fn color_primaries(&self) -> ffmpeg::ffi::AVColorPrimaries;
    fn color_trc(&self) -> ffmpeg::ffi::AVColorTransferCharacteristic;
    fn colorspace(&self) -> ffmpeg::ffi::AVColorSpace;
    fn chroma_location(&self) -> ffmpeg::ffi::AVChromaLocation;
    fn best_effort_timestamp(&self) -> i64;
    fn pkt_pos(&self) -> i64;
    fn pkt_duration(&self) -> i64;
    fn metadata(&self) -> *mut ffmpeg::ffi::AVDictionary;
    fn decode_error_flags(&self) -> i32;
    fn channels(&self) -> i32;
    fn pkt_size(&self) -> i32;
    fn hw_frames_ctx(&self) -> *mut ffmpeg::ffi::AVBufferRef;
    fn opaque_ref(&self) -> *mut ffmpeg::ffi::AVBufferRef;
    fn crop_top(&self) -> usize;
    fn crop_bottom(&self) -> usize;
    fn crop_left(&self) -> usize;
    fn crop_right(&self) -> usize;
    fn private_ref(&self) -> *mut ffmpeg::ffi::AVBufferRef;
    fn ch_layout(&self) -> ffmpeg::ffi::AVChannelLayout;
    fn duration(&self) -> i64;
}

impl VideoFunctions for ffmpeg::Frame {
    fn data(&self) -> [*mut u8; 8] {
        unsafe { (*self.as_ptr()).data }
    }

    fn linesize(&self) -> [i32; 8] {
        unsafe { (*self.as_ptr()).linesize }
    }

    fn extended_data(&self) -> *mut *mut u8 {
        unsafe { (*self.as_ptr()).extended_data }
    }
    fn width(&self) -> i32 {
        unsafe { (*self.as_ptr()).width }
    }

    fn height(&self) -> i32 {
        unsafe { (*self.as_ptr()).height }
    }

    fn nb_samples(&self) -> i32 {
        unsafe { (*self.as_ptr()).nb_samples }
    }

    fn format(&self) -> ffmpeg::ffi::AVPixelFormat {
        unsafe {
            let format = (*self.as_ptr()).format;
            std::mem::transmute::<std::ffi::c_int, ffmpeg::ffi::AVPixelFormat>(format)
        }
    }

    fn key_frame(&self) -> i32 {
        unsafe { (*self.as_ptr()).key_frame }
    }

    fn pict_type(&self) -> ffmpeg::ffi::AVPictureType {
        unsafe { (*self.as_ptr()).pict_type }
    }

    fn sample_aspect_ratio(&self) -> ffmpeg::ffi::AVRational {
        unsafe { (*self.as_ptr()).sample_aspect_ratio }
    }

    fn pts(&self) -> i64 {
        unsafe { (*self.as_ptr()).pts }
    }

    fn pkt_dts(&self) -> i64 {
        unsafe { (*self.as_ptr()).pkt_dts }
    }

    fn time_base(&self) -> ffmpeg::ffi::AVRational {
        unsafe { (*self.as_ptr()).time_base }
    }

    fn coded_picture_number(&self) -> i32 {
        unsafe { (*self.as_ptr()).coded_picture_number }
    }

    fn display_picture_number(&self) -> i32 {
        unsafe { (*self.as_ptr()).display_picture_number }
    }

    fn quality(&self) -> i32 {
        unsafe { (*self.as_ptr()).quality }
    }

    fn opaque(&self) -> *mut std::ffi::c_void {
        unsafe { (*self.as_ptr()).opaque }
    }

    fn repeat_pict(&self) -> i32 {
        unsafe { (*self.as_ptr()).repeat_pict }
    }

    fn interlaced_frame(&self) -> i32 {
        unsafe { (*self.as_ptr()).interlaced_frame }
    }

    fn top_field_first(&self) -> i32 {
        unsafe { (*self.as_ptr()).top_field_first }
    }

    fn palette_has_changed(&self) -> i32 {
        unsafe { (*self.as_ptr()).palette_has_changed }
    }

    fn reordered_opaque(&self) -> i64 {
        unsafe { (*self.as_ptr()).reordered_opaque }
    }

    fn sample_rate(&self) -> i32 {
        unsafe { (*self.as_ptr()).sample_rate }
    }

    fn channel_layout(&self) -> u64 {
        unsafe { (*self.as_ptr()).channel_layout }
    }

    fn buf(&self) -> [*mut ffmpeg::ffi::AVBufferRef; 8] {
        unsafe { (*self.as_ptr()).buf }
    }

    fn extended_buf(&self) -> *mut *mut ffmpeg::ffi::AVBufferRef {
        unsafe { (*self.as_ptr()).extended_buf }
    }

    fn nb_extended_buf(&self) -> i32 {
        unsafe { (*self.as_ptr()).nb_extended_buf }
    }

    fn side_data(&self) -> *mut *mut ffmpeg::ffi::AVFrameSideData {
        unsafe { (*self.as_ptr()).side_data }
    }

    fn nb_side_data(&self) -> i32 {
        unsafe { (*self.as_ptr()).nb_side_data }
    }

    fn flags(&self) -> i32 {
        unsafe { (*self.as_ptr()).flags }
    }

    fn color_range(&self) -> ffmpeg::ffi::AVColorRange {
        unsafe { (*self.as_ptr()).color_range }
    }

    fn color_primaries(&self) -> ffmpeg::ffi::AVColorPrimaries {
        unsafe { (*self.as_ptr()).color_primaries }
    }

    fn color_trc(&self) -> ffmpeg::ffi::AVColorTransferCharacteristic {
        unsafe { (*self.as_ptr()).color_trc }
    }

    fn colorspace(&self) -> ffmpeg::ffi::AVColorSpace {
        unsafe { (*self.as_ptr()).colorspace }
    }

    fn chroma_location(&self) -> ffmpeg::ffi::AVChromaLocation {
        unsafe { (*self.as_ptr()).chroma_location }
    }

    fn best_effort_timestamp(&self) -> i64 {
        unsafe { (*self.as_ptr()).best_effort_timestamp }
    }

    fn pkt_pos(&self) -> i64 {
        unsafe { (*self.as_ptr()).pkt_pos }
    }

    fn pkt_duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).pkt_duration }
    }

    fn metadata(&self) -> *mut ffmpeg::ffi::AVDictionary {
        unsafe { (*self.as_ptr()).metadata }
    }

    fn decode_error_flags(&self) -> i32 {
        unsafe { (*self.as_ptr()).decode_error_flags }
    }

    fn channels(&self) -> i32 {
        unsafe { (*self.as_ptr()).channels }
    }

    fn pkt_size(&self) -> i32 {
        unsafe { (*self.as_ptr()).pkt_size }
    }

    fn hw_frames_ctx(&self) -> *mut ffmpeg::ffi::AVBufferRef {
        unsafe { (*self.as_ptr()).hw_frames_ctx }
    }

    fn opaque_ref(&self) -> *mut ffmpeg::ffi::AVBufferRef {
        unsafe { (*self.as_ptr()).opaque_ref }
    }

    fn crop_top(&self) -> usize {
        unsafe { (*self.as_ptr()).crop_top }
    }

    fn crop_bottom(&self) -> usize {
        unsafe { (*self.as_ptr()).crop_bottom }
    }

    fn crop_left(&self) -> usize {
        unsafe { (*self.as_ptr()).crop_left }
    }

    fn crop_right(&self) -> usize {
        unsafe { (*self.as_ptr()).crop_right }
    }

    fn private_ref(&self) -> *mut ffmpeg::ffi::AVBufferRef {
        unsafe { (*self.as_ptr()).private_ref }
    }

    fn ch_layout(&self) -> ffmpeg::ffi::AVChannelLayout {
        unsafe { (*self.as_ptr()).ch_layout }
    }

    fn duration(&self) -> i64 {
        unsafe { (*self.as_ptr()).duration }
    }
}
