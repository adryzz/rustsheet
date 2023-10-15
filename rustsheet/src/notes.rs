use crate::tempo::NoteSize;
use std::{fmt, default};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use tinyvec::ArrayVec;

#[derive(Debug, Clone, Copy)]
pub enum Note {
    Note(NoteInfo),
    Tuplet(ArrayVec<[NoteInfo; crate::MAX_NOTES_IN_TUPLET]>),
    WithGrace { grace: Tone, note: NoteInfo },
    Rest(NoteSize)
}

impl Default for Note {
    fn default() -> Self {
        Note::Note(Default::default())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NoteInfo {
    pub tone: Tone,
    pub size: NoteSize
}

#[derive(Debug, Clone, Copy)]
pub struct Tone {
    pub octave: Octave,
    pub position: u8,
    pub modifiers: Option<ToneModifiers>,
}

// default is C4
impl Default for Tone {
    fn default() -> Self {
        Self {
            octave: Octave::C,
            position: 4,
            modifiers: None,
        }
    }
}

impl Tone {
    pub const fn new(octave: Octave, position: u8) -> Self {
        Self {
            octave,
            position,
            modifiers: None,
        }
    }

    pub const fn with_modifiers_unchecked(
        octave: Octave,
        position: u8,
        modifiers: ToneModifiers,
    ) -> Self {
        Self {
            octave,
            position,
            modifiers: Some(modifiers),
        }
    }

    pub fn with_modifiers(octave: Octave, position: u8, modifiers: ToneModifiers) -> Option<Self> {
        if octave == Octave::C && position == 0 && modifiers == ToneModifiers::Flat {
            return None;
        }

        Some(Self {
            octave,
            position,
            modifiers: Some(modifiers),
        })
    }

    pub fn from_semitones_since_c0(semitones: u32) -> Option<Tone> {
        // B255 is the highest representable note
        if semitones > 3071 {
            return None;
        }

        let rem = semitones % 12;
        let position = ((semitones - rem) / 12) as u8;

        match rem {
            0 => Some(Tone::new(Octave::C, position)),
            1 => Some(Tone::with_modifiers_unchecked(
                Octave::C,
                position,
                ToneModifiers::Sharp,
            )),
            2 => Some(Tone::new(Octave::D, position)),
            3 => Some(Tone::with_modifiers_unchecked(
                Octave::D,
                position,
                ToneModifiers::Sharp,
            )),
            4 => Some(Tone::new(Octave::E, position)),
            5 => Some(Tone::new(Octave::F, position)),
            6 => Some(Tone::with_modifiers_unchecked(
                Octave::F,
                position,
                ToneModifiers::Sharp,
            )),
            7 => Some(Tone::new(Octave::G, position)),
            8 => Some(Tone::with_modifiers_unchecked(
                Octave::G,
                position,
                ToneModifiers::Sharp,
            )),
            9 => Some(Tone::new(Octave::A, position)),
            10 => Some(Tone::with_modifiers_unchecked(
                Octave::A,
                position,
                ToneModifiers::Sharp,
            )),
            11 => Some(Tone::new(Octave::B, position)),
            12 => Some(Tone::new(Octave::C, position)),
            _ => None,
        }
    }

    pub fn get_semitones_since_c0(&self) -> u32 {
        let mut semitones = self.position as u32 * 12;
        semitones += self.octave as u32;
        match self.modifiers {
            Some(ToneModifiers::Sharp) => semitones += 1,
            // FIXME: dont allow C0 flat
            Some(ToneModifiers::Flat) => semitones -= 1,
            _ => {}
        };

        semitones
    }
}

impl PartialEq for Tone {
    fn eq(&self, other: &Self) -> bool {
        self.get_semitones_since_c0()
            .eq(&other.get_semitones_since_c0())
    }
}

impl Eq for Tone {}

impl PartialOrd for Tone {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_semitones_since_c0()
            .partial_cmp(&other.get_semitones_since_c0())
    }
}

impl Ord for Tone {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_semitones_since_c0()
            .cmp(&other.get_semitones_since_c0())
    }
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.modifiers {
            None => write!(f, "{}{}", self.octave, self.position),
            Some(modifiers) => write!(f, "{}{}{}", self.octave, self.position, modifiers),
        }
    }
}

impl FromStr for Tone {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // u8 max value is 3 digits, so check that value is at most 5 and at least 2
        if s.len() > 4 || s.len() < 2 {
            return Err(NoteError::InvalidNote);
        }

        let oct = s.chars().nth(0).unwrap();

        let octave = Octave::try_from(oct)?;
        let num = &s[1..];
        Ok(Tone {
            octave,
            position: u8::from_str(num)?,
            // not implemented yet
            modifiers: None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// semitones since C
pub enum Octave {
    A = 9,
    B = 11,
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
}
impl TryFrom<char> for Octave {
    type Error = NoteError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Octave::A),
            'B' => Ok(Octave::B),
            'C' => Ok(Octave::C),
            'D' => Ok(Octave::D),
            'E' => Ok(Octave::E),
            'F' => Ok(Octave::F),
            'G' => Ok(Octave::G),
            _ => Err(NoteError::InvalidOctave),
        }
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Octave::A => write!(f, "A"),
            Octave::B => write!(f, "B"),
            Octave::C => write!(f, "C"),
            Octave::D => write!(f, "D"),
            Octave::E => write!(f, "E"),
            Octave::F => write!(f, "F"),
            Octave::G => write!(f, "G"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ToneModifiers {
    Sharp = 2,
    Flat = 0,
    Natural = 1,
}

impl fmt::Display for ToneModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToneModifiers::Sharp => write!(f, "♯"),
            ToneModifiers::Flat => write!(f, "♭"),
            ToneModifiers::Natural => write!(f, "♮"),
        }
    }
}

#[derive(Debug, Clone, Error)]
pub enum NoteError {
    #[error("Invalid note")]
    InvalidNote,
    #[error("Invalid octave")]
    InvalidOctave,
    #[error("Invalid octave number")]
    InvalidNumber(#[from] ParseIntError),
}
