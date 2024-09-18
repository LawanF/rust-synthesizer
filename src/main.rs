use std::sync::mpsc::channel;

use cpal::Stream;
use keyboard::parse_key_as_note_input;
use midi::{open_midi_input, MidiReceiver, MidiSender, MIDI_OFF_VALUE, MIDI_ON_VALUE};
use midir::MidiInputConnection;
use nannou::prelude::*;

mod audio_setup;
mod audio_engine;
mod keyboard;
mod midi;

use audio_setup::initialise_audio;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

struct Model {
    stream: Stream,
    midi_input_connection: Option<MidiInputConnection<()>>,
    midi_tx: MidiSender,
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
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

    let (midi_tx, midi_rx) = channel();

    let stream = match initialise_audio(midi_rx) {
        Ok(stream) => stream,
        Err(_) => panic!("Can't initialise audio stream!"),
    };

    Model {
        stream,
        midi_input_connection: None,
        midi_tx,
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
}

fn update(_app: &App, model: &mut Model, _update: Update) {
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    println!("Pressed: {:?}", key);

    match parse_key_as_note_input(key) {
        Some(index) => {
            model.midi_tx.send([MIDI_ON_VALUE, index, 0]).unwrap();
            return;
        },
        None => println!("teehee!"),
    }
    
    // Request to open midi input.
    if key == Key::Semicolon {
        match open_midi_input(model.midi_tx.clone()) {
            Ok(midi_input_connection) => {
                println!("Found connection!");
                model.midi_input_connection = Some(midi_input_connection);
            },
            Err(_) => println!("No connection found."),
        }
    }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    println!("Released: {:?}", key);

    match parse_key_as_note_input(key) {
        Some(index) => {
            model.midi_tx.send([MIDI_OFF_VALUE, index, 0]).unwrap();
            return;
        },
        None => println!("teehee!"),
    }
}