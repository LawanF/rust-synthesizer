use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc::channel;

use keyboard::parse_key_as_note_input;
use midi::{open_midi_input, MIDI_PRESS_VALUE, MIDI_RELEASE_VALUE};
use midir::MidiInputConnection;
use nannou::prelude::*;
use nannou_audio;

mod audio_processing;
mod envelope;
mod keyboard;
mod midi;
mod note;
mod oscillator;

use audio_processing::AudioModel;
use audio_processing::audio;
use midi::{LENGTH_OF_MESSAGE_ARRAY, MIDI_BYTE_RATE};

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

struct Model {
    stream: nannou_audio::Stream<AudioModel>,
    midi_input: Option<MidiInputConnection<()>>,
    midi_tx: Sender<[u8; LENGTH_OF_MESSAGE_ARRAY]>,
    midi_rx: Receiver<[u8; LENGTH_OF_MESSAGE_ARRAY]>,
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::rate_fps(MIDI_BYTE_RATE))
        .run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
                    .title(app.exe_name().unwrap())
                    .size(WINDOW_WIDTH, WINDOW_HEIGHT)
                    .view(view)
                    .key_pressed(key_pressed)
                    .key_released(key_released)
                    .build()
                    .unwrap();

    let audio_host = nannou_audio::Host::new();

    let audio_model = AudioModel::new();

    let stream = audio_host.new_output_stream(audio_model)
                        .render(audio)
                        .build()
                        .unwrap();

    let (midi_tx, midi_rx) = channel();
    Model {
        stream,
        midi_input: match open_midi_input(midi_tx.clone()) {
            Ok(in_connection) => Some(in_connection),
            Err(err) => {
                println!("{}", err);
                None
            }
        },
        midi_tx,
        midi_rx,
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.midi_rx.try_recv() {
        Ok([MIDI_PRESS_VALUE, index, _]) => {
            model.stream.send(move |audio_model| {
                audio_model.press_note(index as usize).unwrap();
            }).unwrap();
        },
        Ok([MIDI_RELEASE_VALUE, index, _]) => {
            model.stream.send(move |audio_model| {
                audio_model.release_note(index as usize).unwrap();
            }).unwrap();
        },
        _ => (),
    };
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("Pressed: {:?}", key);

    match parse_key_as_note_input(key) {
        Some(index) => {
            model.stream.send(move |audio_model| {
                audio_model.press_note(index).unwrap();
            }).unwrap();
            return;
        },
        None => println!("teehee!"),
    }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    println!("Released: {:?}", key);

    match parse_key_as_note_input(key) {
        Some(index) => {
            model.stream.send(move |audio_model| {
                audio_model.release_note(index).unwrap();
            }).unwrap();
            return;
        },
        None => println!("teehee!"),
    }
}