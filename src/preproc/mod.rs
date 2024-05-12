use crate::font::Font;

use self::atlas::AtlasID;
use anyhow::Result;

mod atlas;
mod text;

pub use atlas::Atlas;

#[derive(Default)]
pub struct Line {
    glyphs: Vec<AtlasID>,
}

#[derive(Default)]
pub struct Text {
    lines: Vec<Line>,
}

pub struct Preprocessor<'a> {
    font: Font<'a>,
    atlas: Atlas,
    text: Text,
    point: f32,
}

impl<'a> Preprocessor<'a> {
    pub fn new(font: Font<'a>, atlas: Atlas, point: f32) -> Self {
        Self {
            font,
            atlas,
            text: Text::default(),
            point,
        }
    }

    fn codepoint(&mut self, codepoint: impl Into<u32>) -> Result<AtlasID> {
        let codepoint = codepoint.into();
        if let Some(id) = self.atlas.get_glyph_id(codepoint) {
            return Ok(id);
        }
        let glyph = self.font.load_glyph(codepoint, self.point)?;
        self.atlas.store_glyph(codepoint, glyph);
        let id = self.atlas.get_glyph_id(codepoint).expect(&format!(
            "Failed to look up codepoint {codepoint} from atlas"
        ));
        Ok(id)
    }

    pub fn add_text(&mut self, text: &text::Text) -> Result<()> {
        let mut lines = Vec::new();
        for line in &text.lines {
            let glyphs = line
                .text
                .chars()
                .map(|codepoint| self.codepoint(codepoint))
                .collect::<Result<Vec<_>, _>>()?;
            lines.push(Line { glyphs });
        }
        self.text.lines.extend(lines);
        Ok(())
    }

    pub fn add_str(&mut self, str: impl Into<String>) -> Result<()> {
        self.add_text(&text::Text::from(str.into()))
    }
}
