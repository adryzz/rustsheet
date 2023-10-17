use crate::{
    notes::{Note, Octave, Tone},
    tempo::TimeSignature,
};

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
    Tenor,
    Bass,
    /// No clef, places the selected tone as the center line
    None {
        center: Tone,
    },
}

impl Default for Clef {
    fn default() -> Self {
        Clef::None {
            center: Tone::new(Octave::C, 4),
        }
    }
}

impl Clef {
    pub fn get_center_tone(&self) -> Tone {
        match self {
            Clef::Treble => Tone::new(Octave::B, 4),
            Clef::Tenor => Tone::new(Octave::C, 4),
            Clef::Bass => Tone::new(Octave::D, 3),
            Clef::None { center } => *center,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BarHeader {
    pub clef: Option<Clef>,
    pub time_signature: Option<TimeSignature>,
    pub attributes: [Option<Tone>; 8],
}

impl BarHeader {
    /// A sane default for starting a new line.
    /// Comes with a bass clef and a 4/4 time signature
    pub fn sane_default() -> Self {
        Self {
            clef: Some(Clef::Bass),
            time_signature: Some(Default::default()),
            attributes: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Bar {
    pub header: BarHeader,
    pub notes: Vec<Note>,
}

impl Bar {
    pub fn is_empty(&self) -> bool {
        // FIXME: is this a crappy way to check?
        self.notes.len() == 0 && self.header == Default::default()
    }

    pub fn check_time_signature(&self, signature: TimeSignature) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct SingleLine {
    pub bars: Vec<Bar>,
    // represent all the stuff on top
}

impl Default for SingleLine {
    fn default() -> Self {
        let mut bars: Vec<Bar> = Vec::with_capacity(crate::STARTING_BAR_CAPACITY);

        bars.push(Bar::default());

        Self { bars }
    }
}

impl SingleLine {
    pub fn sane_default() -> Self {
        let mut bars: Vec<Bar> = Vec::with_capacity(crate::STARTING_BAR_CAPACITY);

        let bar = Bar {
            header: BarHeader::sane_default(),
            notes: Default::default(),
        };

        bars.push(bar);

        Self { bars }
    }
}

#[derive(Debug, Clone)]
pub enum LineType {
    Single(SingleLine),
    Double(SingleLine, SingleLine),
}

impl From<SingleLine> for LineType {
    fn from(value: SingleLine) -> Self {
        LineType::Single(value)
    }
}

impl Default for LineType {
    fn default() -> Self {
        LineType::Single(Default::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub name: Option<String>,
    pub line: LineType,
}

impl Line {
    pub fn new(inner: LineType) -> Self {
        Self {
            name: None,
            line: inner,
        }
    }

    pub fn new_named(name: String, inner: LineType) -> Self {
        Self {
            name: Some(name),
            line: inner,
        }
    }
}
