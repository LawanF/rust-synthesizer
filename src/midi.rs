use anyhow::Error;
use midir::{Ignore, MidiInput, MidiInputConnection};

/*
    Opens first MIDI input device.
    The returned MidiInputConnection must be in scope for as long as input needs to be read.
*/
pub fn open_midi_input() -> Result<MidiInputConnection<()>, Error> {
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
        }, 
        (),
    )?;

    Ok(in_connection)
}