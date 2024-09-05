use crate::note::Note;

const DEFAULT_ATTACK: f64 = 0.8;
const DEFAULT_DECAY: f64 = 5.0;
const DEFAULT_SUSTAIN: f64 = 0.0;
const DEFAULT_RELEASE: f64 = 0.2;

const LOWEST_AMPLITUDE: f64 = 0.0001; // When a signal is below or equal to this value, it is processed as no signal at all. 

/* 
    A structure to store envelope parameters for the synth.
    Attack, decay, and release are stored as seconds.
    Sustain is stored as part of the highest amplitude.
*/
#[derive(Clone, Copy)]
pub struct Envelope {
    attack: f64, // The initial rise of the amplitude.
    decay: f64, // The descent of the amplitude into the sustain.
    sustain: f64, // The amplitude the note has while held.
    release: f64, // The descent into quiet after the signal is released.
}

impl Envelope {
    pub fn new() -> Self {
        Envelope {
            attack: DEFAULT_ATTACK,
            decay: DEFAULT_DECAY,
            sustain: DEFAULT_SUSTAIN,
            release: DEFAULT_RELEASE,
        }
    }

    /*
        Computes the appropriate amplitude of note based on when it was pressed/released in relation to the ADSR values.
    */
    pub fn get_amplitude_of_note(self, note: &mut Note, tick: f64, sample_rate: f64) -> f64 {
        let note_lifetime: f64 = (tick - note.get_tick_pressed()) / sample_rate;
        let ads_amplitude: f64 = self.get_ads(note_lifetime);
        if note.get_pressed() {
            return ads_amplitude;
        } else {
            // RELEASE
            let note_deathtime: f64 = (tick - note.get_tick_released()) / sample_rate;

            // Note that we decrease the amplitude from where it was released in the ADS part of the envelope.
            let release_value: f64 = (1.0 - (note_deathtime / self.release)) * ads_amplitude;

            if release_value <= LOWEST_AMPLITUDE {
                // Signal is quiet enough to be processed as no signal. Deactivate note.
                note.deactivate();
                return 0.0;
            }
            release_value
        }
    }

    /* 
        Fetches the ADS value of a given note based on when it was pressed. 
    */
    fn get_ads(self, note_lifetime: f64) -> f64 {
        if note_lifetime <= self.attack {
            // ATTACK
            return note_lifetime / self.attack;
        } else if note_lifetime <= self.attack + self.decay {
            // DECAY
            return 1.0 - ((note_lifetime - self.attack) / self.decay) * (1.0 - self.sustain);
        }
        // SUSTAIN
        return self.sustain;
    }
}