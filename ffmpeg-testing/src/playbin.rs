use crate::player::error::FFMPEGPLayerErros;

use self::types::SampleCallback;

pub mod decoder;
pub mod playbin_trait;
pub mod types;

pub struct PlayBin {
    player: Player,
}

impl playbin_trait::PlayBinTrait for PlayBin {
    fn new(uri: &std::path::Path, sample_callback: SampleCallback) -> Self {
        Self {
            player: Player::new(uri, sample_callback),
        }
    }

    fn play(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }

    fn seek(&self) {
        todo!()
    }

    fn stop(&self) {
        todo!()
    }
}

struct Thread {}

impl Thread {
    pub fn new(uri: &std::path::Path, mut sample_callback: SampleCallback) -> std::thread::JoinHandle<Result<(), FFMPEGPLayerErros>> {
        let uri = uri.to_path_buf();
        let handle = std::thread::spawn(move || -> Result<(), FFMPEGPLayerErros> {
            let mut input = ffmpeg::format::input(&uri).unwrap();
            let mut decoder = decoder::Decoder::new(
                ffmpeg::format::Pixel::RGB24,
                input.streams().best(ffmpeg::media::Type::Video).unwrap(),
            )
            .unwrap();

            loop {
                let packet = input.packets().next();
                if let Some((_, packet)) = packet {
                    if let Ok(frame) = decoder.decode_rgb(&packet) {
                        (*sample_callback)(frame);
                    }
                }
            }


        });
        
        handle
    }
}

struct Player {
    thread: std::thread::JoinHandle<Result<(), FFMPEGPLayerErros>>,
}

impl Player {
    pub fn new(uri: &std::path::Path, sample_callback: SampleCallback) -> Self {
        Self {
            thread: Thread::new(uri, sample_callback),
        }
    }
}
