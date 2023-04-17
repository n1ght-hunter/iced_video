use std::path::Path;

use ffmpeg::playbin::playbin_trait::PlayBinTrait;
use iced::{
    executor,
    widget::{self, container},
    Application, Command, Element, futures::SinkExt,
};
use iced_video::{
    player_handler::PlayerHandler,
    viewer::{video_view, ControlEvent},
    PlayerBackend, PlayerBuilder, PlayerMessage,
};

fn main() {
    // uncomment to see debug messages from gstreamer
    // std::env::set_var("GST_DEBUG", "3");
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
        iced::subscription::channel(
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            100,
            |mut ouput| async move {
                let url = Path::new("http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4");
                let (sender, mut res) = ffmpeg::tokio::sync::mpsc::unbounded_channel();
                let playbin = ffmpeg::playbin::PlayBin::new(url, Box::new(move |frmae| {
                    sender.send(frmae).unwrap();
                }));
                
                loop {
                    if let Some(frame) = res.recv().await {
                        let frame = frame.iter().map(|x| *x).collect::<Vec<u8>>();
                        ouput.send(Message::Frame(iced::widget::image::Handle::from_memory(frame)));
                    }
                }
            },
        )
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
            widget::Image::new(image.clone())
        } else {
            widget::Image::new(iced::widget::image::Handle::from_memory([]))
        })
        .center_x()
        .center_y()
        .into()
    }
}
