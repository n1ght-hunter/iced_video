//! # ffmpeg-playbin
//! a video player buint on top of ffmpeg.
//! this player impls the [BasicPlayer] traits.
//! 
//! WIP

#![deny(
    missing_debug_implementations,
    missing_docs,
    unused_results,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion,
    unsafe_code
)]
#![forbid(rust_2018_idioms)]
#![allow(clippy::inherent_to_string, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]


pub mod player;

pub use playbin_core::*;

/// convert ffmpeg frame to iced [image::Handle]
pub fn frame_to_image_handle(frame: &ffmpeg::util::frame::Video) -> image::Handle {
    let mut buffer = vec![0; (frame.height() * frame.width() * 4) as usize];
    let ffmpeg_line_iter = frame.data(0).chunks_exact(frame.stride(0));

    let slint_pixel_line_iter = buffer.chunks_exact_mut(frame.stride(0));

    for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
        dest_line.copy_from_slice(&source_line[..])
    }

    image::Handle::from_pixels(frame.width() as u32, frame.height() as u32, buffer)
}
