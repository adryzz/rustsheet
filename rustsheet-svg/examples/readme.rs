use std::io::Write;

use rustsheet::{
    bar::{Bar, BarHeader, Line, SingleLine},
    notes::{Note, NoteInfo, Octave, Tone},
    tempo::{NoteSize, NoteSizeUnit},
    MusicSheet,
};
use rustsheet_svg::{MusicSheetSVGRenderer, RendererConfig};
use rustsheet_utils::{scales::MajorScale, tones};

/// This example renders the line shown in the README
fn main() {
    // create a single line of 3 bars, 4/4 bass clef with 1 beat large notes (G2 to C3)
    let bar = Bar {
        header: BarHeader::sane_default(),
        notes: vec![
            Note::Note(NoteInfo {
                tone: tones::G2,
                ..Default::default()
            }),
            Note::Note(NoteInfo {
                tone: tones::A2,
                ..Default::default()
            }),
            Note::Note(NoteInfo {
                tone: tones::B2,
                ..Default::default()
            }),
            Note::Note(NoteInfo {
                tone: tones::C3,
                ..Default::default()
            }),
        ],
    };

    let renderer = MusicSheetSVGRenderer::new();

    let out = renderer.render_bar(&bar, &RendererConfig::default_accessibility());

    std::fs::File::create("example.svg")
        .unwrap()
        .write_all(&out)
        .unwrap();
}
