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
    clippy::useless_conversion
)]
#![forbid(rust_2018_idioms, unsafe_code)]
#![allow(clippy::inherent_to_string, clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod iced_subscription;
pub mod player;
pub mod video_handler;
pub mod video_settings;
pub mod svgs;
pub mod overlay;
pub mod viewer;
