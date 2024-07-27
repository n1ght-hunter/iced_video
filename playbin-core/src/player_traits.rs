use std::{path::PathBuf, time::Duration};

use crate::{PlayerBuilder, PlayerMessage};


/// Basic player trait
/// this trait is used to create a player with a given backend
/// it impls functions needed for a basic player
pub trait BasicPlayer {
    /// Error type of the player
    type Error;

    /// Create a new instance of the player
    fn create(player_builder: PlayerBuilder) -> (Self, smol::channel::Receiver<PlayerMessage<Self>>)
    where
        Self: Sized;

    /// Set the source of the player
    fn set_source(&mut self, uri: &PathBuf) -> Result<(), Self::Error>
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


/// Advanced player trait
/// this trait extends the basic player trait
/// it impls functions needed for a advanced player
pub trait AdvancedPlayer: BasicPlayer {
    /// Set the volume of the player
    fn set_volume(&self, volume: f64);

    /// Get the volume of the player
    fn get_volume(&self) -> f64;

    /// Set the mute state of the player
    fn set_muted(&self, mute: bool);

    /// Get the mute state of the player
    fn get_muted(&self) -> bool;

    /// Set the looping state of the player
    fn set_looping(&self, looping: bool);

    /// Get the looping state of the player
    fn get_looping(&self) -> bool;

    /// Seek to a given time
    fn seek(&self, time: Duration) -> Result<(), Self::Error>;

    /// Get the current position of the player
    fn get_position(&self) -> Duration;

    /// Get the duration of the player
    fn get_duration(&self) -> Duration;

    /// Set the playback rate of the player
    fn set_playback_rate(&self, rate: f64) -> Result<(), Self::Error>;

    /// Get the playback rate of the player
    fn get_playback_rate(&self) -> f64;

    /// restart the stream usually done by seeking to 0
    fn restart_stream(&self) -> Result<(), Self::Error>;
}
