mod envelope;
mod note;
mod oscillator;

use std::f64::consts::PI;

use anyhow::Error;
use cpal::{FromSample, OutputCallbackInfo, Sample, SizedSample};
use envelope::Envelope;
use note::Note;
use oscillator::triangle;

use crate::{keyboard::A4_MIDI_VALUE, midi::{MidiReceiver, MIDI_OFF_VALUE, MIDI_ON_VALUE}};

const TWO_PI: f64 = 2.0 * PI;
const OCTAVE_FACTOR: f64 = 2.0;
const NUMBER_OF_NOTES_IN_OCTAVE: f64 = 12.0;
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
    envelope: Envelope,
    midi_rx: MidiReceiver
}

impl AudioModel {
    pub fn new(midi_rx: MidiReceiver) -> Self {
        AudioModel {
            tick: 0.0,
            a4_frequency: STANDARD_A4_FREQUENCY,
            volume: DEFAULT_VOLUME,
            notes: [Note::new(); NUMBER_OF_MIDI_NOTES],
            envelope: Envelope::new(),
            midi_rx,
        }
    }

    /* 
        Updates tick and processes data in the receivers.
    */
    fn update(&mut self) -> Result<(), Error> {
        // UPDATE MIDI
        let mut midi_message = self.midi_rx.try_recv();
        
        loop {
            match midi_message {
                Ok([MIDI_ON_VALUE, note_index, _]) => {
                    self.press_note(note_index as usize);
                    midi_message = self.midi_rx.try_recv();
                },
                Ok([MIDI_OFF_VALUE, note_index, _]) => {
                    self.release_note(note_index as usize);
                    midi_message = self.midi_rx.try_recv();
                },
                Err(_) => break, // Receiver is empty.
                _ => midi_message = self.midi_rx.try_recv() // Other type of message. Ignore for now, FIX LATER.
            }
        }

        Ok(())
    }

    // === METHODS FOR INTERACTING WITH NOTES ===
    /* 
        Finds correct notes by index and delegates the call.
    */
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

    /* 
        NOT USED. REMOVE PERHAPS?
    */
    pub fn deactivate_note(&mut self, index: usize) -> Result<(), Error> {
        AudioModel::check_note_index(index)?;

        self.notes[index].deactivate();
        Ok(())
    }

    fn check_note_index(index: usize) -> Result<(), Error> {
        if index >= NUMBER_OF_MIDI_NOTES {
            return Err(Error::msg("Note index out of bounds."));
        }
        Ok(())
    }
}

pub fn make_audio_callback<SampleType>(
    channels: usize, 
    sample_rate: f64, 
    midi_rx: MidiReceiver,
) -> impl FnMut(&mut [SampleType], &OutputCallbackInfo) + Send + 'static 
where
    SampleType: SizedSample + FromSample<f64>,
{
    // AudioModel to store program state.
    let mut audio_model = AudioModel::new(midi_rx);

    // Audio callback to fill buffer for output.
    let audio_callback = move |buffer: &mut [SampleType], _: &OutputCallbackInfo| {
        audio_model.update();

        // One frame represents one point in time. 
        // The number of elements in a frame depends on the number of channels.
        // For example, stereo audio has two channels for left and right respectively.
        for frame in buffer.chunks_mut(channels) {
            let mut amplitude: f64 = 0.0;

            // Evaluates which notes are activated and calculates their frequency.
            // DOES THIS NEED TO REPEAT FOR EVERY FRAME? WHY NOT JUST ADD THE INDICES TO A VECTOR TO AVOID REPEAT CALCULATIONS? BENCHMARK PLS.
            for (index, note) in audio_model.notes.iter_mut().enumerate() {
                if note.get_active() {
                    amplitude += triangle(TWO_PI * (audio_model.tick / sample_rate) 
                        * (OCTAVE_FACTOR.powf((index as i16 - A4_MIDI_VALUE) as f64 / NUMBER_OF_NOTES_IN_OCTAVE) * audio_model.a4_frequency)) 
                        * audio_model.envelope.get_amplitude_of_note(note, audio_model.tick, sample_rate) 
                        * audio_model.volume;
                }
            }

            for channel in frame {
                *channel = Sample::from_sample(amplitude);
            }
            audio_model.tick += 1.0;
        }
    };

    audio_callback
}
