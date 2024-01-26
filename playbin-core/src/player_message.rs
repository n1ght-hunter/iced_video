//! Player message types.
//! these are the messages that are sent to the player handler.

/// Player message types.
#[derive(Debug, Clone)]
pub enum PlayerMessage<P, F = crate::image::Handle> {
    /// Player frame message.
    Frame(String, F),
    /// returns a new player
    Player(String, P),
}
