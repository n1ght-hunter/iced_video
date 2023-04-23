use std::path::Path;

use ffmpeg::{playbin::playbin_trait::PlayBinTrait, tracing::error};
use iced::{
    executor,
    futures::SinkExt,
    widget::{self, container},
    Application, Color, Command, Element, Length,
};
use iced_video::{
    player_handler::PlayerHandler,
    viewer::{video_view, ControlEvent},
    PlayerBackend, PlayerBuilder, PlayerMessage,
};

fn main() {
    ffmpeg::tracing::subscriber::set_global_default(
        ffmpeg::tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(ffmpeg::tracing::Level::INFO)
            .finish(),
    )
    .expect("setting default subscriber failed");

    ffmpeg::player::init::init().unwrap();

    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Frame(iced::widget::image::Handle),
}

struct App {
    frame: Option<iced::widget::image::Handle>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (App { frame: None }, Command::none())
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::channel("some rnado id", 100, |mut ouput| async move {
            let url = Path::new("assets/Finch.2021.1080p.WEBRip.x264-RARBG.mp4");
            // let url = Path::new("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4");
            let (sender, mut res) = ffmpeg::tokio::sync::mpsc::channel(100);
            let mut playbin = ffmpeg::playbin::PlayBin::new();

            playbin.set_source(url);
            playbin.set_sample_callback(move |image| {
                
                if let Err(err) = sender.blocking_send(image.into_raw_image()) {
                    error!("error: {:?}", err)
                }
            });

            playbin.play();

            while let Some(frame) = res.recv().await {
                if let Err(err) = ouput.send(Message::Frame(frame)).await {
                    println!("error: {:?}", err);
                    break;
                };
            }
            println!("end");

            loop {
                iced::futures::pending!()
            }
        })
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Frame(frame) => {
                self.frame = Some(frame);
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        container(if let Some(image) = &self.frame {
            Element::from(
                widget::Image::new(image.clone())
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
            .explain(Color::BLACK)
        } else {
            Element::from(
                widget::Image::new(iced::widget::image::Handle::from_memory([]))
                    .width(Length::Fill)
                    .height(Length::Fill),
            )
        })
        .center_x()
        .center_y()
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
