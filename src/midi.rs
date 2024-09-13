use std::sync::mpsc::Sender;

use anyhow::Error;
use midir::{Ignore, MidiInput, MidiInputConnection};

pub const LENGTH_OF_MESSAGE_ARRAY: usize = 3;
pub const MIDI_PRESS_VALUE: u8 = 144;
pub const MIDI_RELEASE_VALUE: u8 = 128;
pub const MIDI_BYTE_RATE: f64 = 3125.0;

/*
    Opens first MIDI input device.
    The returned MidiInputConnection must be in scope for as long as input needs to be read.
*/
pub fn open_midi_input(midi_tx: Sender<[u8; 3]>) -> Result<MidiInputConnection<()>, Error> {
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port. Take index 0 if multiple are available.
    let in_ports = midi_in.ports();
    let in_port = if in_ports.len() == 0 {
        return Err(Error::msg("No input port found."));
    } else {
        &in_ports[0]
    };

    println!("Chosen input port: {}", midi_in.port_name(in_port).unwrap());

    // If this goes out of scope, input is no longer read.
    let in_connection = midi_in.connect(
        in_port, 
        "midir-read-input", 
        move |stamp, message, _| {
            // Insert correct function here.
            println!("Stamp: {}, Message: {:?}", stamp, message);
            midi_tx.send(message.try_into().unwrap()).unwrap();
        }, 
        (),
    )?;

    Ok(in_connection)
}