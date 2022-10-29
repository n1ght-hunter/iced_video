use iced::{
    executor,
    widget::{button, column, container, text, Image, Text},
    Application, Command, Event, Length, Settings, Theme,
};
use iced_native::{image, window};
use iced_pure_video_player::{GSTMessage, VideoEvent, VideoPlayer};

fn main() {
    dotenv::dotenv().unwrap();
    App::run(Settings {
        exit_on_close_request: false,
        ..Default::default()
    })
    .unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(VideoEvent),
    PausePlay,
    EventOccurred(iced_native::Event),
}

struct App {
    video: Option<VideoPlayer>,
    frame: Option<image::Handle>,
    should_exit: bool,
    starting_exit: bool,
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
                should_exit: false,
                starting_exit: false,
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
                VideoEvent::Connected(player, channel) => {
                    if let Some(message) = channel {
                        match message {
                            iced_pure_video_player::GSTChannel::Image(image) => {
                                println!("connected");

                                self.video = Some(player);
                                self.frame = Some(image);
                            }
                            iced_pure_video_player::GSTChannel::Message(message) => (),
                        }
                    }
                }
                VideoEvent::Disconnected => println!("Disconnected"),
                VideoEvent::FrameUpdate(channel) => {
                    if let Some(message) = channel {
                        match message {
                            iced_pure_video_player::GSTChannel::Image(image) => {
                                self.frame = Some(image);
                            }
                            iced_pure_video_player::GSTChannel::Message(message) => match message {
                                GSTMessage::Eos => {
                                    if self.starting_exit == true {
                                        self.should_exit = true;
                                    }
                                }
                                _ => (),
                            },
                        }
                    }
                }
            },
            Message::PausePlay => {
                if self.video.is_some() {
                    let video = self.video.as_mut().unwrap();
                    if video.paused {
                        video.set_paused(false);
                    } else {
                        video.set_paused(true);
                    }
                }
            }
            Message::EventOccurred(event) => {
                println!("event: {:?}", event);
                if let Event::Window(window::Event::CloseRequested) = event {
                    println!("close");
                    if let Some(video) = self.video.as_mut() {
                        println!("close2");
                        video.exit().unwrap();
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::batch(vec![
            VideoPlayer::new("https://www.freedesktop.org/software/gstreamer-sdk/data/media/sintel_trailer-480p.webm", false).unwrap().map(|x| Message::Video(x)),
            iced_native::subscription::events().map(Message::EventOccurred),
        ])
    }

    fn should_exit(&self) -> bool {
        self.should_exit
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let image = if self.frame.is_some() {
            Image::new(self.frame.clone().unwrap())
                .height(Length::Units(self.video.as_ref().unwrap().height as u16))
                .width(Length::Units(self.video.as_ref().unwrap().width as u16))
        } else {
            Image::new(image::Handle::from_pixels(0, 0, vec![]))
        };
        let button = button(text("pause/play")).on_press(Message::PausePlay);

        let text = if let Some(video) = &self.video {
            Text::new(format!(
                "{:#?}s / {:#?}s",
                video.position().as_secs(),
                video.duration().as_secs()
            ))
        } else {
            Text::new(format!("0s / 0s",))
        };
        container(column![image, text, button])
            .center_x()
            .center_y()
            .into()
    }
}
