use crate::{notes::Tone, tempo::TimeSignature};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotePosition {
    pub line: usize,
    pub bar: usize,
    pub index: usize,
}

impl PartialOrd for NotePosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // you can only order notes if they're on the same line
        if self.line != other.line {
            return None;
        }

        match self.bar.partial_cmp(&other.bar) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.index.partial_cmp(&other.index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Clef {
    Treble,
    Baritone,
    Bass,
    SubBass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BarHeader {
    pub clef: Option<Clef>,
    pub time_signature: Option<TimeSignature>,
    pub attributes: [Option<Tone>; 8],
}

#[derive(Debug, Clone)]
pub struct Bar {}

#[derive(Debug, Clone)]
pub struct Line {
    pub name: Option<String>,
    pub bars: Vec<Bar>,
    // represent all the stuff on top
}
