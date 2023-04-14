//! # Iced Player
//! A video player built with [iced] and gstreamer.
//! this is a WIP project, so expect bugs and missing features.
//!
//! this is a simple to use viper player api, that can be used to build a video player with iced.
//!
//! ## Features
//! - [x] Play videos from local files and streams
//! - [] Play videos in fullscreen
//! - [x] has a overlay for video controls
#![deny(
    missing_debug_implementations,
    missing_docs,
    unused_results,
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default,
    clippy::useless_conversion,
    unsafe_code
)]
#![forbid(rust_2018_idioms)]
#![allow(clippy::inherent_to_string, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod backends;
mod player_builder;

pub mod helpers;
pub mod overlay;
pub mod player_handler;
pub mod viewer;

pub use helpers::player_backend::PlayerBackend;
pub use player_builder::PlayerBuilder;

#[cfg(all(feature = "gstreamer", not(target_arch = "wasm32")))]
pub use backends::gstreamer::tag_convert;

/// gstreamer modules that are only available when the gstreamer feature is enabled
#[cfg(all(feature = "gstreamer", not(target_arch = "wasm32")))]
pub mod gstreamer {

    pub use gst::{MessageView, DateTime};
}

#[cfg(all(feature = "gstreamer", not(target_arch = "wasm32")))]
pub use backends::gstreamer::GstreamerBackend as Player;

#[cfg(all(feature = "gstreamer", not(target_arch = "wasm32")))]
pub use backends::gstreamer::GstreamerMessage as PlayerMessage;
