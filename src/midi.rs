use std::sync::mpsc::{Receiver, Sender};

use anyhow::Error;
use midir::{Ignore, MidiInput, MidiInputConnection};

pub const LENGTH_OF_MESSAGE_ARRAY: usize = 3;
pub const MIDI_ON_VALUE: u8 = 144;
pub const MIDI_OFF_VALUE: u8 = 128;


pub type MidiReceiver = Receiver<[u8; LENGTH_OF_MESSAGE_ARRAY]>;
pub type MidiSender = Sender<[u8; LENGTH_OF_MESSAGE_ARRAY]>;

/*
    Opens first MIDI input device.
    The returned MidiInputConnection must be in scope for as long as input needs to be read.
*/
pub fn open_midi_input(midi_tx: MidiSender) -> Result<MidiInputConnection<()>, Error> {
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
            midi_tx.send(message.try_into().unwrap()); // How do I handle the error here?
        }, 
        (),
    )?;

    Ok(in_connection)
}