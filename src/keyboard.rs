use nannou::prelude::Key;

pub const C3_MIDI_VALUE: usize = 48; // MOVE THESE TO MIDI.RS?
pub const A4_MIDI_VALUE: i16 = 69;

const KEYBOARD_NOTE_LAYOUT: [Key; 25] = [Key::Z, Key::S, Key::X, Key::D, Key::C, Key::V, Key::G, Key::B, Key::H, Key::N, Key::J, Key::M, // First octave.
                                         Key::Q, Key::Key2, Key::W, Key::Key3, Key::E, Key::R, Key::Key5, Key::T, Key::Key6, Key::Y, Key::Key7, Key::U, // Second octave.
                                         Key::I]; // High C.


/* 
    Parses keyboard input as note input. Returns the distance of the note from A4 in semitones.
*/
pub fn parse_key_as_note_input(key: Key) -> Option<usize> {
    match KEYBOARD_NOTE_LAYOUT.iter().position(|&elem| elem == key) {
        Some(position) => Some(position + C3_MIDI_VALUE),
        None => None,
    }
}