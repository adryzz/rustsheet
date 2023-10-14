use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use crate::tempo::NoteSize;


#[derive(Debug, Clone, Copy)]
pub struct Note {
    pub pitch: NotePitch,
    pub size: NoteSize,
    pub r#type: NoteType

}

#[derive(Debug, Clone, Copy)]
pub enum NoteType {
    Note,
    Rest
}

#[derive(Debug, Clone, Copy)]
pub struct NotePitch {
    pub octave: Octave,
    pub position: u8,
    pub modifiers: Option<NotePitchModifiers>
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
            modifiers: None
        })
        
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Octave {
    A,
    B,
    C,
    D,
    E,
    F,
    G
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
            _ => Err(NoteError::InvalidOctave)
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

#[derive(Debug, Clone, Copy)]
pub enum NotePitchModifiers {
    Sharp,
    Flat,
    Natural
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
    InvalidNumber(#[from] ParseIntError)
}