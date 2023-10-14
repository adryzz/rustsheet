use crate::tempo::NoteSize;
use std::num::ParseIntError;
use std::str::FromStr;
use std::fmt;
use thiserror::Error;
use tinyvec::ArrayVec;

#[derive(Debug, Clone, Copy)]
pub enum NoteWhateverFixMeFindANewNamePleaseImBeggingYou {
    Note(Note),
    Tuplet(ArrayVec<[Note; crate::MAX_NOTES_IN_TUPLET]>),
    // TODO: find if we even need a full Note for the grace or just its pitch
    WithGrace(Note, Note),
}

impl Default for NoteWhateverFixMeFindANewNamePleaseImBeggingYou {
    fn default() -> Self {
        NoteWhateverFixMeFindANewNamePleaseImBeggingYou::Note(Default::default())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Note {
    pub pitch: NotePitch,
    pub size: NoteSize,
    pub ty: NoteType,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum NoteType {
    #[default]
    Note,
    Rest,
}

#[derive(Debug, Clone, Copy)]
pub struct NotePitch {
    pub octave: Octave,
    pub position: u8,
    pub modifiers: Option<NotePitchModifiers>,
}

// default is C4
impl Default for NotePitch {
    fn default() -> Self {
        Self {
            octave: Octave::C,
            position: 4,
            modifiers: None,
        }
    }
}

impl NotePitch {
    pub fn get_semitones_since_c0(&self) -> u32 {
        let mut semitones = self.position as u32 * 12;
        semitones += self.octave as u32;
        match self.modifiers {
            Some(NotePitchModifiers::Sharp) => semitones += 1,
            // FIXME: dont allow C0 flat
            Some(NotePitchModifiers::Flat) => semitones -= 1,
            _ => {}
        };

        semitones
    }
}

impl PartialEq for NotePitch {
    fn eq(&self, other: &Self) -> bool {
        self.get_semitones_since_c0()
            .eq(&other.get_semitones_since_c0())
    }
}

impl Eq for NotePitch {}

impl PartialOrd for NotePitch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_semitones_since_c0()
            .partial_cmp(&other.get_semitones_since_c0())
    }
}

impl Ord for NotePitch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_semitones_since_c0()
            .cmp(&other.get_semitones_since_c0())
    }
}

impl fmt::Display for NotePitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.octave, self.position)
    }
}

impl FromStr for NotePitch {
    type Err = NoteError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // u8 max value is 3 digits, so check that value is at most 5 and at least 2
        if s.len() > 4 || s.len() < 2 {
            return Err(NoteError::InvalidNote);
        }

        let oct = s.chars().nth(0).unwrap();

        let octave = Octave::try_from(oct)?;
        let num = &s[1..];
        Ok(NotePitch {
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
pub enum NotePitchModifiers {
    Sharp = 2,
    Flat = 0,
    Natural = 1,
}

impl fmt::Display for NotePitchModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotePitchModifiers::Sharp => write!(f, "♯"),
            NotePitchModifiers::Flat => write!(f, "♭"),
            NotePitchModifiers::Natural => write!(f, "♮"),
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
