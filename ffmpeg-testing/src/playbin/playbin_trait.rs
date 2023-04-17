use std::path::Path;

use super::types::{Frame, SampleCallback};

pub trait PlayBinTrait {
    fn new(uri: &Path, sample_callback: SampleCallback) -> Self;

    fn play(&self);

    fn pause(&self);

    fn seek(&self);

    fn stop(&self);
}
