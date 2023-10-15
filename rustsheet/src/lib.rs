use bar::Line;

pub mod bar;
mod math;
pub mod notes;
pub mod tempo;

const MAX_NOTES_IN_TUPLET: usize = 5;
const STARTING_BAR_CAPACITY: usize = 4;

#[derive(Debug, Clone)]
pub struct MusicSheet {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub lines: Vec<Line>,
}
