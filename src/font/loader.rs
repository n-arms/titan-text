use fontdb::{Database, Query};
use swash::FontRef;

use super::{Error, Font};
use anyhow::Result;

pub struct Loader<'a> {
    database: Database,
    backing: &'a mut Vec<u8>,
}

impl<'a> Loader<'a> {
    pub fn system(backing: &'a mut Vec<u8>) -> Self {
        let mut database = Database::new();
        database.load_system_fonts();
        Self { database, backing }
    }

    pub fn load_font(&mut self, query: &Query) -> Result<Font> {
        let id = self
            .database
            .query(query)
            .ok_or_else(|| Error::CouldNotLoadFont(format!("{query:?}")))?;

        let (range, face) = self
            .database
            .with_face_data(id, |font_data, face_index| {
                let start = self.backing.len();
                self.backing.extend_from_slice(font_data);
                (start..start + font_data.len(), face_index as usize)
            })
            .ok_or_else(|| Error::CouldNotParseFont(format!("{query:?}")))?;
        let font_ref = FontRef::from_index(&self.backing[range], face)
            .ok_or_else(|| Error::CouldNotParseFont(format!("{query:?}")))?;

        Ok(Font::new(font_ref))
    }
}
