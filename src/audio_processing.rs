use anyhow::Error;
use nannou::{frame, prelude::*};
use nannou_audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;

use crate::note::Note;
use crate::oscillator::{sin, triangle, square, sawtooth};
use crate::keyboard::A4_MIDI_VALUE;

const NUMBER_OF_MIDI_NOTES: usize = 128;
const STANDARD_A4_FREQUENCY: f64 = 440.0;
const DEFAULT_VOLUME: f64 = 0.1;

/*
    Stores program state information relevant to audio generation.
*/
pub struct AudioModel {
    tick: f64,
    a4_frequency: f64,
    volume: f64,
    notes: [Note; NUMBER_OF_MIDI_NOTES],
}

impl AudioModel {
    pub fn new() -> Self {
        AudioModel {
            tick: 0.0,
            a4_frequency: STANDARD_A4_FREQUENCY,
            volume: DEFAULT_VOLUME,
            notes: [Note::new(); NUMBER_OF_MIDI_NOTES],
        }
    }

    // === METHODS FOR INTERACTING WITH NOTES ===
    pub fn press_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index].press(self.tick);
        Ok(())
    }

    pub fn release_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index].release(self.tick);
        Ok(())
    }

    pub fn deactivate_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index].deactivate();
        Ok(())
    }

    fn check_note_index(index: usize) -> Result<(), Error> {
        if index >= NUMBER_OF_MIDI_NOTES {
            return Err(Error::msg("Note index out of bounds."))
        }
        Ok(())
    }
}

/* 
    Audio rendering function. 
    Places the calculated PCM values inside a buffer to be then sent to playback.
*/
pub fn audio(audio_model: &mut AudioModel, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64; 
    
    // One frame represents one point in time. 
    // The number of elements in a frame depends on the number of channels.
    // For example, stereo audio has two channels for left and right respectively.
    for frame in buffer.frames_mut() {
        let mut amplitude: f64 = 0.0;

        // Evaluates which notes are activated and calculates their frequency.
        // DOES THIS NEED TO REPEAT FOR EVERY FRAME? WHY NOT JUST ADD THE INDICES TO A VECTOR TO AVOID REPEAT CALCULATIONS? BENCHMARK PLS.
        for (index, &note) in audio_model.notes.iter().enumerate() {
            if note.get_active() {
                amplitude += triangle(2.0 * PI * (audio_model.tick / sample_rate) * (2.0.pow((index as i16 - A4_MIDI_VALUE) as f64 / 12.0) * audio_model.a4_frequency)) * audio_model.volume;
            }
        }

        for channel in frame {
            *channel = amplitude as f32;
        }
        audio_model.tick += 1.0;
    }
}