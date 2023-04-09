#[derive(Debug)]
pub enum GstreamerError {
    Glib(glib::Error),
    MissingElement(&'static str),
    GstBoolError(glib::BoolError),
    TypeMismatch(gst::structure::GetError<glib::value::ValueTypeMismatchError>),
    CustomError(&'static str),
}

impl From<gst::StateChangeError> for GstreamerError {
    fn from(_: gst::StateChangeError) -> Self {
        GstreamerError::CustomError("Element failed to change its state")
    }
}

impl From<glib::Error> for GstreamerError {
    fn from(e: glib::Error) -> Self {
        GstreamerError::Glib(e)
    }
}

impl From<glib::BoolError> for GstreamerError {
    fn from(e: glib::BoolError) -> Self {
        GstreamerError::GstBoolError(e)
    }
}

impl From<gst::structure::GetError<glib::value::ValueTypeMismatchError>> for GstreamerError {
    fn from(e: gst::structure::GetError<glib::value::ValueTypeMismatchError>) -> Self {
        GstreamerError::TypeMismatch(e)
    }
}
