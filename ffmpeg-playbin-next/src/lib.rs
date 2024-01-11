pub mod playbins;
pub mod player;


// // Work around https://github.com/zmwangx/rust-ffmpeg/issues/102
// #[derive(derive_more::Deref, derive_more::DerefMut)]
// struct Rescaler(ffmpeg::software::scaling::Context);
// unsafe impl std::marker::Send for Rescaler {}

// fn rgba_rescaler_for_frame(frame: &ffmpeg::util::frame::Video) -> Rescaler {
//     Rescaler(
//         ffmpeg::software::scaling::Context::get(
//             frame.format(),
//             frame.width(),
//             frame.height(),
//             ffmpeg::format::Pixel::RGB24,
//             frame.width(),
//             frame.height(),
//             ffmpeg::software::scaling::Flags::BILINEAR,
//         )
//         .unwrap(),
//     )
// }
