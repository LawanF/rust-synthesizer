use anyhow::Error;
use cpal::{default_host, traits::{DeviceTrait, HostTrait}, Device, FromSample, SampleFormat, SizedSample, Stream, StreamConfig, SupportedStreamConfig};

use crate::{audio_engine::make_audio_callback, midi::MidiReceiver};

/*
    Initialises audio host, device, and returns an output stream with the synthesizer callback function.
    The returned stream might need to be unpaused before audio is played.
    Takes a MidiReceiver for the AudioModel. The MidiSender should be passed to keyboard and MIDI input.
*/
pub fn initialise_audio(
    midi_rx: MidiReceiver,
) -> Result<Stream, Error> {
    let host = default_host();

    let device = host.default_output_device().expect("No output device available.");

    let supported_config = device.default_output_config()?;

    initialise_stream(midi_rx, device, supported_config)
}

/*
    Matches sample format and then calls a function that will create the stream.
*/
fn initialise_stream(
    midi_rx: MidiReceiver,
    device: Device, 
    supported_config: SupportedStreamConfig,
) -> Result<Stream, Error> {
    match supported_config.sample_format() {
        SampleFormat::I8 => make_stream::<i8>(midi_rx, &device, &supported_config.into()),
        SampleFormat::I16 => make_stream::<i16>(midi_rx, &device, &supported_config.into()),
        SampleFormat::I32 => make_stream::<i32>(midi_rx, &device, &supported_config.into()),
        SampleFormat::I64 => make_stream::<i64>(midi_rx, &device, &supported_config.into()),
        SampleFormat::U8 => make_stream::<u8>(midi_rx, &device, &supported_config.into()),
        SampleFormat::U16 => make_stream::<u16>(midi_rx, &device, &supported_config.into()),
        SampleFormat::U32 => make_stream::<u32>(midi_rx, &device, &supported_config.into()),
        SampleFormat::U64 => make_stream::<u64>(midi_rx, &device, &supported_config.into()),
        SampleFormat::F32 => make_stream::<f32>(midi_rx, &device, &supported_config.into()),
        SampleFormat::F64 => make_stream::<f64>(midi_rx, &device, &supported_config.into()),
        _ => return Err(Error::msg("Sample format not supported!")),
    }
}

/*
    Creates the stream for a device using information from the given config.
*/
fn make_stream<SampleType>(
    midi_rx: MidiReceiver,
    device: &Device,
    config: &StreamConfig,
) -> Result<Stream, Error> 
where
    SampleType: SizedSample + FromSample<f64>,
{
    // Get information from config.
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f64; // f64 for compatibility with amplitude calculation.

    let err_fn = |err| eprintln!("Failed to build output stream: {err}");

    // Audio callback to fill buffer for output.
    let audio_callback = make_audio_callback::<SampleType>(channels, sample_rate, midi_rx);

    let stream = device.build_output_stream(config, audio_callback, err_fn, None)?;

    Ok(stream)
}