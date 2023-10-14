use bar::Line;

pub mod notes;
pub mod tempo;
pub mod bar;
mod math;

#[derive(Debug, Clone)]
pub struct MusicSheet {
    pub title: String,
    pub subtitle: String,
    pub author: String,
    pub lines: Vec<Line>
}