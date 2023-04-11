#[derive(Debug)]
pub enum GstreamerError {
    Glib(gst::glib::Error),
    MissingElement(&'static str),
    GstBoolError(gst::glib::BoolError),
    TypeMismatch(gst::structure::GetError<gst::glib::value::ValueTypeMismatchError>),
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
