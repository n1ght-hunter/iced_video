use std::time::Duration;

use ffmpeg::{ffi::AV_TIME_BASE, Frame, Packet, Rational};

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
            video_stream_index,
            audio_stream_index,
        })
    }

    pub fn use_input<T>(
        &mut self,
        input_function: impl FnOnce(&mut ffmpeg::format::context::Input) -> T,
    ) -> T {
        input_function(&mut self.input)
    }

    pub fn read(&mut self) -> Result<(Packet, Rational), ReaderError> {
        let mut error_count = 0;
        loop {
            match self.input.packets().next() {
                Some((stream, packet)) => {
                    if stream.index() == self.video_stream_index {
                        return Ok((packet, stream.time_base()));
                    }
                }
                None => {
                    error_count += 1;
                    if error_count > 3 {
                        return Err(ReaderError::ReadExhausted);
                    }
                }
            }
        }
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
