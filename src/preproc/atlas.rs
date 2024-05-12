use std::collections::HashMap;

use crate::font::LoadedGlyph;

pub type AtlasID = u16;

pub struct Atlas {
    entries: HashMap<u32, AtlasGlyph>,
    width: u32,
    height: u32,
    next_entry_x: u32,
    next_entry_y: u32,
    next_id: u16,
    glyph_height: u32,
}

pub struct AtlasGlyph {
    id: AtlasID,
    glyph: LoadedGlyph,
    x: u32,
    y: u32,
}

impl Atlas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            entries: HashMap::new(),
            width,
            height,
            next_entry_x: 0,
            next_entry_y: 0,
            next_id: 0,
            glyph_height: 0,
        }
    }

    pub fn store_glyph(&mut self, codepoint: impl Into<u32>, glyph: LoadedGlyph) {
        self.glyph_height = self.glyph_height.max(glyph.image.placement.height);
        let outer_edge = glyph.image.placement.width + self.next_entry_x;
        let (x, y) = if outer_edge >= self.width {
            self.next_entry_y += self.glyph_height;
            self.next_entry_x = glyph.image.placement.width;
            (0, self.next_entry_y)
        } else {
            let p = (self.next_entry_x, self.next_entry_y);
            self.next_entry_x += glyph.image.placement.width;
            p
        };
        let id = self.next_id;
        self.next_id += 1;

        self.entries
            .insert(codepoint.into(), AtlasGlyph { id, glyph, x, y });
    }

    pub fn get_glyph_id(&self, codepoint: impl Into<u32>) -> Option<AtlasID> {
        let glyph = self.entries.get(&codepoint.into())?;
        Some(glyph.id)
    }
}
