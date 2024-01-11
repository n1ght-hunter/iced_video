use std::path::Path;

use ffmpeg_playbin::{player, BasicPlayer};

use iced::{
    executor,
    futures::SinkExt,
    widget::{self, container, image},
    Application, Color, Command, Element, Length,
};

fn main() {
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
            let (mut player, reciver) = player::Player::start();

            let url = Path::new(
                "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            );
            player.set_source(url);

            loop {
                if let Ok(frame) = reciver.recv().await {
                    match frame {
                        player::Event::Frame(mut frame) => {
                            let pixels = ffmpeg_playbin::frame_to_image_handle(&mut frame);
                            if let Err(err) = ouput.send(Message::Frame(pixels)).await {
                                println!("error: {:?}", err);
                                continue;
                            };
                        }
                    }
                } else {
                    iced::futures::pending!()
                }
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
