use video_player::iced_subscription::{video_subscription, VideoSettings};

use crate::{update::Message, State};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    video_subscription(
        "http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
            .to_string(),
        VideoSettings::default(),
    )
    .map(Message::Video)
}
