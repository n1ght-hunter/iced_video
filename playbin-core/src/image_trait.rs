
/// Trait to turn a struct into an iced image
pub trait IcedImage {
    /// Get the image handle
    fn get_image(self) -> iced::widget::image::Handle;
}