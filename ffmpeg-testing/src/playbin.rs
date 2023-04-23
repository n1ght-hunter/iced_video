use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
    time::Duration,
};

use tokio::sync::{Mutex, Notify};
use tracing::{error, info};

use crate::player::error::{FFMPEGPLayerErros, PlaybinError};

use self::{
    image::VideoFrame,
    reader::{Reader, ReaderError},
};

pub mod decoder;
pub mod image;
pub mod playbin_trait;
pub mod reader;
pub mod time;
pub mod types;

pub struct PlayBin<T> {
    paused: Arc<AtomicBool>,
    unpause: Arc<Notify>,
    curret_handle: Option<tokio::task::JoinHandle<()>>,
    sample_callback: Arc<Mutex<Option<T>>>,
    reader: Arc<Mutex<Option<Reader>>>,
}

impl<T> playbin_trait::PlayBinTrait<T> for PlayBin<T>
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    fn new() -> Self {
        let paused = Arc::new(AtomicBool::new(true));
        let unpause = Arc::new(Notify::new());
        Self {
            paused,
            unpause,
            curret_handle: None,
            sample_callback: Arc::new(Mutex::new(None)),
            reader: Arc::new(Mutex::new(None)),
        }
    }

    fn play(&self) {
        self.paused
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.unpause.notify_one();
    }

    fn pause(&self) {
        self.paused
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    fn seek(&self, time: Duration) {
        let reader = self.reader.clone();
        tokio::spawn(async move {
            if let Some(reader) = reader.lock().await.as_mut() {
                if let Err(e) = reader.seek(time) {
                    error!("seek: {:?}", e);
                }
            }
        });
    }

    fn stop(&mut self) {
        if let Some(handle) = self.curret_handle.take() {
            handle.abort();
        }
    }

    fn set_source(&mut self, uri: &std::path::Path) {
        if let Some(handle) = self.curret_handle.take() {
            handle.abort();
        }
        let uri = uri.to_path_buf();
        self.spawn_reader(uri);
    }

    fn set_sample_callback(&self, sample_callback: T) {
        let callback = self.sample_callback.clone();
        tokio::spawn(async move {
            let mut callback = callback.lock().await;
            *callback = Some(sample_callback);
        });
    }
}

impl<T> PlayBin<T>
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    fn spawn_reader(&mut self, uri: PathBuf) {
        let paused_clone = self.paused.clone();
        let unpause_clone = self.unpause.clone();
        let sample_callback = self.sample_callback.clone();
        let reader = self.reader.clone();

        self.curret_handle = Some(tokio::spawn(async move {
            if let Err(err) =
                thread_handler(uri, paused_clone, unpause_clone, sample_callback, reader).await
            {
                error!("ffmpeg backend thread: {:?}", err);
            }
        }));
    }
}

async fn thread_handler<T>(
    uri: PathBuf,
    paused: Arc<AtomicBool>,
    unpause: Arc<Notify>,
    sample_callback: Arc<Mutex<Option<T>>>,
    reader: Arc<Mutex<Option<Reader>>>,
) -> Result<(), FFMPEGPLayerErros>
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    let mut current_reader = Reader::new(&uri)?;

    let mut decoder = current_reader.use_input(|input| {
        decoder::Decoder::new(
            ffmpeg::format::Pixel::RGBA,
            // safe to unwrap because we checked for a video stream when creating the reader
            input
                .streams()
                .best(ffmpeg::media::Type::Video)
                .ok_or(ReaderError::NoVideoStream)
                .unwrap(),
        )
    })?;

    let (width, height) = decoder.size();

    {
        let mut reader = reader.lock().await;
        *reader = Some(current_reader);
    }

    let start = std::time::Instant::now();

    loop {
        if paused.load(std::sync::atomic::Ordering::Relaxed) {
            unpause.notified().await;
        }

        if let Ok((mut packet, time)) = reader.lock().await.as_mut().unwrap().read() {
            match decoder.decode_image_handle(&mut packet, time) {
                Ok((time, image)) => {
                    let image = VideoFrame::new(time, width, height, image);

                    let callback = sample_callback.clone();

                    let startdur = start.elapsed();
                    let frame_tiem = Duration::from(image.time());
                    if frame_tiem > startdur {
                        let deadline = tokio::time::Instant::now() + (frame_tiem - startdur);

                        tokio::time::sleep_until(deadline).await;
                    }
                    send_frame(callback, image)
                }
                Err(err) => {
                    error!("error: {:?}", err);
                }
            }
        } else {
            info!("Read Exhausted");
            break;
        }
    }
    Ok(())
}

fn send_frame<T>(function: Arc<Mutex<Option<T>>>, image: VideoFrame)
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    tokio::spawn(async move {
        if let Err(e) = tokio::task::spawn_blocking(move || {
            let mut sample_callback = function.blocking_lock();
            if let Some(sample_callback) = sample_callback.as_mut() {
                sample_callback(image);
            }
        })
        .await
        {
            error!("sample callback error: {:?}", e);
        }
    });
}
