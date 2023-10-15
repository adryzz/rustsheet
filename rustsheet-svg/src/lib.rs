use rustsheet::MusicSheet;

pub struct MusicSheetSVGRenderer {}

impl MusicSheetSVGRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render_sheet(&self, sheet: &MusicSheet) -> String {
        String::new()
    }
}
