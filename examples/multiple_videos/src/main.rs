use iced::{
    executor,
    widget::{self, button, container, scrollable},
    Application, Command, Length,
};
use video_player::{
    iced_subscription::PlayerMessage, video_handler::PlayerHandler, video_settings::VideoSettings,
    viewer::ControlEvent,
};

fn main() {
    // uncomment to see debug messages from gstreamer
    // std::env::set_var("GST_DEBUG", "3");
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(PlayerMessage),
    ControlEvent(String, ControlEvent),
}

struct App {
    player_handler: PlayerHandler,
    seek: Option<u64>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut player_handler = PlayerHandler::default();
        let urls = [
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4",
        ];

        urls.into_iter()
            .for_each(|uri| player_handler.start_player(VideoSettings::new(uri).set_uri(uri)));

        (
            App {
                player_handler,
                seek: None,
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
                if let Some((player_id, message)) = self.player_handler.handle_event(event) {
                    println!("message: {:?}", message);
                }
            }

            Message::ControlEvent(uri, event) => {
                if let Some(player) = self.player_handler.get_player(&uri) {
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
        let players = self
            .player_handler
            .players_and_images()
            .iter()
            .map(|(id, player, handle)| {
                let image = button(
                    iced::widget::image((*handle).clone())
                        .height(Length::Units(480))
                        .width(Length::Units(480)),
                )
                .on_press(Message::ControlEvent(
                    (*id).clone(),
                    if player.paused() {
                        ControlEvent::Play
                    } else {
                        ControlEvent::Pause
                    },
                ));

                container(image).into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        container(scrollable(widget::Column::with_children(players)))
            .center_x()
            .center_y()
            .into()
    }
}
