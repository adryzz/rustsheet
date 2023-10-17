mod line;

use rustsheet::{bar::{LineType, Bar}, MusicSheet};
use svg::{
    node::{
        element::{Line, Text as TextElement, Rectangle},
        Text,
    },
    Document,
};

const VIEWBOX: &str = "viewBox";
const FONT_SIZE: &str = "font-size";
const TEXT_ANCHOR: &str = "text-anchor";
pub struct MusicSheetSVGRenderer {}

impl MusicSheetSVGRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_sheet(&self, sheet: &MusicSheet, config: &RendererConfig) -> Vec<u8> {
        let title = h1_centered(&sheet.title, 100);
        let subtitle = sheet.subtitle.as_ref().map(|s| h2_centered(s, 150));
        let author = Text::new(&sheet.author);
        let mut doc = Document::new().set(VIEWBOX, (0, 0, 500, 1000)).add(title);

        if let Some(t) = subtitle {
            doc = doc.add(t);
        }

        let mut s = Vec::new();

        svg::write(&mut s, &doc).unwrap();

        s
    }

    pub fn render_line(&self, line: &LineType, config: &RendererConfig) {}

    pub fn render_bar(&self, bar: &Bar, config: &RendererConfig) -> Vec<u8> {
        let rendered = line::generate_bar(10, 10, true, bar, config);
        let doc = Document::new()
        .set(VIEWBOX, (0, 0, 800, 400))
        .add(bg_color(config.background_color))
        .add(rendered);
        let mut s = Vec::new();

        svg::write(&mut s, &doc).unwrap();

        s
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RendererConfig<'a> {
    pub background_color: &'a str,
    pub octave_colors: [&'a str; 7],
    pub line_help_colors: [&'a str; 8],
    pub error_checking: bool,
}

impl Default for RendererConfig<'static> {
    fn default() -> Self {
        Self { 
            background_color: "white",
            octave_colors: ["black"; 7],
            line_help_colors: ["#00000000"; 8],
            error_checking: false
        }
    }
}

impl<'a> RendererConfig<'a> {
    pub fn default_accessibility() -> Self {
        Self { 
            background_color: "#EFEFEF",
            octave_colors: ["black"; 7],
            line_help_colors: ["#00000000"; 8],
            error_checking: false
        }
    }
}

fn bg_color(color: &str) -> Rectangle {
    Rectangle::new().set("width", "100%")
    .set("height", "100%")
    .set("fill", color)
}

fn h1_centered(text: &str, height: u32) -> TextElement {
    TextElement::new()
        .add(Text::new(text))
        .set(FONT_SIZE, 48)
        .set("x", 250)
        .set("y", height)
        .set(TEXT_ANCHOR, "middle")
}

fn h2_centered(text: &str, height: u32) -> TextElement {
    TextElement::new()
        .add(Text::new(text))
        .set(FONT_SIZE, 36)
        .set("x", 250)
        .set("y", height)
        .set(TEXT_ANCHOR, "middle")
}

/*fn music_line(x: u32, y: u32) -> [Line; 5] {
    Line::new().set("x1", x).set("y1")
}*/
