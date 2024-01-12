
/// Error type for the gstreamer-playbin crate
#[derive(Debug)]
pub enum GstreamerError {
    /// Glib error
    Glib(gst::glib::Error),
    /// Element not found
    MissingElement(&'static str),
    /// unknow error
    GstBoolError(gst::glib::BoolError),
    /// Type mismatch error
    TypeMismatch(gst::structure::GetError<gst::glib::value::ValueTypeMismatchError>),
    /// Custom error
    CustomError(&'static str),
}

impl From<&'static str> for GstreamerError {
    fn from(e: &'static str) -> Self {
        GstreamerError::CustomError(e)
    }
}

impl From<gst::StateChangeError> for GstreamerError {
    fn from(_: gst::StateChangeError) -> Self {
        GstreamerError::CustomError("Element failed to change its state")
    }
}

impl From<gst::glib::Error> for GstreamerError {
    fn from(e: gst::glib::Error) -> Self {
        GstreamerError::Glib(e)
    }
}

impl From<gst::glib::BoolError> for GstreamerError {
    fn from(e: gst::glib::BoolError) -> Self {
        GstreamerError::GstBoolError(e)
    }
}

impl From<gst::structure::GetError<gst::glib::value::ValueTypeMismatchError>> for GstreamerError {
    fn from(e: gst::structure::GetError<gst::glib::value::ValueTypeMismatchError>) -> Self {
        GstreamerError::TypeMismatch(e)
    }
}
