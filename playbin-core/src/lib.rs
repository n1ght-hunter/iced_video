
//! Playbin core is a lib for providing a simple to use different video player apis, that can be used to build varous video players.


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



mod player_traits;
mod player_builder;
mod player_message;

pub use iced::widget::image;
pub use smol;

pub use player_traits::*;
pub use player_builder::*;
pub use player_message::*;
