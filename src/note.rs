/*
    Represents a single onte. Stores if note is pressed (on a MIDI controller),
    if the note is active (can be active without being pressed with release time),
    velocity (a value which monitors how fast the key is pressed),
    and when the tick when the note was pressed and released.
*/
#[derive(Clone, Copy)]
pub struct Note {
    pressed: bool,
    active: bool, // Used to show that the note is still sounding despite its key being released.
    // velocity: u8,
    tick_pressed: f64,
    tick_released: f64,
}

impl Note {
    pub fn new() -> Self {
        Note {
            pressed: false,
            active: false,
            // velocity: 0,
            tick_pressed: 0.0,
            tick_released: 0.0,
        }
    }

    pub fn press(&mut self, tick: f64) {
        // To avoid repeat presses as a result of holding a key.
        if !self.pressed {
            self.pressed = true;
            self.active = true;
            self.tick_pressed = tick;
        }
    }

    pub fn release(&mut self, tick: f64) {
        self.pressed = false;
        self.tick_released = tick;
    }

    // Used when the release time of the note elapses.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn get_pressed(&self) -> bool {
        self.pressed
    }

    pub fn get_active(&self) -> bool {
        self.active
    }

    pub fn get_tick_pressed(&self) -> f64 {
        self.tick_pressed
    }

    pub fn get_tick_released(&self) -> f64 {
        self.tick_released
    }
}