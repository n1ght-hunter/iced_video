//! all unsafe functions are in this module

/// checks if gstreamer is initialized
#[allow(unsafe_code)]
pub fn is_initialized() -> bool {
    unsafe { gst::ffi::gst_is_initialized() != 0 }
}
