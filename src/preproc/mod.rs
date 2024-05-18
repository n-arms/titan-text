use crate::font::Font;

use self::atlas::AtlasID;
use anyhow::Result;

mod atlas;
mod text;

pub use atlas::{Atlas, AtlasView};

#[derive(Debug, Default)]
pub struct Line {
    pub glyphs: Vec<AtlasID>,
}

#[derive(Debug)]
pub struct Text {
    pub lines: Vec<Line>,
    pub line_height: f32,
}

pub struct Preprocessor<'a> {
    pub font: Font<'a>,
    pub atlas: Atlas,
    pub text: Text,
    pub point: f32,
}

impl Text {
    pub fn new(line_height: f32) -> Self {
        Self {
            lines: Vec::new(),
            line_height,
        }
    }
}

impl<'a> Preprocessor<'a> {
    pub fn new(font: Font<'a>, atlas: Atlas, point: f32) -> Self {
        Self {
            text: Text::new(font.line_height(point)),
            font,
            atlas,
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
