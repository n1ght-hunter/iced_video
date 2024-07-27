//! # gstreamer-playbin
//! a video player buint on top of gstreamer's playbin3.
//! this player impls the [BasicPlayer] and [AdvancedPlayer] traits.


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



mod error;
mod extra_functions;
mod player;
mod tag_convert;
mod unsafe_functions;

pub use error::*;
pub use extra_functions::*;
pub use player::*;
pub use tag_convert::*;
pub use unsafe_functions::*;