use cpal::{traits::{DeviceTrait, HostTrait}, Device, FromSample, Sample, SizedSample, StreamConfig, SupportedStreamConfig};

fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device().expect("No output device available.");

    let supported_config = device.default_output_config().expect("No output configuration available.");

    let stream = initialise_stream(device, supported_config).expect("Could not initialise stream.");

    loop {
    }
}

/*
    Matches sample format and then calls a function that will create the stream.
*/
fn initialise_stream(
    device: Device, 
    supported_config: SupportedStreamConfig,
) -> Result<cpal::Stream, anyhow::Error> {
    match supported_config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &supported_config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &supported_config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &supported_config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &supported_config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &supported_config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &supported_config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &supported_config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &supported_config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &supported_config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &supported_config.into()),
        _ => panic!("Sample format not supported.") // Change the error handling later.
    }
}

mod math;
use math::sin;
use std::f64::consts::PI;

/*
    Creates the stream for a device using information from the given config.
*/
fn make_stream<SampleType>(
    device: &Device,
    config: &StreamConfig,
) -> Result<cpal::Stream, anyhow::Error> 
where
    SampleType: SizedSample + FromSample<f64>,
{
    // Get information from config.
    let channels = config.channels as usize;
    let sample_rate = config.sample_rate.0 as f64; // f64 for compatibility with amplitude calculation.

    let err_fn = |err| eprintln!("Failed to build output stream: {err}");

    let mut tick: f64 = 0.0; // To keep track of time.

    let data_callback = move |buffer: &mut [SampleType], _: &cpal::OutputCallbackInfo| {
        for frame in buffer.chunks_mut(channels) {
            let value = sin(2.0 * PI * (tick / sample_rate) * 440.0);
            
            for sample in frame.iter_mut() {
                *sample = Sample::from_sample(value);
            }

            tick += 1.0;
        }
    };

    let stream = device.build_output_stream(config, data_callback, err_fn, None)?;

    Ok(stream)
}