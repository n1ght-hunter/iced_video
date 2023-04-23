use crate::playbin::{decoder::DecoderError, reader::ReaderError};

#[derive(Debug)]
pub enum PlaybinError {
    NoVideoStream,
}

#[derive(Debug)]
pub enum FFMPEGPLayerErros {
    FFmpegError(ffmpeg::Error),
    Playbin(PlaybinError),
    Decoder(DecoderError),
    Reader(ReaderError),
    CustomError(String),
}

impl From<ReaderError> for FFMPEGPLayerErros {
    fn from(err: ReaderError) -> Self {
        FFMPEGPLayerErros::Reader(err)
    }
}

impl From<DecoderError> for FFMPEGPLayerErros {
    fn from(err: DecoderError) -> Self {
        FFMPEGPLayerErros::Decoder(err)
    }
}

impl From<PlaybinError> for FFMPEGPLayerErros {
    fn from(err: PlaybinError) -> Self {
        FFMPEGPLayerErros::Playbin(err)
    }
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
