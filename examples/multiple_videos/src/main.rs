use std::collections::HashMap;

use iced::{
    executor,
    widget::{self, button, container, image, scrollable},
    Application, Command, Length, Subscription,
};
use video_player::{
    iced_subscription::{video_subscription, SubMSG},
    player::{VideoPlayer, VideoSettings},
    viewer::ControlEvent,
};

fn main() {
    std::env::set_var("GST_DEBUG", "3");
    App::run(Default::default()).unwrap();
}

#[derive(Clone, Debug)]
enum Message {
    Video(SubMSG),
    ControlEvent(String, ControlEvent),
}

struct App {
    players: HashMap<String, (Option<VideoPlayer>, Option<image::Handle>)>,
    seek: Option<u64>,
}

impl Application for App {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let urls = [
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ElephantsDream.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4",
            "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/ForBiggerEscapes.mp4",
        ];
        let mut players = HashMap::new();
        urls.into_iter().for_each(|gif| {
            players.insert(gif.to_string(), (None, None));
        });

        (
            App {
                players,
                seek: None,
            },
            Command::none(),
        )
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let subscriptions = self
            .players
            .iter()
            .map(|(uri, _player)| {
                video_subscription(uri.to_string(), VideoSettings::default()).map(Message::Video)
            })
            .collect::<Vec<Subscription<Message>>>();

        iced::Subscription::batch(subscriptions)
    }

    fn title(&self) -> String {
        String::from("Video Player")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Video(event) => match event {
                SubMSG::Image(id, image) => {
                    if let Some((_self_player, self_image)) = self.players.get_mut(&id) {
                        *self_image = Some(image);
                    }
                }
                SubMSG::Message(_id, message) => {
                    println!("message: {:?}", message);
                }
                SubMSG::Player(id, player) => {
                    if let Some((self_player, _image)) = self.players.get_mut(&id) {
                        *self_player = Some(player);
                    }
                }
            },
            Message::ControlEvent(uri, event) => {
                if let Some((player, _self_image)) = self.players.get_mut(&uri) {
                    if let Some(player) = player {
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
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<Message> {
        let players = self
            .players
            .iter()
            .map(|(uri, (player, frame))| {
                let image: iced::Element<Message> = if let Some(handle) = frame {
                    button(
                        iced::widget::image(handle.clone())
                            .height(Length::Units(480))
                            .width(Length::Units(480)),
                    )
                    .on_press(Message::ControlEvent(
                        uri.clone(),
                        if player.is_some() && player.as_ref().unwrap().paused() {
                            ControlEvent::Play
                        } else {
                            ControlEvent::Pause
                        },
                    ))
                    .into()
                } else {
                    iced::widget::image(image::Handle::from_pixels(480, 480, vec![])).into()
                };
                container(image).into()
            })
            .collect::<Vec<iced::Element<Message>>>();
        container(scrollable(widget::Column::with_children(players)))
            .center_x()
            .center_y()
            .into()
    }
}
