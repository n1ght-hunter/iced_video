

pub type Frame = ndarray::Array3<u8>;
pub type SampleCallback = Box<dyn FnMut(Frame) + Send  + 'static>;