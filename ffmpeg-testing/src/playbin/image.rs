use super::time::Time;

#[derive(Debug)]
pub struct VideoFrame {
    time: Time,
    width: u32,
    height: u32,
    data: iced_native::image::Handle,
}

impl VideoFrame {
    pub fn new(time: Time, width: u32, height: u32, data: iced_native::image::Handle) -> Self {
        Self {
            time,
            width,
            height,
            data,
        }
    }

    pub fn into_raw_image(self) -> iced_native::image::Handle {
        self.data
    }

    pub fn time(&self) -> Time {
        self.time.clone()
    }
}