use iced::{
    executor,
    widget::{self, container, image, text},
    Application, Command,
};
use video_player::{
    iced_subscription::{video_subscription, SubMSG},
    player::{VideoPlayer, VideoSettings},
    viewer::{video_view, ControlEvent},
};

fn main() {
    std::env::set_var("GST_DEBUG", "3");
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(SubMSG),
    ControlEvent(ControlEvent),
}

struct App {
    video_players: Option<VideoPlayer>,
    frame: Option<image::Handle>,
    seek: Option<u64>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            App {
                video_players: None,
                frame: None,
                seek: None,
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        video_subscription(
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
                .to_string(),
            VideoSettings::default(),
        )
        .map(Message::Video)
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Video(event) => match event {
                SubMSG::Image(_id, image) => {
                    self.frame = Some(image);
                }
                SubMSG::Message(_id, message) => {
                    println!("message: {:?}", message);
                    match message {
                        _ => (),
                    }
                }
                SubMSG::Player(_id, player) => self.video_players = Some(player),
            },
            Message::ControlEvent(event) => {
                let player = self.video_players.as_mut().unwrap();

                match event {
                    ControlEvent::Play => player.set_paused_state(false),
                    ControlEvent::Pause => player.set_paused_state(true),
                    ControlEvent::ToggleMute => {
                        if player.muted() {
                            player.set_muted(false)
                        } else {
                            player.set_muted(true)
                        }
                    }
                    ControlEvent::Volume(volume) => player.set_volume(volume),
                    ControlEvent::Seek(p) => {
                        self.seek = Some(p as u64);
                    }
                    ControlEvent::Released => {
                        player.seek(self.seek.unwrap()).unwrap_or_else(|_| ());
                        self.seek = None;
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let player: iced::Element<Message> = if let Some(player) = &self.video_players {
            video_view(player, &self.frame, &Message::ControlEvent, &self.seek).into()
        } else {
            text("no vid").into()
        };

        container(widget::column![player])
            .center_x()
            .center_y()
            .into()
    }
}
