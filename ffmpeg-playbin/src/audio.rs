// mod stream;

use std::{fmt::Debug, sync::Arc};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample, Sample,
};
use ffmpeg::frame::Audio;
use tracing::error;

use crate::render_queue::Queue;

#[derive(Debug)]
pub enum AudioError {
    NoDefaultOutputDevice,
    BuildStreamError(cpal::BuildStreamError),
}

pub struct AudioHandler<T> {
    queue: Arc<Queue<T>>,
    audio_config: cpal::SupportedStreamConfig,
    audio_device: cpal::Device,
}

impl<T> AudioHandler<T>
where
    T: SizedSample  + Default + Send + 'static,
{
    pub fn new() -> Result<Self, AudioError> {
        let host = cpal::default_host();

        let audio_device = host
            .default_output_device()
            .ok_or(AudioError::NoDefaultOutputDevice)?;

        let audio_config = audio_device.default_output_config().unwrap();

        Ok(Self {
            queue: Arc::new(Queue::new(100)),
            audio_config,
            audio_device,
        })
    }

    pub fn config(&self) -> &cpal::SupportedStreamConfig {
        &self.audio_config
    }

    pub fn spawn(self) -> (tokio::sync::mpsc::Sender<f32>, tokio::task::JoinHandle<()>) {
        let (sender, mut receiver) = tokio::sync::mpsc::channel(100);

        let handle = tokio::spawn(async move {
            let _ = tokio::task::spawn_blocking(move || {
                let config = self.audio_config.config();
                let queue = self.queue.clone();
                let stream = create_player::<T>(&self.audio_device, &config, queue).unwrap();

                stream.play().unwrap();

                while let Some(command) = receiver.blocking_recv() {
                    println!("command: {}", command);
                }
            })
            .await;
        });

        (sender, handle)
    }

    pub fn get_queue(&self) -> Arc<Queue<T>> {
        self.queue.clone()
    }
}

fn create_player<T>(
    audio_device: &cpal::Device,
    config: &cpal::StreamConfig,
    queue: Arc<Queue<T>>,
) -> Result<cpal::Stream, cpal::BuildStreamError>
where
    T: SizedSample + Default + Send + 'static,
{
    audio_device.build_output_stream(
        &config,
        move |data: &mut [T], cbinfo: &cpal::OutputCallbackInfo| {
            
                write_audio(data, queue.clone(), &cbinfo);
          
        },
        move |err| {
            error!("audio stream error: {:?}", err);
        },
        None,
    )
}

fn write_audio<T: SizedSample + Default>(data: &mut [T],  queue: Arc<Queue<T>>, _: &cpal::OutputCallbackInfo) {
    for d in data {
        // copy as many samples as we have.
        // if we run out, write silence
        match queue.try_pop() {
            Some(sample) => *d = sample,
            None => *d = T::default()
        }
    }
}
