use ffmpeg::format::sample::Type;

pub trait ToFFMPEG<T> {
    fn to_ffmpeg(&self) -> T;
}

impl ToFFMPEG<ffmpeg::format::Sample> for cpal::SampleFormat {
    fn to_ffmpeg(&self) -> ffmpeg::format::Sample {
        match self {
            cpal::SampleFormat::I8 => ffmpeg::format::Sample::U8(Type::Packed),
            cpal::SampleFormat::U8 => ffmpeg::format::Sample::U8(Type::Packed),
            cpal::SampleFormat::I16 => ffmpeg::format::Sample::I16(Type::Packed),
            cpal::SampleFormat::U16 => ffmpeg::format::Sample::I16(Type::Packed),
            cpal::SampleFormat::F32 => ffmpeg::format::Sample::F32(Type::Packed),
            cpal::SampleFormat::I32 => ffmpeg::format::Sample::I32(Type::Packed),
            cpal::SampleFormat::I64 => ffmpeg::format::Sample::I64(Type::Packed),
            cpal::SampleFormat::U32 => ffmpeg::format::Sample::I32(Type::Packed),
            cpal::SampleFormat::U64 => ffmpeg::format::Sample::I64(Type::Packed),
            cpal::SampleFormat::F64 => ffmpeg::format::Sample::F64(Type::Packed),
            _ => todo!(),
        }
    }
}