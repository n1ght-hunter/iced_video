use crate::BasicPlayer;

#[derive(Debug, Clone)]
pub enum PlayerMessage<P> {
    Frame(String, crate::image::Handle),
    Player(String, P),
}
