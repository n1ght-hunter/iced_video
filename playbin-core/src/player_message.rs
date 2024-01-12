//! Player message types.
//! these are the messages that are sent to the player handler.

use crate::BasicPlayer;

/// Player message types.
#[derive(Debug, Clone)]
pub enum PlayerMessage<P> {
    /// Player frame message.
    Frame(String, crate::image::Handle),
    /// returns a new player
    Player(String, P),
}
