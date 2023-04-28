use std::sync::Arc;

use ffmpeg::{codec, frame, ChannelLayout, Packet, Rational};
use tracing::{debug, error, info};

use crate::{ffi::set_decoder_context_time_base, helpers::time::Time, render_queue::Queue};

#[derive(Debug)]
pub enum AudioDecoderError {
    Parameters(ffmpeg::Error),
    AudioDecoder(ffmpeg::Error),
    MissingCodecParameters,
    ResamplingContext(ffmpeg::Error),
    SendPacket(ffmpeg::Error),
    ReceivePacket(ffmpeg::Error),
}

pub struct AudioDecoder {
    decoder: ffmpeg::decoder::Audio,
    decoder_time_base: Rational,
    resampler: ffmpeg::software::resampling::Context,
    rendered_queue: Arc<Queue<frame::Audio>>,
}

impl AudioDecoder {
    pub fn new(
        format: ffmpeg::format::Sample,
        layout: u16,
        rate: u32,
        time_base: Rational,
        parameters: codec::Parameters,
    ) -> Result<Self, AudioDecoderError> {
        let layout = ChannelLayout::default(layout as i32);
        let mut decoder = ffmpeg::codec::Context::new();

        set_decoder_context_time_base(&mut decoder, time_base);

        decoder
            .set_parameters(parameters)
            .map_err(AudioDecoderError::Parameters)?;

        let decoder = decoder
            .decoder()
            .audio()
            .map_err(AudioDecoderError::AudioDecoder)?;

        let decoder_time_base = decoder.time_base();

        if decoder.format() == ffmpeg::format::Sample::None
            || decoder.rate() == 0
            || decoder.channels() == 0
        {
            return Err(AudioDecoderError::MissingCodecParameters);
        }

        let resampler = ffmpeg::software::resampling::Context::get(
            decoder.format(),
            decoder.channel_layout(),
            decoder.rate(),
            format,
            layout,
            rate,
        )
        .map_err(AudioDecoderError::ResamplingContext)?;

        Ok(Self {
            decoder,
            decoder_time_base,
            resampler,
            rendered_queue: Arc::new(Queue::new(20)),
        })
    }

    pub fn spawn(
        mut self,
        raw_audio_queue: Arc<Queue<(Packet, Rational)>>,
    ) -> (Arc<Queue<frame::Audio>>, tokio::task::JoinHandle<()>) {
        let cloned_queue = self.rendered_queue.clone();

        let handle = tokio::spawn(async move {
            loop {
                let (mut packet, stream_time_base) = raw_audio_queue.pop().await;
                match self.decode(&mut packet, stream_time_base) {
                    Ok(maybe_frame) => {
                        if let Some((_time, frame)) = maybe_frame {
                            self.rendered_queue.push(frame).await;
                        }
                    }
                    Err(err) => {
                        error!("audio decoding: {:?}", err);
                    }
                }
            }
        });

        (cloned_queue, handle)
    }

    pub fn decode(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<Option<(Time, frame::Audio)>, AudioDecoderError> {
        let frame = self.decode_raw(packet, stream_time_base)?;
        if let Some(frame) = frame {
            // We use the packet DTS here (which is `frame->pkt_dts`) because that is
            // what the encoder will use when encoding for the `PTS` field.
            let timestamp = Time::new(Some(frame.packet().dts), self.decoder_time_base);

            Ok(Some((timestamp, frame)))
        } else {
            Ok(None)
        }
    }

    pub fn decode_raw(
        &mut self,
        packet: &mut ffmpeg::Packet,
        stream_time_base: Rational,
    ) -> Result<Option<frame::Audio>, AudioDecoderError> {
        packet.rescale_ts(stream_time_base, self.decoder_time_base);

        self.decoder
            .send_packet(packet)
            .map_err(AudioDecoderError::SendPacket)?;
        if let Some(frame) = self.decoder_receive_frame()? {
            let mut buffer = frame::Audio::empty();

            self.resampler
                .run(&frame, &mut buffer)
                .map_err(AudioDecoderError::ResamplingContext)?;

            Ok(Some(buffer))
        } else {
            Ok(None)
        }
    }

    /// Pull a decoded frame from the decoder. This function also implements
    /// retry mechanism in case the decoder signals `EAGAIN`.
    fn decoder_receive_frame(&mut self) -> Result<Option<ffmpeg::frame::Audio>, AudioDecoderError> {
        let mut frame = ffmpeg::frame::Audio::empty();
        let decode_result = self.decoder.receive_frame(&mut frame);
        match decode_result {
            Ok(()) => Ok(Some(frame)),
            Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
            Err(err) => Err(AudioDecoderError::ReceivePacket(err)),
        }
    }
}
