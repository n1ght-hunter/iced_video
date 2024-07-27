pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;
pub mod helpers;
pub mod components;

use iced::{executor, Application};
use iced_video::BasicPlayer;
use state::State;
use subscriptions::subscriptions;
use update::{update, Message};
use view::view;

fn main() {
    env_logger::init();
    State::run(Default::default()).unwrap();
}

pub type Element<'a> = iced::Element<'a, Message,theme::Theme, iced::Renderer>;

impl Application for State {
    type Executor = executor::Default;

    type Message = Message;

    type Theme = theme::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        State::new()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        subscriptions(self)
    }

    fn title(&self) -> String {
        if let Some(player) = self.player_handler.get_player("main player") {
            let uri = player.get_source();
            if let Some(uri) = uri {
                let uri = uri.split("\\").last().unwrap_or(&uri);
                format!("{} - {}", uri, self.title)
            } else {
                self.title.to_owned()
            }
        } else {
            self.title.to_owned()
        }
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element {
        view(self)
    }
}
