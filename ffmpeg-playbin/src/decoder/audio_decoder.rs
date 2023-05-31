use std::sync::Arc;

use async_ringbuf::{AsyncHeapRb, AsyncProducer};
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

pub struct AudioDecoder<T> {
    decoder: ffmpeg::decoder::Audio,
    decoder_time_base: Rational,
    resampler: ffmpeg::software::resampling::Context,
    rendered_queue: AsyncProducer<T>,
}

impl<T: frame::audio::Sample> AudioDecoder<T> {
    pub fn new(
        format: ffmpeg::format::Sample,
        layout: u16,
        rate: u32,
        time_base: Rational,
        parameters: codec::Parameters,
    ) {
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

        let (producer, consumer) = AsyncHeapRb::<T>::new(100).split();

        Ok((Self {
            decoder,
            decoder_time_base,
            resampler,
            rendered_queue: producer,
        }, consumer))
    }

    pub fn spawn(
        mut self,
        raw_audio_queue: Arc<Queue<(Packet, Rational)>>,
    ) -> (Arc<Queue<T>>, tokio::task::JoinHandle<()>) {
        let cloned_queue = self.rendered_queue.clone();

        let handle = tokio::spawn(async move { loop {} });

        (cloned_queue, handle)
    }

    pub async fn decode_test(
        &self,
        raw_audio_queue: &Arc<Queue<(Packet, Rational)>>,
    ) -> Result<(), AudioDecoderError> {
        let (mut packet, stream_time_base) = raw_audio_queue.pop().await;

        packet.rescale_ts(stream_time_base, self.decoder_time_base);

        self.decoder
            .send_packet(&packet)
            .map_err(AudioDecoderError::SendPacket)?;

        let mut decoded = frame::Audio::empty();

        // Ask the decoder for frames
        while self.decoder.receive_frame(&mut decoded).is_ok() {
            // Resample the frame's audio into another frame
            let mut resampled = frame::Audio::empty();
            self.resampler.run(&decoded, &mut resampled)?;

            // DON'T just use resampled.data(0).len() -- it might not be fully populated
            // Grab the right number of bytes based on sample count, bytes per sample, and number of channels.
            let both_channels = packed::<T>(&resampled);

            // // Sleep until the buffer has enough space for all of the samples
            // // (the producer will happily accept a partial write, which we don't want)
            // while producer.remaining() < both_channels.len() {
            //     std::thread::sleep(std::time::Duration::from_millis(10));
            // }

            self.rendered_queue.push(i).await;
        }
        Ok(())
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
    ) -> Result<Option<ffmpeg::frame::Audio>, AudioDecoderError> {
        packet.rescale_ts(stream_time_base, self.decoder_time_base);

        self.decoder
            .send_packet(packet)
            .map_err(AudioDecoderError::SendPacket)?;

        let mut decoded = frame::Audio::empty();

        let continue_decoding = true;

        while continue_decoding {
            let mut decoded = ffmpeg::frame::Audio::empty();
            let decode_result = self.decoder.receive_frame(&mut decoded);
            match decode_result {
                Ok(()) => {
                    // Resample the frame's audio into another frame
                    let mut resampled = frame::Audio::empty();
                    self.resampler
                        .run(&decoded, &mut resampled)
                        .map_err(AudioDecoderError::ResamplingContext)?;

                    // DON'T just use resampled.data(0).len() -- it might not be fully populated
                    // Grab the right number of bytes based on sample count, bytes per sample, and number of channels.
                    let both_channels = packed(&resampled);

                    Ok(Some(decoded));
                }
                Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
                Err(err) => Err(AudioDecoderError::ReceivePacket(err)),
            }
        }

        if let Some(frame) = self.decoder_receive_frame()? {
            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }

    /// Pull a decoded frame from the decoder. This function also implements
    /// retry mechanism in case the decoder signals `EAGAIN`.
    fn decoder_receive_frame(&mut self) -> Result<Option<ffmpeg::frame::Audio>, AudioDecoderError> {
        let mut decoded = ffmpeg::frame::Audio::empty();
        let decode_result = self.decoder.receive_frame(&mut decoded);
        match decode_result {
            Ok(()) => Ok(Some(decoded)),
            Err(ffmpeg::Error::Other { errno }) if errno == ffmpeg::error::EAGAIN => Ok(None),
            Err(err) => Err(AudioDecoderError::ReceivePacket(err)),
        }
    }
}

// Interpret the audio frame's data as packed (alternating channels, 12121212, as opposed to planar 11112222)
pub fn packed<T: frame::audio::Sample>(frame: &frame::Audio) -> &[T] {
    if !frame.is_packed() {
        panic!("data is not packed");
    }

    if !<T as frame::audio::Sample>::is_valid(frame.format(), frame.channels()) {
        panic!("unsupported type");
    }

    unsafe {
        std::slice::from_raw_parts(
            (*frame.as_ptr()).data[0] as *const T,
            frame.samples() * frame.channels() as usize,
        )
    }
}
