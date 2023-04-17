
#[derive(Debug)]
pub enum FFMPEGPLayerErros {
    FFmpegError(ffmpeg::Error),
    CustomError(String),
}

impl From<&'static str> for FFMPEGPLayerErros {
    fn from(err: &'static str) -> Self {
        FFMPEGPLayerErros::CustomError(err.to_string())
    }
}

impl From<String> for FFMPEGPLayerErros {
    fn from(err: String) -> Self {
        FFMPEGPLayerErros::CustomError(err)
    }
}

impl From<ffmpeg::Error> for FFMPEGPLayerErros {
    fn from(err: ffmpeg::Error) -> Self {
        FFMPEGPLayerErros::FFmpegError(err)
    }
}