use rustsheet::notes::{Octave, Tone, ToneModifiers};

pub struct MajorScale {
    current: u32,
    iter: u32,
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
            Some(Self { current: start.get_semitones_since_c0(), iter: 0 }),
            _ => None
        }
    }
}

impl Iterator for MajorScale {
    type Item = Tone;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = self.current;
        match self.iter {
            0 => {},
            1 | 2 | 4 | 5 | 6 => num += 2,
            3 | 7 => num += 1,
            _=> return None
        }

        self.iter += 1;
        if self.iter == 8 {
            self.iter = 1;
        }

        self.current = num;

        Tone::from_semitones_since_c0(num)
    }
}
