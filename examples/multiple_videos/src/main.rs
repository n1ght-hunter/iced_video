use iced::{
    executor,
    widget::{self, button, container, scrollable},
    Application, Task,
};
use iced_video::{
     viewer::ControlEvent, PlayerBuilder, PlayerHandler, PlayerMessage,
     AdvancedPlayer, BasicPlayer
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
    Video(PlayerMessage),
    ControlEvent(String, ControlEvent),
}

struct App {
    player_handler: PlayerHandler,
    seek: Option<u64>,
}

impl App {

    fn new() -> (Self, iced::Task<Message>) {
        let mut player_handler = PlayerHandler::default();
        let urls = [
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4",
        ];

        urls.into_iter()
            .for_each(|uri| player_handler.start_player(PlayerBuilder::new(uri).set_uri(uri)));

        (
            App {
                player_handler,
                seek: None,
            },
            Task::none(),
        )
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        self.player_handler.subscriptions().map(Message::Video)
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::Video(event) => {
                self.player_handler.handle_event(event);
            }

            Message::ControlEvent(uri, event) => {
                if let Some(player) = self.player_handler.get_player_mut(&uri) {
                    match event {
                        ControlEvent::Play => player.play(),
                        ControlEvent::Pause => player.pause(),
                        ControlEvent::ToggleMute => {
                            if player.get_muted() {
                                player.set_muted(false)
                            } else {
                                player.set_muted(true)
                            }
                        }
                        ControlEvent::Volume(volume) => {
                            // player.set_volume(volume)
                        }
                        ControlEvent::Seek(p) => {
                            self.seek = Some(p as u64);
                        }
                        ControlEvent::Released => {
                            player
                                .seek(std::time::Duration::from_secs(self.seek.unwrap()))
                                .unwrap_or_else(|err| println!("Error seeking: {:?}", err));
                            self.seek = None;
                        }
                    }
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let players = self
            .player_handler
            .players_and_images()
            .iter()
            .map(|(id, player, handle)| {
                let image = button(
                    iced::widget::image((*handle).clone())
                        .height(480)
                        .width(480),
                )
                .on_press(Message::ControlEvent(
                    (*id).clone(),
                    if player.is_playing() {
                        ControlEvent::Pause
                    } else {
                        ControlEvent::Play
                    },
                ));

                container(image).into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        container(scrollable(widget::Column::with_children(players)))
            .center(iced::Length::Fill)
            .into()
    }
}
