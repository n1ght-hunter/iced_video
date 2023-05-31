pub mod video_frame;

use std::ffi::{c_char, c_int, c_void, CStr};

use ffmpeg::{ffi::*, Error};
use iced_native::image;
use tracing::{error, info, Level};

use crate::helpers::types::Frame;

use video_frame::VideoFunctions;

pub fn create_ffmpeg_logger() {
    unsafe {
        av_log_set_callback(Some(logger_callback));
    }
}

/// Set the `time_base` field of a decoder. (Not natively supported in
/// the public API.)
///
/// # Arguments
///
/// * `decoder_context` - Decoder context.
/// * `time_base` - Time base to assign.
pub fn set_decoder_context_time_base(
    decoder_context: &mut ffmpeg::codec::Context,
    time_base: ffmpeg::Rational,
) {
    unsafe {
        (*decoder_context.as_mut_ptr()).time_base = time_base.into();
    }
}

/// Internal function with C-style callback behavior that receives all log
/// messages from ffmpeg and handles them with the `log` crate, the Rust way.
///
/// # Arguments
///
/// * `avcl` - Internal struct with log message data.
/// * `level_no` - Log message level integer.
/// * `fmt` - Log message format string.
/// * `vl` - Variable list with format string items.
unsafe extern "C" fn logger_callback(
    avcl: *mut c_void,
    level_no: c_int,
    fmt: *const c_char,
    #[cfg(all(target_arch = "x86_64", target_family = "unix"))] vl: *mut __va_list_tag,
    #[cfg(not(all(target_arch = "x86_64", target_family = "unix")))] vl: va_list,
) {
    static mut PRINT_PREFIX: c_int = 1;

    // Check whether or not the message would be printed at all.
    let event_would_log = match level_no {
        // These are all error states.
        AV_LOG_PANIC | AV_LOG_FATAL | AV_LOG_ERROR => tracing::enabled!(Level::ERROR),
        AV_LOG_WARNING => tracing::enabled!(Level::WARN),
        AV_LOG_INFO => tracing::enabled!(Level::INFO),
        // There is no "verbose" in `log`, so we just put it
        // in the "debug" category.
        AV_LOG_VERBOSE | AV_LOG_DEBUG => tracing::enabled!(Level::DEBUG),
        AV_LOG_TRACE => tracing::enabled!(Level::TRACE),
        _ => {
            return;
        }
    };

    if event_would_log {
        // Allocate some memory for the log line (might be truncated).
        // 1024 bytes is the number used by ffmpeg itself, so it should
        // be mostly fine.
        let mut line = [0_i8; 1024];
        // Use the ffmpeg default formatting.
        let ret = av_log_format_line2(
            avcl,
            level_no,
            fmt,
            vl,
            line.as_mut_ptr(),
            (line.len()) as c_int,
            (&mut PRINT_PREFIX) as *mut c_int,
        );
        // Simply discard the log message if formatting fails.
        if ret > 0 {
            if let Ok(line) = CStr::from_ptr(line.as_mut_ptr()).to_str() {
                let line = line.trim();
                if log_filter_hacks(line) {
                    match level_no {
                        // These are all error states.
                        AV_LOG_PANIC | AV_LOG_FATAL | AV_LOG_ERROR => {
                            tracing::error!(target: "video", "{}", line)
                        }
                        AV_LOG_WARNING => tracing::warn!(target: "video", "{}", line),
                        AV_LOG_INFO => tracing::info!(target: "video", "{}", line),
                        // There is no "verbose" in `log`, so we just put it
                        // in the "debug" category.
                        AV_LOG_VERBOSE | AV_LOG_DEBUG => {
                            tracing::debug!(target: "video", "{}", line)
                        }
                        AV_LOG_TRACE => tracing::trace!(target: "video", "{}", line),
                        _ => {
                            return;
                        }
                    };
                }
            }
        }
    }
}

/// Helper function to filter out any lines that we don't want to log
/// because they contaminate. Currently, it includes the following log
/// line hacks:
///
/// * **Pelco H264 encoding issue**. Pelco cameras and encoders have a
///   problem with their SEI NALs that causes ffmpeg to complain but
///   does not hurt the stream. It does cause continuous error messages
///   though which we filter out here.
fn log_filter_hacks(line: &str) -> bool {
    /* Hack 1 */
    const HACK_1_PELCO_NEEDLE_1: &str = "SEI type 5 size";
    const HACK_1_PELCO_NEEDLE_2: &str = "truncated at";
    if line.find(HACK_1_PELCO_NEEDLE_1).is_some() && line.find(HACK_1_PELCO_NEEDLE_2).is_some() {
        return false;
    }

    true
}

/// Convert a `ffmpeg::Frame` to a `Frame` struct.
pub fn convert_frame_to_image_handle(frame: &mut ffmpeg::Frame) -> Result<Frame, Error> {
    let frame_width = frame.width();
    let frame_height = frame.height();
    let frame_format = frame.format();
    let src_data = frame.data();
    let src_linesize = frame.linesize();

    let frame_array = vec![0; (frame_height * frame_width * 4) as usize];

    let pixels = copy_image_to_buffer(
        frame_array,
        src_data,
        src_linesize,
        frame_format,
        frame_width,
        frame_height,
    )?;

    Ok(image::Handle::from_pixels(
        frame_width as u32,
        frame_height as u32,
        pixels,
    ))
}

fn copy_image_to_buffer(
    mut buffer: Vec<u8>,
    src_data: [*mut u8; 8],
    src_linesize: [i32; 8],
    frame_format: AVPixelFormat,
    frame_width: i32,
    frame_height: i32,
) -> Result<Vec<u8>, Error> {
    let bytes_copied = unsafe {
        av_image_copy_to_buffer(
            buffer.as_mut_ptr(),
            buffer.len() as i32,
            src_data.as_ptr() as *const *const u8,
            src_linesize.as_ptr() as *const i32,
            frame_format,
            frame_width,
            frame_height,
            1,
        )
    };

    if bytes_copied == buffer.len() as i32 {
        Ok(buffer)
    } else {
        error!(
            "Failed to copy image to buffer: {} should be {}",
            bytes_copied,
            buffer.len()
        );
        Err(Error::from(bytes_copied))
    }
}

/// Copy frame properties from `src` to `dst`.
///
/// # Arguments
///
/// * `src` - Frame to get properties from.
/// * `dst` - Frame to copy properties to.
pub fn copy_frame_props(src: &ffmpeg::frame::Video, dst: &mut ffmpeg::frame::Video) {
    unsafe {
        av_frame_copy_props(dst.as_mut_ptr(), src.as_ptr());
    }
}
