use log::debug;
use std::time::Duration;

use iced::{keyboard::{key::Named, Key}, Task};
use iced_video::{BasicPlayer, AdvancedPlayer};

use crate::{helpers::{component_trait::Update, open_file::open_file}, update::{Message, menu_event::MenuEvent}};

use super::KeyPressHandler;

impl Update for KeyPressHandler {
    type Message = iced::keyboard::Event;

    fn update(
        state: &mut crate::state::State,
        params: Self::Message,
    ) -> iced::Task<crate::Message> {
        if let iced::keyboard::Event::KeyPressed {
            key,
            modifiers,
            ..
        } = params
        {
            if let Some(player) = state.player_handler.get_player_mut("main player") {
                match key.as_ref() {
                    // File Operations
                    Key::Character("O") if modifiers.is_empty() => {
                        debug!("Open a single file");
                        return Task::perform(async { open_file().await }, |f| {
                            Message::MenuEvent(MenuEvent::OpenFile(f))
                        });
                    }
                    // Playing Operations
                    Key::Named(Named::Space) if modifiers.is_empty() => {
                        if player.is_playing() {
                            player.pause();
                        } else {
                            player.play();
                        }
                    }

                    Key::Character("L") if modifiers.is_empty() => {
                        debug!("Loop the current media");
                        player.set_looping(!player.get_looping());
                    }

                    // Playing Speed
                    Key::Character("[") if modifiers.is_empty() => {
                        if let Err(err) = player.set_playback_rate((player.get_playback_rate() - 0.25).max(0.25)) {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Character("]") if modifiers.is_empty() => {
                        if let Err(err) = player.set_playback_rate((player.get_playback_rate() + 0.25).min(2.0)) {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Character("=") if modifiers.is_empty() => {
                        if let Err(err) = player.set_playback_rate(1.0) {
                            eprintln!("Error: {:?}", err);
                        }
                    }

                    // Quick Forward and Backward
                    Key::Named(Named::ArrowRight) if modifiers.shift() => {
                        debug!("Seek forward 3 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(3))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowRight) if modifiers.alt() => {
                        debug!("Seek forward 10 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(10))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowRight) if modifiers.control() => {
                        debug!("Seek forward 60 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(60))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowRight) if modifiers.is_empty() => {
                        debug!("Seek forward 1 second");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(1))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowLeft) if modifiers.shift() => {
                        debug!("Seek backward 3 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(3))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowLeft) if modifiers.alt() => {
                        debug!("Seek backward 10 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(10))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowLeft) if modifiers.control() => {
                        debug!("Seek backward 60 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(60))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    Key::Named(Named::ArrowLeft) if modifiers.is_empty() => {
                        debug!("Seek backward 1 second");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(1))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }

                    // Sound and Audio Operations
                    Key::Named(Named::ArrowUp) if modifiers.is_empty() => {
                        player.set_volume((player.get_volume() + 0.1).min(1.0));
                    }
                    Key::Named(Named::ArrowDown) if modifiers.is_empty() => {
                        player.set_volume((player.get_volume() - 0.1).max(0.0));
                    }

                    Key::Character("M") if modifiers.is_empty() => {
                        debug!("Mute sound on and off");
                        player.set_muted(!player.get_muted());
                    }

                    // Disc Operations
                    _ => {}
                }
            }
        }

        iced::Task::none()
    }
}

// File Operations

// CTRL + O : Open a single  file
// CTRL + SHIFT + O : Open multiple files
// CTRL + F : Open folder
// CTRL + D : Open disk
// CTRL + N : Open network stream
// CTRL + C : Open capture device
// CTRL + V : Open location copied in the clipboard
// CTRL + R : Convert and save file
// CTRL + S : Stream your media locally or on the internet

// Program Operations

// CTRL + Q or ALT + F4 or ALT + Q : Quit
// CTRL + E : Open the adjustment and effects menu
// CTRL + SHIFT + W : VLM Configuration
// CTRL + M : Open the message screen
// CTRL + P : Open the preferences menu
// F1 : Help
// SHIFT + F1 : About

// Playing Operations

// SPACE : Play and Pause a file
// N : Next Track
// P : Previous Track
// F or F11 or Mouse Double Click : Full screen mode on and off
// CTRL + H : Switch minimal interface on and off
// T : Show current and remaining time information
// CTRL + T : Go to a specific time of a playing media
// P : Go and play from the start of a file
// S : Stop movie
// Esc : Full screen exit
// E : Switch to next frame
// L : Loop off, one or all
// R : Random on and off

// Subtitles

// G : Decrease subtitle delay
// H : Increase subtitle delay
// V : Cycle through subtitles

// Playing Speed

// [ : Decrease playing speed
// ] : Increase playing speed
// = : Normal playing speed

// Quick Forward and Backward

// SHIFT + RIGHT ARROW : 3 seconds forward
// SHIFT + LEFT ARROW : 3 seconds backward
// ALT + RIGHT ARROW : 10 seconds forward
// ALT + LEFT ARROW : 10 seconds backward
// CTRL + RIGHT ARROW : 1 minute forward
// CTRL + LEFT ARROW : 1 minute backward

// Display Options

// A: Cycle aspect ratio
// C : Cycle through crop display area
// Z : Cycle through zoom mode
// D : Deinterlace On and Off
// O : Switch to original size of the video
// W : Wallpaper mode on (with DirectX output)

// Sound and Audio Operations

// CTRL + UP ARROW or Mouse Scroll Up : Volume Up
// CTRL + DOWN ARROW or Mouse Scroll Down : Volume Down
// J : Decrease audio delay
// K : Increase audio delay
// B : Cycle through available audio tracks
// M : Mute sound on and off
// Shift + A : Cycle through audio devices

// Media Information

// CTRL + I : View and edit media information like title, artist, album
// CTRL + J : View codec information like your media dimensions, audio and video codecs

// Playlist Operations

// CTRL + L : Switch to playlist or back to media
// CTRL + Y : Save current playlist to a file
// CTRL + B : Create, delete and clear bookmarks menu.

// Disc Operations

// Shift + B : Next title
// Shift + O : Previous title
// Shift + N : Next chapter
// Shift + P : Previous chapter
// Shift + M : Disk Menu
// Shift + H : History forward
// Shift + G : History back

// Miscellaneous

// Shift + R : Start and stop recording
// Shift + S : Take snapshot
// ALT + C : Crop from bottom
// ALT+ SHIFT + C : Uncrop from bottom
// ALT + D : Crop from left
// ALT + SHIFT + D : Uncrop from left
// ALT + F : Crop from right
// ALT + SHIFT + F : Uncrop from right
// ALT + R : Crop from top
// ALT + SHIFT + R : Uncrop from top

// File Menu Operations

// ALT + M : Open media menu
// ALT + L : Open playback menu
// ALT + A : Open audio menu
// ALT + V : Open video menu
// ALT + S : Open subtitle menu
// ALT + O : Open tools menu
// ALT + I : Open view menu
// ALT + H : Open help menu
