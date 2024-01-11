use std::{path::Path, time::Duration};

use crate::{PlayerBuilder, PlayerMessage};

pub trait BasicPlayer {
    type Error;

    /// Create a new instance of the player
    fn create(player_builder: PlayerBuilder) -> (Self, smol::channel::Receiver<PlayerMessage<Self>>)
    where
        Self: Sized;

    /// Set the source of the player
    fn set_source(&mut self, uri: &Path) -> Result<(), Self::Error>
    where
        Self: Sized;

    /// Get the source of the player
    fn get_source(&self) -> Option<String>;

    /// pause the player
    fn pause(&self);

    /// play the player
    fn play(&self);

    /// get if the player is playing
    fn is_playing(&self) -> bool;

    /// stop the player and close all resources
    fn stop(&mut self);
}

pub trait AdvancedPlayer: BasicPlayer {
    fn set_volume(&self, volume: f64);

    fn get_volume(&self) -> f64;

    fn set_muted(&self, mute: bool);

    fn get_muted(&self) -> bool;

    fn set_looping(&self, looping: bool);

    fn get_looping(&self) -> bool;

    fn seek(&self, time: Duration) -> Result<(), Self::Error>;

    fn get_position(&self) -> Duration;

    fn get_duration(&self) -> Duration;

    fn set_playback_rate(&self, rate: f64) -> Result<(), Self::Error>;

    fn get_playback_rate(&self) -> f64;

    fn restart_stream(&self) -> Result<(), Self::Error>;
}
