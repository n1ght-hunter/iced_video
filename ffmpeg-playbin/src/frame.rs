use std::ops::Deref;

use playbin_core::image;

/// a wrapper around a ffmpeg frame
pub struct Frame(pub(crate) ffmpeg::util::frame::Video);

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frame")
            .field("width", &self.0.width())
            .field("height", &self.0.height())
            .finish()
    }
}

impl Deref for Frame {
    type Target = ffmpeg::util::frame::Video;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl playbin_core::IcedImage for Frame {
    fn get_image(self) -> image::Handle {
        let mut buffer = vec![0; (self.height() * self.width() * 4) as usize];
        let ffmpeg_line_iter = self.data(0).chunks_exact(self.stride(0));

        let slint_pixel_line_iter = buffer.chunks_exact_mut(self.stride(0));

        for (source_line, dest_line) in ffmpeg_line_iter.zip(slint_pixel_line_iter) {
            dest_line.copy_from_slice(&source_line[..])
        }

        image::Handle::from_rgba(self.width() as u32, self.height() as u32, buffer)
    }
}
