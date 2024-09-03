use anyhow::Error;
use nannou::{frame, prelude::*};
use nannou_audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

use crate::oscillator::sin;

const NUMBER_OF_NOTES: usize = 13;
const DEFAULT_BASE_FREQUENCY: f64 = 440.0;
const DEFAULT_VOLUME: f64 = 0.5;

pub struct AudioModel {
    tick: f64,
    base_frequency: f64,
    volume: f64,
    notes: Vec<bool>,
}

impl AudioModel {
    pub fn new() -> Self {
        AudioModel {
            tick: 0.0,
            base_frequency: DEFAULT_BASE_FREQUENCY,
            volume: DEFAULT_VOLUME,
            notes: vec![false; NUMBER_OF_NOTES],
        }
    }

    pub fn activate_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index] = true;
        Ok(())
    }

    pub fn deactivate_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index] = false;
        Ok(())
    }

    fn check_note_index(index: usize) -> Result<(), Error> {
        if index >= NUMBER_OF_NOTES {
            return Err(Error::msg("Note index out of bounds."))
        }
        Ok(())
    }
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play a simple sine wave at the audio's current frequency in `hz`.
pub fn audio(audio_model: &mut AudioModel, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    
    for (index, &note) in audio_model.notes.iter().enumerate() {
        if note {
            // One frame represents one point in time. 
            // The number of elements in a frame depends on the number of channels.
            // For example, stereo audio has two channels for left and right respectively.
            for frame in buffer.frames_mut() {
                let amplitude = sin(2.0 * PI * (audio_model.tick / sample_rate) * (2.0.pow(index as f64 / 12.0) * audio_model.base_frequency)) * audio_model.volume;
                println!("amp: {amplitude}");
                for channel in frame {
                    *channel = amplitude as f32;
                }
                println!("tick: {}", audio_model.tick);
                audio_model.tick += 1.0;
            }
        }
    }
}