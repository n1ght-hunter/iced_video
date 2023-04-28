use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use ffmpeg::{ffi::AV_TIME_BASE, Frame, Packet, Rational};
use tokio::sync::{Mutex, Notify};
use tracing::info;

use crate::render_queue::Queue;

pub type ReaderChannelType = ReaderCommand;

#[derive(Debug)]
pub enum ReaderCommand {
    Seek(Duration),
    Pause,
    Play,
}

#[derive(Debug)]
pub enum ReaderError {
    InputCreate(ffmpeg::Error),
    NoVideoStream,
    NoAudioStream,
    ReadExhausted,
    Seek(ffmpeg::Error),
}

pub struct Reader {
    input: ffmpeg::format::context::Input,
    video_queue: Arc<Queue<(Packet, Rational)>>,
    audio_queue: Arc<Queue<(Packet, Rational)>>,
    video_stream_index: usize,
    audio_stream_index: usize,
}

impl Reader {
    pub fn new(uri: &std::path::Path) -> Result<Self, ReaderError> {
        let input = ffmpeg::format::input(&uri).map_err(ReaderError::InputCreate)?;
        let video_stream_index = input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(ReaderError::NoVideoStream)?
            .index();

        let audio_stream_index = input
            .streams()
            .best(ffmpeg::media::Type::Audio)
            .ok_or(ReaderError::NoAudioStream)?
            .index();

        Ok(Self {
            input,
            video_queue: Arc::new(Queue::new(20)),
            audio_queue: Arc::new(Queue::new(20)),
            video_stream_index,
            audio_stream_index,
        })
    }

    pub fn spawn(
        mut self,
        mut use_reader: tokio::sync::mpsc::Receiver<ReaderChannelType>,
        paused: Arc<AtomicBool>,
        unpause: Arc<Notify>,
    ) -> (
        Arc<Queue<(Packet, Rational)>>,
        Arc<Queue<(Packet, Rational)>>,
        tokio::task::JoinHandle<()>,
    ) {
        let cloned_video_queue = self.video_queue.clone();
        let cloned_audio_queue = self.audio_queue.clone();

        let handle = tokio::spawn(async move {
            loop {
                if paused.load(std::sync::atomic::Ordering::Relaxed) {
                    unpause.notified().await;
                }

                match self.read() {
                    Ok(packet) => match packet {
                        StreamPacket::Video(packet, time) => {
                            self.video_queue.push((packet, time)).await;
                        }
                        StreamPacket::Audio(packet, time) => {
                            self.audio_queue.push((packet, time)).await;
                        }
                    },
                    Err(err) => {
                        info!("read: {:?}", err);
                        break;
                    }
                }
                if let Ok(command) = use_reader.try_recv() {
                    self.handle_command(command);
                }
            }
        });

        (cloned_video_queue, cloned_audio_queue, handle)
    }

    fn handle_command(&mut self, command: ReaderChannelType) {
        match command {
            ReaderCommand::Seek(time) => {
                self.seek(time).unwrap();
            }
            ReaderCommand::Pause => {}
            ReaderCommand::Play => {}
        }
    }

    pub fn audio_parameters(&self) -> (ffmpeg::codec::Parameters, Rational) {
        let stream = self
            .input
            .streams()
            .find(|s| s.index() == self.audio_stream_index)
            .unwrap();
        (stream.parameters(), stream.time_base())
    }

    pub fn video_parameters(&self) -> (ffmpeg::codec::Parameters, Rational) {
        let stream = self
            .input
            .streams()
            .find(|s| s.index() == self.video_stream_index)
            .unwrap();
        (stream.parameters(), stream.time_base())
    }

    pub fn read(&mut self) -> Result<StreamPacket, ReaderError> {
        let mut error_count = 0;

        while error_count < 3 {
            if let Some((stream, packet)) = self.input.packets().next() {
                match stream.index() {
                    index if index == self.video_stream_index => {
                        return Ok(StreamPacket::Video(packet, stream.time_base()));
                    }
                    index if index == self.audio_stream_index => {
                        return Ok(StreamPacket::Audio(packet, stream.time_base()));
                    }
                    _ => {}
                }
            } else {
                error_count += 1;
            }
        }

        Err(ReaderError::ReadExhausted)
    }

    pub fn seek(&mut self, time: Duration) -> Result<(), ReaderError> {
        let time = time.as_micros() as i64;
        const RANGE: i64 = Duration::from_millis(500).as_micros() as i64;
        let range = time - RANGE..time + RANGE;
        self.input.seek(time, range).map_err(ReaderError::Seek)?;
        Ok(())
    }

    pub fn duration(&self) -> Duration {
        let duration = self.input.duration() as u64;
        Duration::from_micros(duration)
    }

    pub fn time_base(&self) -> Rational {
        self.input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .unwrap()
            .time_base()
    }
}

pub enum StreamPacket {
    Video(Packet, Rational),
    Audio(Packet, Rational),
}
