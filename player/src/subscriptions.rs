use video_player::iced_subscription::{video_subscription, VideoSettings};

use crate::{update::Message, State};

pub fn subscriptions(state: &State) -> iced::Subscription<Message> {
    if let Some(uri) = &state.uri {
        video_subscription(uri.clone(), VideoSettings::default()).map(Message::Video)
    } else {
        iced::Subscription::none()
    }
}
// http://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4
