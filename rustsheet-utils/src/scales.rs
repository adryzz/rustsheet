use rustsheet::notes::{Octave, Tone, ToneModifiers};

use crate::tones;

const MAJOR_SCALE_0: [Tone; 7] = [
    tones::C0,
    tones::D0,
    tones::E0,
    tones::F0,
    tones::G0,
    tones::A0,
    tones::B0
];

pub struct MajorScale {
    start: Tone,
    current: Tone,
}

impl MajorScale {
    /// Generates a major scale (if it exists) from the starting tone
    /// 
    /// Major scales exist only for C, G, D, A, E, B, F# and C#
    #[rustfmt::skip]
    pub fn new(start: Tone) -> Option<Self> {
        match start {
            Tone { octave: Octave::C, modifiers: None, ..} |
            Tone { octave: Octave::G, modifiers: None, ..} |
            Tone { octave: Octave::D, modifiers: None, ..} |
            Tone { octave: Octave::A, modifiers: None, ..} |
            Tone { octave: Octave::E, modifiers: None, ..} |
            Tone { octave: Octave::B, modifiers: None, ..} |
            Tone { octave: Octave::F, modifiers: Some(ToneModifiers::Sharp), ..} |
            Tone { octave: Octave::G, modifiers: Some(ToneModifiers::Flat), ..} |
            Tone { octave: Octave::C, modifiers: Some(ToneModifiers::Sharp), ..} |
            Tone { octave: Octave::D, modifiers: Some(ToneModifiers::Flat), ..} => 
            Some(Self { start, current: start }),
            _ => None
        }
    }
}

impl Iterator for MajorScale {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
