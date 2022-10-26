use iced::{
    executor,
    widget::{Column, Image, Text},
    Application, Command, Theme,
};
use iced_native::image;
use iced_pure_video_player::{VideoEvent, VideoPlayer};

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(VideoEvent),
}

struct App {
    video: VideoPlayer,
    frame: Option<image::Handle>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                video: VideoPlayer::new("https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm", false).unwrap(),
                frame: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Video(event) => match event {
                VideoEvent::Connected => println!("connect"),
                VideoEvent::Disconnected => println!("Disconnected"),
                VideoEvent::FrameUpdate(handle) => self.frame = handle,
            },
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.video.subscription().map(|x| Message::Video(x))
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let frame = if self.frame.is_some() {
            Image::new(self.frame.clone().unwrap())
        } else {
            Image::new(image::Handle::from_pixels(0, 0, vec![]))
        };
        Column::new()
            .push(frame)
            .push(Text::new(format!(
                "{:#?}s / {:#?}s",
                self.video.position().as_secs(),
                self.video.duration().as_secs()
            )))
            .into()
    }
}
