use ffmpeg::Rational;

use crate::player::{
    error::FFMPEGPLayerErros,
    ffi::{convert_frame_to_vec_rgba, copy_frame_props, set_decoder_context_time_base},
};

use super::{time::Time, types::Frame};

#[derive(Debug)]
pub enum DecoderError {
    Parameters(ffmpeg::Error),
    VideoDecoder(ffmpeg::Error),
    MissingCodecParameters,
    ScallingContext(ffmpeg::Error),
    ConvertToRgb(ffmpeg::Error),
    SendPacket(ffmpeg::Error),
    ScallingSend(ffmpeg::Error),
    ReceivePacket(ffmpeg::Error),
}

pub struct Decoder {
    decoder: ffmpeg::decoder::Video,
    decoder_time_base: ffmpeg::util::rational::Rational,
    width: u32,
    height: u32,
    format: ffmpeg::format::Pixel,
    scaler: ffmpeg::software::scaling::Context,
}

impl Decoder {
    pub fn new(
        format: ffmpeg::format::Pixel,
        video_stream: ffmpeg::Stream<'_>,
    ) -> Result<Self, DecoderError> {
        let mut decoder = ffmpeg::codec::Context::new();

        set_decoder_context_time_base(&mut decoder, video_stream.time_base());

        decoder
            .set_parameters(video_stream.parameters())
            .map_err(DecoderError::Parameters)?;

        let decoder = decoder
            .decoder()
            .video()
            .map_err(DecoderError::VideoDecoder)?;

        let decoder_time_base = decoder.time_base();

        if decoder.format() == ffmpeg::format::Pixel::None
            || decoder.width() == 0
            || decoder.height() == 0
        {
            return Err(DecoderError::MissingCodecParameters);
        }

        let scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            format,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::Flags::BILINEAR,
        )
        .map_err(DecoderError::ScallingContext)?;

        let (width, height) = (decoder.width(), decoder.height());

        Ok(Self {
            decoder,
            decoder_time_base,
            width,
            height,
            format,
            scaler,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn decode_image_handle(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<(Time, Frame), DecoderError> {
        let mut frame = self.decode_raw(packet, stream_time_base)?;

        // We use the packet DTS here (which is `frame->pkt_dts`) because that is
        // what the encoder will use when encoding for the `PTS` field.
        let timestamp = Time::new(Some(frame.packet().dts), self.decoder_time_base);
        let frame =
            convert_frame_to_vec_rgba(&mut frame).map_err(DecoderError::ConvertToRgb)?;

        Ok((timestamp, frame))
    }

    pub fn decode_raw(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<ffmpeg::frame::Video, DecoderError> {
        let mut frame: Option<ffmpeg::frame::Video> = None;
        while frame.is_none() {
            packet.rescale_ts(stream_time_base, self.decoder_time_base);

            self.decoder
                .send_packet(packet)
                .map_err(DecoderError::SendPacket)?;

            frame = self.decoder_receive_frame()?;
        }

        let frame = frame.unwrap();
        let mut frame_scaled = ffmpeg::frame::Video::empty();
        self.scaler
            .run(&frame, &mut frame_scaled)
            .map_err(DecoderError::ScallingSend)?;

        copy_frame_props(&frame, &mut frame_scaled);

        Ok(frame_scaled)
    }

    /// Pull a decoded frame from the decoder. This function also implements
    /// retry mechanism in case the decoder signals `EAGAIN`.
    fn decoder_receive_frame(&mut self) -> Result<Option<ffmpeg::frame::Video>, DecoderError> {
        let mut frame = ffmpeg::frame::Video::empty();
        let decode_result = self.decoder.receive_frame(&mut frame);
        match decode_result {
            Ok(()) => Ok(Some(frame)),
            Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
            Err(err) => Err(DecoderError::ReceivePacket(err)),
        }
    }
}
