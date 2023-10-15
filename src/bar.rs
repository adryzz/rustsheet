use crate::{notes::{Tone, NoteWhateverFixMeFindANewNamePleaseImBeggingYou}, tempo::TimeSignature};

pub const STARTING_BAR_CAPACITY: usize = 4;

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
            attributes: Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Bar {
    pub header: BarHeader,
    pub notes: Vec<NoteWhateverFixMeFindANewNamePleaseImBeggingYou>
}

impl Bar {
    pub fn is_empty(&self) -> bool {
        // FIXME: is this a crappy way to check?
        self.notes.len() == 0 && self.header == Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct SingleLine {
    pub bars: Vec<Bar>,
    // represent all the stuff on top
}

impl Default for SingleLine {
    fn default() -> Self {
        let mut bars: Vec<Bar> = Vec::with_capacity(STARTING_BAR_CAPACITY);

        bars.push(Bar::default());

        Self { bars }
    }
}

impl SingleLine {
    fn sane_default() -> Self {
        let mut bars: Vec<Bar> = Vec::with_capacity(STARTING_BAR_CAPACITY);

        let bar = Bar {
            header: BarHeader::sane_default(),
            notes: Default::default()
        };

        bars.push(bar);

        Self { bars }
    }
}

#[derive(Debug, Clone)]
pub enum LineType {
    Single(SingleLine),
    Double(SingleLine, SingleLine)
}

impl Default for LineType {
    fn default() -> Self {
        LineType::Single(Default::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Line {
    pub name: Option<String>,
    pub line: LineType
}