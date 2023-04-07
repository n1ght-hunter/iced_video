use iced_video::{iced_subscription::video_subscription, player::VideoSettings};

use crate::{update::Message, State};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    video_subscription(
        "testing id",
        VideoSettings {
            auto_start: true,
            ..VideoSettings::default()
        },
    )
    .map(Message::Video)
}
// http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4
