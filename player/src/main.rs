pub mod state;
pub mod subscriptions;
pub mod theme;
pub mod update;
pub mod view;

use iced::{executor, Application};
use state::State;
use subscriptions::subscriptions;
use update::{update, Message};
use view::view;

fn main() {
    // std::env::set_var("GST_DEBUG", "3");
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
        self.title.to_owned()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Message> {
        update(self, message)
    }

    fn view(&self) -> Element {
        view(self)
    }
}
