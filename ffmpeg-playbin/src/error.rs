use crate::{
    decoder::{audio_decoder::AudioDecoderError, video_decoder::VideoDecoderError},
    reader::ReaderError,
};

#[derive(Debug)]
pub enum PlaybinError {
    NoVideoStream,
    NoDefaultOutputDevice,
}

#[derive(Debug)]
pub enum FFMPEGPLayerErros {
    FFmpegError(ffmpeg::Error),
    Playbin(PlaybinError),
    VideoDecoder(VideoDecoderError),
    AudioDecoder(AudioDecoderError),
    Reader(ReaderError),
    CustomError(String),
}

impl From<ReaderError> for FFMPEGPLayerErros {
    fn from(err: ReaderError) -> Self {
        FFMPEGPLayerErros::Reader(err)
    }
}

impl From<VideoDecoderError> for FFMPEGPLayerErros {
    fn from(err: VideoDecoderError) -> Self {
        FFMPEGPLayerErros::VideoDecoder(err)
    }
}

impl From<AudioDecoderError> for FFMPEGPLayerErros {
    fn from(err: AudioDecoderError) -> Self {
        FFMPEGPLayerErros::AudioDecoder(err)
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
