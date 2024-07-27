//! # ffmpeg-playbin
//! a video player buint on top of ffmpeg.
//! this player impls the [BasicPlayer] traits.
//!
//! WIP

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

mod player;
mod frame;

pub use playbin_core::*;


pub use player::*;
pub use frame::*;
