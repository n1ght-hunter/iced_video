// extern crate ffmpeg_next as ffmpeg;

pub mod playbin;
pub mod player;

use playbin::PlayBin;
use player::error::FFMPEGPLayerErros;
use std::path::Path;

use tracing::Level;

use crate::playbin::playbin_trait::PlayBinTrait;

#[tokio::main]
async fn main() -> Result<(), FFMPEGPLayerErros> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish(),
    )
    .expect("setting default subscriber failed");

    player::init::init()?;

    let uri = Path::new("Finch.2021.1080p.WEBRip.x264-RARBG.mp4");
    println!("uri: {:?}", uri);

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut playbin = PlayBin::new(
        uri,
        Box::new(move |frame| {
            tx.send(frame).unwrap();
        }),
    );

    while let Some(frame) = rx.recv().await {
        println!("frame: {:?}", frame.len());
    }

    // playbin.spawn_player();

    println!("playbin");

    // playbin.play()?;

    Ok(())
}

