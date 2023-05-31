use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicUsize},
        Arc,
    },
    time::Duration,
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample,
};
use ffmpeg::frame::Audio;
use tokio::sync::{mpsc::Sender, Mutex, Notify};
use tracing::{error, info};

use crate::{
    audio::AudioHandler,
    decoder::{audio_decoder::AudioDecoder, convert_audio::ToFFMPEG, video_decoder::VideoDecoder},
    error::{FFMPEGPLayerErros, PlaybinError},
    helpers::{convert_audio::ToType, image::VideoFrame, playbin_trait},
    reader::{ReaderChannelType, StreamPacket},
};

use crate::reader::{Reader, ReaderError};

pub struct PlayBin<T> {
    paused: Arc<AtomicBool>,
    unpause: Arc<Notify>,
    curret_handle: Option<tokio::task::JoinHandle<()>>,
    sample_callback: Arc<Mutex<Option<T>>>,
    use_reader: Option<tokio::sync::mpsc::Sender<ReaderChannelType>>,
}

impl<T> playbin_trait::PlayBinTrait<T> for PlayBin<T>
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    fn new() -> Self {
        Self {
            paused: Arc::new(AtomicBool::new(true)),
            unpause: Arc::new(Notify::new()),
            curret_handle: None,
            sample_callback: Arc::new(Mutex::new(None)),
            use_reader: None,
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
        if let Some(sender) = self.use_reader.clone() {
            tokio::spawn(async move {
                sender
                    .send(crate::reader::ReaderCommand::Seek(time))
                    .await
                    .expect("seek");
            });
        }
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
        self.start_player(uri);
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
    fn start_player(&mut self, uri: PathBuf) {
        let paused_clone = self.paused.clone();
        let unpause_clone = self.unpause.clone();
        let sample_callback = self.sample_callback.clone();
        let (use_reader_send, use_reader_recv) = tokio::sync::mpsc::channel(20);
        self.use_reader = Some(use_reader_send);
        self.curret_handle = Some(tokio::spawn(async move {
            if let Err(err) = thread_handler(
                uri,
                paused_clone,
                unpause_clone,
                sample_callback,
                use_reader_recv,
            )
            .await
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
    use_reader: tokio::sync::mpsc::Receiver<ReaderChannelType>,
) -> Result<(), FFMPEGPLayerErros>
where
    T: FnMut(VideoFrame) + Send + 'static,
{
    let audio_handler = AudioHandler::new().unwrap();

    let audio_config = audio_handler.config();

    let reader = Reader::new(&uri)?;
    // let video_parameters = reader.video_parameters();

    // let video_decoder = VideoDecoder::new(
    //     ffmpeg::format::Pixel::RGBA,
    //     video_parameters.1,
    //     video_parameters.0,
    // )?;

    let audio_parameters = reader.audio_parameters();

    let audio_decoder = AudioDecoder::new(
        audio_config.sample_format().to_ffmpeg(),
        audio_config.channels(),
        audio_config.sample_rate().0,
        audio_parameters.1,
        audio_parameters.0,
    )?;

    let (raw_video_queue, raw_audio_queue, _) = reader.spawn(use_reader, paused, unpause);

    // let (rendered_video_queue, _video_decoder_handle) = video_decoder.spawn(raw_video_queue);
    let (rendered_audio_queue, _audio_decoder_handle) = audio_decoder.spawn(raw_audio_queue);

    // let start = std::time::Instant::now();

    // let _ = tokio::spawn(async move {
    //     loop {
    //         let video_frame = rendered_video_queue.pop().await;
    //         let callback = sample_callback.clone();

    //         let startdur = start.elapsed();
    //         let frame_tiem = Duration::from(video_frame.time());
    //         if frame_tiem > startdur {
    //             let deadline = tokio::time::Instant::now() + (frame_tiem - startdur);

    //             tokio::time::sleep_until(deadline).await;
    //         }
    //         send_frame(callback, video_frame)
    //     }
    // });

    let _ = tokio::spawn(async move {
        let audio_handler = audio_handler;
        let queue = audio_handler.get_queue();
        let (stream, _) = audio_handler.spawn();

        loop {
            let audio_frame = rendered_audio_queue.pop().await;

            let data = audio_frame.data(0).to_type::<f32>();
            for x in data.iter() {
                queue.push(*x).await;
            }
        }
    });

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
