use std::{path::Path, time::Duration};

pub trait PlayBinTrait<T> {
    fn new() -> Self;

    fn set_source(&mut self, uri: &Path);

    fn set_sample_callback(&self, sample_callback: T);

    fn play(&self);

    fn pause(&self);

    fn seek(&self, time: Duration);

    fn stop(&mut self);
}
