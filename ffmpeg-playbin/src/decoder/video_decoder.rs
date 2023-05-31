use std::sync::Arc;

use ffmpeg::{codec, Packet, Rational};
use tracing::{error, debug, info};

use crate::{
    ffi::{convert_frame_to_image_handle, copy_frame_props, set_decoder_context_time_base},
    helpers::{time::Time, types::Frame, image::VideoFrame},
    render_queue::Queue,
};

#[derive(Debug)]
pub enum VideoDecoderError {
    Parameters(ffmpeg::Error),
    VideoDecoder(ffmpeg::Error),
    MissingCodecParameters,
    ScallingContext(ffmpeg::Error),
    ConvertToRgb(ffmpeg::Error),
    SendPacket(ffmpeg::Error),
    ScallingSend(ffmpeg::Error),
    ReceivePacket(ffmpeg::Error),
}

pub struct VideoDecoder {
    decoder: ffmpeg::decoder::Video,
    decoder_time_base: ffmpeg::util::rational::Rational,
    width: u32,
    height: u32,
    format: ffmpeg::format::Pixel,
    scaler: ffmpeg::software::scaling::Context,
    rendered_queue: Arc<Queue<VideoFrame>>,
}

impl VideoDecoder {
    pub fn new(
        format: ffmpeg::format::Pixel,
        time_base: Rational,
        parameters: codec::Parameters,
    ) -> Result<Self, VideoDecoderError> {
        let mut decoder = codec::Context::new();

        set_decoder_context_time_base(&mut decoder, time_base);

        decoder
            .set_parameters(parameters)
            .map_err(VideoDecoderError::Parameters)?;

        let decoder = decoder
            .decoder()
            .video()
            .map_err(VideoDecoderError::VideoDecoder)?;

        let decoder_time_base = decoder.time_base();

        if decoder.format() == ffmpeg::format::Pixel::None
            || decoder.width() == 0
            || decoder.height() == 0
        {
            return Err(VideoDecoderError::MissingCodecParameters);
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
        .map_err(VideoDecoderError::ScallingContext)?;

        let (width, height) = (decoder.width(), decoder.height());

        Ok(Self {
            decoder,
            decoder_time_base,
            width,
            height,
            format,
            scaler,
            rendered_queue: Arc::new(Queue::new(20)),
        })
    }

    pub fn spawn(
        mut self,
        raw_queue: Arc<Queue<(Packet, Rational)>>,
    ) -> (
        Arc<Queue<VideoFrame>>,
        tokio::task::JoinHandle<()>,
    ) {
        let cloned_queue = self.rendered_queue.clone();

        let handle = tokio::spawn(async move {
            let (width, height) = (self.width, self.height);

            loop {
                let (mut packet, time) = raw_queue.pop().await;
                match self.decode_image_handle(&mut packet, time) {
                    Ok((time, image)) => {
                        self.rendered_queue
                            .push(VideoFrame::new(time, width, height, image))
                            .await;
                    }
                    Err(err) => {
                        error!("video decoding: {:?}", err);
                    }
                }
            }
        });

        (cloned_queue, handle)
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn decode_image_handle(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<(Time, Frame), VideoDecoderError> {
        let mut frame = self.decode_raw(packet, stream_time_base)?;

        // We use the packet DTS here (which is `frame->pkt_dts`) because that is
        // what the encoder will use when encoding for the `PTS` field.
        let timestamp = Time::new(Some(frame.packet().dts), self.decoder_time_base);
        let frame =
            convert_frame_to_image_handle(&mut frame).map_err(VideoDecoderError::ConvertToRgb)?;

        Ok((timestamp, frame))
    }

    pub fn decode_raw(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<ffmpeg::frame::Video, VideoDecoderError> {
        let mut frame: Option<ffmpeg::frame::Video> = None;
        while frame.is_none() {
            packet.rescale_ts(stream_time_base, self.decoder_time_base);

            self.decoder
                .send_packet(packet)
                .map_err(VideoDecoderError::SendPacket)?;

            frame = self.decoder_receive_frame()?;
        }

        let frame = frame.unwrap();
        let mut frame_scaled = ffmpeg::frame::Video::empty();
        self.scaler
            .run(&frame, &mut frame_scaled)
            .map_err(VideoDecoderError::ScallingSend)?;

        copy_frame_props(&frame, &mut frame_scaled);

        Ok(frame_scaled)
    }

    /// Pull a decoded frame from the decoder. This function also implements
    /// retry mechanism in case the decoder signals `EAGAIN`.
    fn decoder_receive_frame(&mut self) -> Result<Option<ffmpeg::frame::Video>, VideoDecoderError> {
        let mut frame = ffmpeg::frame::Video::empty();
        let decode_result = self.decoder.receive_frame(&mut frame);
        match decode_result {
            Ok(()) => Ok(Some(frame)),
            Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
            Err(err) => Err(VideoDecoderError::ReceivePacket(err)),
        }
    }
}
