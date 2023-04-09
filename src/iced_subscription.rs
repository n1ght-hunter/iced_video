//! This module contains the subscription for the video player. this will update the frame with the new video frame.

use iced::subscription;
use tokio::sync::mpsc;

use crate::{PlayerBuilder, PlayerMessage};

/// The Subscription state.
#[derive(Debug)]
enum PlayerSubscription {
    Starting(PlayerBuilder),
    Next(mpsc::UnboundedReceiver<PlayerMessage>),
}

/// The subscription for the video player.
pub fn video_subscription(settings: PlayerBuilder) -> iced::Subscription<PlayerMessage> {
    subscription::unfold(
        settings.id.clone(),
        PlayerSubscription::Starting(settings),
        |state| async move {
            match state {
                PlayerSubscription::Starting(settings) => {
                    let (player, receiver) = settings.build();
                    (Some(player), PlayerSubscription::Next(receiver))
                }
                PlayerSubscription::Next(mut stream) => {
                    let item = stream.recv().await.unwrap();
                    (Some(item), PlayerSubscription::Next(stream))
                }
            }
        },
    )
}
