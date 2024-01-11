use std::{path::Path, time::Duration};

pub trait BasicPlayer {

    /// Create a new instance of the player
    fn new() -> Self;

    /// Set the source of the player
    fn set_source(&mut self, uri: &Path);

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

    fn set_mute(&self, mute: bool);

    fn get_mute(&self) -> bool;

    fn set_looping(&self, looping: bool);

    fn get_looping(&self) -> bool;

    fn seek(&self, time: Duration);

    fn get_position(&self) -> Duration;

    fn get_duration(&self) -> Duration;

    fn set_playback_rate(&self, rate: f64);

    fn get_playback_rate(&self) -> f64;

    fn restart_stream(&self);
}
