use std::path::Path;

use ffmpeg_playbin::{player, BasicPlayer};

use iced::{
    executor,
    futures::SinkExt,
    widget::{self, container, image},
    Application, Color, Task, Element, Length,
};

fn main() -> iced::Result {
    iced::application(
        App::title,
        App::update,
        App::view,
    ).run_with(App::new)
}

#[derive(Clone, Debug)]
enum Message {
    Frame(iced::widget::image::Handle),
}

struct App {
    frame: Option<iced::widget::image::Handle>,
}

impl  App {

    fn new() -> (Self, iced::Task<Message>) {
        (App { frame: None }, Task::none())
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(|| iced::stream::channel( 100, |mut ouput| async move {
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
        }))
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Frame(frame) => {
                self.frame = Some(frame);
            }
        }
        Task::none()
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
            widget::text!("No frame").into()
        })
        .center(Length::Fill)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
