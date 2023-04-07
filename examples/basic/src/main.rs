use iced::{
    executor,
    widget::{self, container},
    Application, Command, Element,
};
use iced_video::{
    iced_subscription::PlayerMessage,
    player_handler::PlayerHandler,
    video_settings::VideoSettings,
    viewer::{video_view, ControlEvent},
};

fn main() {
    // uncomment to see debug messages from gstreamer
    // std::env::set_var("GST_DEBUG", "3");
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(PlayerMessage),
    ControlEvent(ControlEvent),
}

struct App {
    player_handler: PlayerHandler,
    seek: Option<u64>,
    id: String,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut player_handler = PlayerHandler::default();
        let url =
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";
        player_handler.start_player(VideoSettings::new(url).set_auto_start(true).set_uri(url));

        (
            App {
                player_handler,
                seek: None,
                id: url.to_string(),
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        self.player_handler.subscriptions().map(Message::Video)
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Video(event) => {
                if let Some((_player_id, message)) = self.player_handler.handle_event(event) {
                    println!("message: {:?}", message);
                }
            }
            Message::ControlEvent(event) => {
                if let Some(player) = self.player_handler.get_player_mut(&self.id) {
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
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let player: Element<Message> =
            if let Some(player) = self.player_handler.get_player(&self.id) {
                let frame = self.player_handler.get_frame(&self.id);
                video_view(player, frame, &Message::ControlEvent, &self.seek).into()
            } else {
                widget::Text::new("No player").size(30).into()
            };

        container(player).center_x().center_y().into()
    }
}
