use bar::Line;

pub mod bar;
mod math;
pub mod notes;
pub mod tempo;

pub const MAX_NOTES_IN_TUPLET: usize = 5;

#[derive(Debug, Clone)]
pub struct MusicSheet {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub lines: Vec<Line>,
}
