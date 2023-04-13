pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;
pub mod helpers;
pub mod componets;

use iced::{executor, Application};
use iced_video::PlayerBackend;
use state::State;
use subscriptions::subscriptions;
use update::{update, Message};
use view::view;

fn main() {
    std::env::set_var("GST_DEBUG", "3");
    env_logger::init();
    State::run(Default::default()).unwrap();
}

pub type Element<'a> = iced::Element<'a, Message, iced::Renderer<theme::Theme>>;

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
        if let Some(iced_video::tag_convert::Tag::Title(title)) = self.tags.get("title") {
            format!("{} - {}", title, self.title)
        } else if let Some(player) = self.player_handler.get_player("main player") {
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
