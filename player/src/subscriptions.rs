use video_player::{iced_subscription::video_subscription, player::VideoSettings};

use crate::{update::Message, State};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    video_subscription("testing id", VideoSettings::default()).map(Message::Video)
}
// http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4
