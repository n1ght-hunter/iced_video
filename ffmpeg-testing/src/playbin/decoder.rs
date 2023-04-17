use crate::player::{error::FFMPEGPLayerErros, ffi::{set_decoder_context_time_base, convert_frame_to_ndarray_rgb24, copy_frame_props}};

use super::types::Frame;


pub struct Decoder {
    decoder: ffmpeg::decoder::Video,
    time_base: ffmpeg::util::rational::Rational,
    width: u32,
    height: u32,
    format: ffmpeg::format::Pixel,
    scaler: ffmpeg::software::scaling::Context,
}

impl Decoder {
    pub fn new(
        format: ffmpeg::format::Pixel,
        video_stream: ffmpeg::Stream<'_>,
    ) -> Result<Self, FFMPEGPLayerErros> {
        let mut decoder = ffmpeg::codec::Context::new();

        set_decoder_context_time_base(&mut decoder, video_stream.time_base());

        decoder.set_parameters(video_stream.parameters())?;

        let decoder = decoder.decoder().video()?;

        let time_base = decoder.time_base();

        if decoder.format() == ffmpeg::format::Pixel::None
            || decoder.width() == 0
            || decoder.height() == 0
        {
            return Err("Missing Codec Parameters".into());
        }

        let scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            format,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::Flags::AREA,
        )?;

        let (width, height) = (decoder.width(), decoder.height());

        Ok(Self {
            decoder,
            time_base,
            width,
            height,
            format,
            scaler,
        })
    }

    pub fn decode_rgb(&mut self, packet: &ffmpeg::Packet) -> Result<Frame, FFMPEGPLayerErros> {
        let mut frame = self.decode_raw(packet)?;

        let frame = convert_frame_to_ndarray_rgb24(&mut frame)?;
        Ok(frame)
    }

    pub fn decode_raw(
        &mut self,
        packet: &ffmpeg::Packet,
    ) -> Result<ffmpeg::frame::Video, FFMPEGPLayerErros> {
        //
        // self.decoder.send_packet(packet)?;

        // let frame = self.decoder_receive_frame(frame)?;

        // Ok(frame)

        let mut frame: Option<ffmpeg::frame::Video> = None;
        while frame.is_none() {
            //   packet.rescale_ts(self.stream_time_base(), self.decoder_time_base);

            self.decoder.send_packet(packet)?;

            frame = self.decoder_receive_frame()?;
        }

        let frame = frame.unwrap();
        let mut frame_scaled = ffmpeg::frame::Video::empty();
        self.scaler.run(&frame, &mut frame_scaled)?;

        copy_frame_props(&frame, &mut frame_scaled);

        Ok(frame_scaled)
    }

    /// Pull a decoded frame from the decoder. This function also implements
    /// retry mechanism in case the decoder signals `EAGAIN`.
    fn decoder_receive_frame(&mut self) -> Result<Option<ffmpeg::frame::Video>, ffmpeg::Error> {
        let mut frame = ffmpeg::frame::Video::empty();
        let decode_result = self.decoder.receive_frame(&mut frame);
        match decode_result {
            Ok(()) => Ok(Some(frame)),
            Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
            Err(err) => Err(err.into()),
        }
    }
}
