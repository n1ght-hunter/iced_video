use log::debug;
use std::time::Duration;

use iced::keyboard::KeyCode;
use iced_video::PlayerBackend;

use crate::helpers::component_trait::Update;

use super::KeyPressHandler;

impl Update for KeyPressHandler {
    type Message = iced::keyboard::Event;

    fn update(
        state: &mut crate::state::State,
        params: Self::Message,
    ) -> iced::Command<crate::Message> {
        if let iced::keyboard::Event::KeyPressed {
            key_code,
            modifiers,
        } = params
        {
            if let Some(player) = state.player_handler.get_player_mut("main player") {
                match key_code {
                    // File Operations
                    KeyCode::O if modifiers.is_empty() => {
                        debug!("Open a single  file");
                    }
                    KeyCode::O if modifiers.control() && modifiers.shift() => {
                        debug!("Open multiple files");
                    }
                    KeyCode::F if modifiers.control() => {
                        debug!("Open folder");
                    }

                    KeyCode::D if modifiers.control() => {
                        debug!("Open disk");
                    }
                    KeyCode::N if modifiers.control() => {
                        debug!("Open network stream");
                    }
                    KeyCode::C if modifiers.control() => {
                        debug!("Open capture device");
                    }
                    KeyCode::V if modifiers.control() => {
                        debug!("Open location copied in the clipboard");
                    }
                    KeyCode::R if modifiers.control() => {
                        debug!("Convert and save file");
                    }
                    KeyCode::S if modifiers.control() => {
                        debug!("Stream your media locally or on the internet");
                    }

                    // Program Operations
                    KeyCode::Q if modifiers.control() => {
                        debug!("close");
                    }
                    KeyCode::Q if modifiers.alt() => {
                        debug!("close");
                    }
                    KeyCode::F4 if modifiers.alt() => {
                        debug!("close");
                    }
                    KeyCode::E if modifiers.control() => {
                        debug!("Open the adjustment and effects menu");
                    }
                    KeyCode::W if modifiers.shift() => {
                        debug!("VLM Configuration");
                    }
                    KeyCode::M if modifiers.control() => {
                        debug!("Open the message screen");
                    }
                    KeyCode::P if modifiers.control() => {
                        debug!("Open the preferences menu");
                    }
                    KeyCode::F1 if modifiers.control() => {
                        debug!("Open the About menu");
                    }
                    KeyCode::F1 if modifiers.is_empty() => {
                        debug!("Open the help menu");
                    }

                    // Playing Operations
                    KeyCode::Space if modifiers.is_empty() => {
                        if let Err(err) = player.set_paused(!player.get_paused()) {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::N if modifiers.is_empty() => {
                        debug!("Next Track")
                    }
                    KeyCode::P if modifiers.is_empty() => {
                        debug!("Previous Track")
                    }

                    KeyCode::F | KeyCode::F11 if modifiers.is_empty() => {
                        debug!("Fullscreen")
                    }
                    KeyCode::H if modifiers.control() => {
                        debug!("Switch minimal interface on and off");
                    }
                    KeyCode::T if modifiers.control() => {
                        debug!("Go to a specific time of a playing media")
                    }
                    KeyCode::T if modifiers.is_empty() => {
                        debug!("Show current and remaining time information");
                    }
                    KeyCode::P if modifiers.is_empty() => {
                        debug!("go and play from the start of a file");
                        if let Err(err) = player.restart_stream() {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::S if modifiers.is_empty() => {
                        if let Err(err) = player.exit() {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Escape if modifiers.is_empty() => {
                        debug!("Exit fullscreen mode");
                    }
                    KeyCode::E if modifiers.is_empty() => {
                        if let Err(err) = player.next_frame() {
                            eprintln!("Error: {:?}", err);
                        }
                    }

                    KeyCode::L if modifiers.is_empty() => {
                        debug!("Loop the current media");
                        player.set_looping(!player.get_looping());
                    }

                    // Playing Speed
                    KeyCode::LBracket if modifiers.is_empty() => {
                        if let Err(err) = player.set_rate((player.get_rate() - 0.25).max(0.25)) {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::RBracket if modifiers.is_empty() => {
                        if let Err(err) = player.set_rate((player.get_rate() + 0.25).min(2.0)) {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Equals if modifiers.is_empty() => {
                        if let Err(err) = player.set_rate(1.0) {
                            eprintln!("Error: {:?}", err);
                        }
                    }

                    // Quick Forward and Backward
                    KeyCode::Right if modifiers.shift() => {
                        debug!("Seek forward 3 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(3))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Right if modifiers.alt() => {
                        debug!("Seek forward 10 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(10))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Right if modifiers.control() => {
                        debug!("Seek forward 60 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(60))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Right if modifiers.is_empty() => {
                        debug!("Seek forward 1 second");
                        if let Err(err) =
                            player.seek(player.get_position() + Duration::from_secs(1))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Left if modifiers.shift() => {
                        debug!("Seek backward 3 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(3))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Left if modifiers.alt() => {
                        debug!("Seek backward 10 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(10))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Left if modifiers.control() => {
                        debug!("Seek backward 60 seconds");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(60))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }
                    KeyCode::Left if modifiers.is_empty() => {
                        debug!("Seek backward 1 second");
                        if let Err(err) =
                            player.seek(player.get_position() - Duration::from_secs(1))
                        {
                            eprintln!("Error: {:?}", err);
                        }
                    }

                    // Sound and Audio Operations
                    KeyCode::Up if modifiers.is_empty() => {
                        player.set_volume((player.get_volume() + 0.1).min(1.0));
                    }
                    KeyCode::Down if modifiers.is_empty() => {
                        player.set_volume((player.get_volume() - 0.1).max(0.0));
                    }

                    KeyCode::M if modifiers.is_empty() => {
                        debug!("Mute sound on and off");
                        player.set_muted(!player.get_muted());
                    }

    

            

                    // Disc Operations
                    _ => {}
                }
            }
        }

        iced::Command::none()
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
