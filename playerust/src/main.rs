pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;
pub mod helpers;
pub mod components;

use iced::{executor};
use iced_video::BasicPlayer;
use state::State;
use subscriptions::subscriptions;
use update::{update, Message};
use view::view;

fn main() -> iced::Result {
    env_logger::init();
    iced::application(
        State::title,
        State::update,
        State::view,
    ).subscription(State::subscription).run_with(State::new)
}

pub type Element<'a> = iced::Element<'a, Message,iced::Theme, iced::Renderer>;

impl  State {

    fn subscription(&self) -> iced::Subscription<Message> {
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

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        update(self, message)
    }

    fn view(&self) -> Element {
        view(self)
    }
}
