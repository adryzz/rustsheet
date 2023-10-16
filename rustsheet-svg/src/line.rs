use rustsheet::bar::Bar;
use svg::{node::element::{Line, Group, Circle}, Node};

const VERTICAL_NOTE_MARGIN: usize = 5;
const LINES_MARGIN: usize = 2 * VERTICAL_NOTE_MARGIN;
const LINES_THICKNESS: usize = 1;
const STROKE_COLOR: &str = "black";
const STROKE: &str = "stroke";
const STROKE_WIDTH: &str = "stroke-width";

const NOTES_MARGIN: usize = 25;
const MIN_SIZE_BAR: usize = 50;
const NOTE_SIZE: usize = 25;
const CLEF_SIZE: usize = 40;
const TIME_SIGNATURE_SIZE: usize = 25;
const END_BAR_MARGIN: usize = 5;

pub fn generate_bar(x: usize, y: usize, end: bool, bar: &Bar) -> Group {
    // calculate bar length
    let mut bar_size = MIN_SIZE_BAR;
    if bar.header.clef.is_some() {
        bar_size += CLEF_SIZE;
        bar_size += NOTES_MARGIN;
    }

    if bar.header.time_signature.is_some() {
        bar_size += TIME_SIGNATURE_SIZE;
        bar_size += NOTES_MARGIN;
    }
    // margin left, note, margin right
    bar_size += bar.notes.len() * ((2 * NOTES_MARGIN) + NOTE_SIZE);

    if end {
        bar_size += END_BAR_MARGIN;
    }

    let mut g = Group::new();

    // starting vertical line
    g.append(
        Line::new()
            .set("x1", x)
            .set("x2", x)
            .set("y1", y)
            .set("y2", y + (4 * LINES_MARGIN))
            .set(STROKE, STROKE_COLOR)
            .set(STROKE_WIDTH, LINES_THICKNESS),
    );

    // the 5 bar lines
    for i in 0..5 {
        let cur_y = y + (i * LINES_MARGIN);
        g.append(
            Line::new()
                .set("x1", x)
                .set("x2", x + bar_size)
                .set("y1", cur_y)
                .set("y2", cur_y)
                .set(STROKE, STROKE_COLOR)
                .set(STROKE_WIDTH, LINES_THICKNESS),
        );
    }

    // if it's the last bar, draw one thin line, otherwise do a thin and a thick
    if !end {
        g.append(
            Line::new()
                .set("x1", x + bar_size)
                .set("x2", x + bar_size)
                .set("y1", y)
                .set("y2", y + (4 * LINES_MARGIN))
                .set(STROKE, STROKE_COLOR)
                .set(STROKE_WIDTH, LINES_THICKNESS),
        );
    } else {
        g.append(
            Line::new()
                .set("x1", x + bar_size - END_BAR_MARGIN)
                .set("x2", x + bar_size - END_BAR_MARGIN)
                .set("y1", y)
                .set("y2", y + (4 * LINES_MARGIN))
                .set(STROKE, STROKE_COLOR)
                .set(STROKE_WIDTH, LINES_THICKNESS),
        );
        g.append(
            Line::new()
                .set("x1", x + bar_size)
                .set("x2", x + bar_size)
                .set("y1", y)
                .set("y2", y + (4 * LINES_MARGIN))
                .set(STROKE, STROKE_COLOR)
                .set(STROKE_WIDTH, 2 * LINES_THICKNESS),
        );
    }

    if let Some(clef) = bar.header.clef {
        // draw clef
        
    }

    if let Some(ts) = bar.header.time_signature {
        // draw time signature
    }

    for note in &bar.notes {
        // draw notes
        
    }

    g
}