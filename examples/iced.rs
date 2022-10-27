use iced::{
    executor,
    widget::{container, Image, Text, column},
    Application, Command, Theme,
};
use iced_native::{image};
use iced_pure_video_player::{VideoEvent, VideoPlayer};

fn main() {
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(VideoEvent),
}

struct App {
    video: Option<VideoPlayer>,
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
                video: None,
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
                VideoEvent::Connected(player, handle) => {
                    self.video = Some(player);
                    self.frame = handle;
                }
                VideoEvent::Disconnected => println!("Disconnected"),
                VideoEvent::FrameUpdate(handle) => self.frame = handle,
            },
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        VideoPlayer::new("https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm", false).unwrap().map(|x| Message::Video(x))
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let image = if self.frame.is_some() {
            Image::new(self.frame.clone().unwrap())
        } else {
            Image::new(image::Handle::from_pixels(0, 0, vec![]))
        };
        let text =  if let Some(video) = &self.video {
            Text::new(format!(
                "{:#?}s / {:#?}s",
                video.position().as_secs(),
                video.duration().as_secs()
            ))
        } else {
            Text::new(format!(
                "0s / 0s",
            ))
        };
        container(column![image, text]).into()
    }
}
