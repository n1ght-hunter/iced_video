use cpal::{traits::{DeviceTrait, HostTrait}, SizedSample};
use tracing::error;

#[derive(Debug)]
pub enum AudioError {
    NoDefaultOutputDevice,
    BuildStreamError(cpal::BuildStreamError),
}

pub struct AudioHandler {
    sender: tokio::sync::mpsc::Sender<ffmpeg::frame::Audio>,
    stream: cpal::Stream,
    audio_config: cpal::SupportedStreamConfig,
}

impl AudioHandler {
    pub fn new() -> Result<Self, AudioError> {
        let host = cpal::default_host();

        let audio_device = host
            .default_output_device()
            .ok_or(AudioError::NoDefaultOutputDevice)?;

        let audio_config = audio_device.default_output_config().unwrap();

        let (sender, mut receiver) = tokio::sync::mpsc::channel(100);

        let stream = create_player::<f32>(&audio_device, &audio_config.config(), receiver).map_err(AudioError::BuildStreamError)?;

        Ok(Self {
            stream,
            sender,
            audio_config,
        })
    }

    pub fn config(&self) -> &cpal::SupportedStreamConfig {
        &self.audio_config
    }
}


fn create_player<T>(audio_device: &cpal::Device, config: &cpal::StreamConfig, mut receiver: tokio::sync::mpsc::Receiver<ffmpeg::frame::Audio>)
-> Result<cpal::Stream, cpal::BuildStreamError> where
    T: SizedSample + Default,
{
    audio_device.build_output_stream(
        &config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            data.iter_mut().for_each(|d| {
                if let Ok(frame) = receiver.try_recv() {
                    // *d = frame.samples()
                }
                else {
                    *d = T::default()
                }
            });
            
        },
        move |err| {
            error!("audio stream error: {:?}", err);
        },
        None,
    )
}