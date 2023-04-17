
use super::ffi::create_ffmpeg_logger;

/// Initialize global ffmpeg settings. This also intializes the
/// logging capability and redirect it to `tracing`.
pub fn init() -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;
  
    // Redirect logging to the Rust `tracing` crate.
    create_ffmpeg_logger();
  
    Ok(())
  }