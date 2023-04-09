//! The player backend trait and structs
//! all player backends must implement this trait

/// the player trait
pub trait PlayerBackend {
    /// the error type
    type Error: Send + Sync + 'static;
    /// Sets the source of the player
    /// # Arguments
    /// * `uri` - the uri to set the source to
    /// # Errors
    /// * `glib::Error` - if the uri is invalid
    fn set_source(&mut self, uri: &str) -> Result<(), Self::Error>;

    /// get the uri of the current source
    /// # Returns
    /// * `Option<String>` - the uri of the current source
    fn get_source(&self) -> Option<String>;

    /// set the volume multiplier of the player
    /// # Arguments
    /// * `volume` - the volume multiplier to set the player to (0.0 - 1.0) `0.0` = muted, `1.0` = full volume
    fn set_volume(&mut self, volume: f64);

    /// get the volume multiplier of the player
    /// # Returns
    /// * `f64` - the volume multiplier of the player (0.0 - 1.0) `0.0` = muted, `1.0` = full volume
    fn get_volume(&self) -> f64;

    /// set the audio mute state of the player
    /// # Arguments
    /// * `mute` - the mute state to set the player to this does not affect the volume multiplier
    fn set_muted(&mut self, mute: bool);

    /// get the audio mute state of the player
    /// # Returns
    /// * `bool` - the mute state of the player
    fn get_mute(&self) -> bool;

    /// set the looping state of the player
    /// # Arguments
    /// * `looping` - the looping state to set the player to
    fn set_looping(&mut self, looping: bool);

    /// get the looping state of the player
    /// # Returns
    /// * `bool` - the looping state of the player
    fn get_looping(&self) -> bool;

    /// set the paused state of the player
    /// # Arguments
    /// * `paused` - the paused state to set the player to
    fn set_paused(&mut self, paused: bool) -> Result<(), Self::Error>;

    /// get the paused state of the player
    /// # Returns
    /// * `bool` - the paused state of the player
    fn get_paused(&self) -> bool;

    /// seek to a position in the video
    /// # Arguments
    /// * `position` - the position to seek to in nanoseconds
    /// # Errors
    /// * `glib::Error` - if the position is invalid
    fn seek(&mut self, position: u64) -> Result<(), Self::Error>;

    /// get the current position of the video
    /// # Returns
    /// * `std::time::Duration` - the current position of the video
    fn get_position(&self) -> std::time::Duration;

    /// get the duration of the video
    /// # Returns
    /// * `std::time::Duration` - the duration of the video
    fn get_duration(&self) -> std::time::Duration;

    /// get the gstreamer bus
    /// # Returns
    /// * `&gst::Bus` - the gstreamer bus
    fn get_bus(&self) -> &gst::Bus;

    /// send exit event to the player
    /// # Errors
    /// * `glib::Error` - if the player is already stopped
    fn exit(&mut self) -> Result<(), Self::Error>;

    /// restart the stream
    fn restart_stream(&mut self) -> Result<(), Self::Error>;
}
