mod error;
pub mod ffi;
pub mod helpers;
pub mod decoder;
pub mod playbin;
pub mod reader;
mod render_queue;
mod audio;
pub use tokio;
pub use tracing;

/// Initialize global ffmpeg settings. This also intializes the
/// logging capability and redirect it to `tracing`.
pub fn init() -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;

    // Redirect logging to the Rust `tracing` crate.
    ffi::create_ffmpeg_logger();

    Ok(())
}
