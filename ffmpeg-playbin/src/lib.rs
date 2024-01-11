pub mod player;

pub use playbin_core::*;

pub fn frame_to_image_handle(frame: &ffmpeg::util::frame::Video) -> iced::widget::image::Handle {
    let mut buffer = vec![0; (frame.height() * frame.width() * 4) as usize];
    let ffmpeg_line_iter = frame.data(0).chunks_exact(frame.stride(0));

    let slint_pixel_line_iter = buffer.chunks_exact_mut(frame.stride(0));

    for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
        dest_line.copy_from_slice(&source_line[..])
    }

    iced::widget::image::Handle::from_pixels(frame.width() as u32, frame.height() as u32, buffer)
}
