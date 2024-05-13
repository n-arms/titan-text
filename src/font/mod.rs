use std::cell::OnceCell;

use anyhow::Result;
use swash::{
    scale::{image::Image, Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::{Format, Vector},
    Charmap, FontRef,
};
use thiserror::Error;

mod loader;

pub use loader::Loader;

pub struct Font<'a> {
    inner: FontRef<'a>,
    charmap: Charmap<'a>,
    render: Render<'a>,
}

pub struct LoadedGlyph {
    pub image: Image,
    pub advance_width: f32,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Swash couldn't render codepoint {0}")]
    CouldNotRender(u32),
    #[error("Fontdb couldn't load font {0}")]
    CouldNotLoadFont(String),
    #[error("Fontdb couldn't parse font {0}")]
    CouldNotParseFont(String),
}

impl<'a> Font<'a> {
    pub fn new(inner: FontRef<'a>) -> Self {
        Self {
            inner,
            charmap: inner.charmap(),
            render: Render::new(&[
                Source::ColorOutline(0),
                Source::ColorBitmap(StrikeWith::BestFit),
                Source::Outline,
            ]),
        }
    }

    pub fn load_glyph(&mut self, codepoint: impl Into<u32>, point: f32) -> Result<LoadedGlyph> {
        let codepoint = codepoint.into();
        let id = self.charmap.map(codepoint);
        let mut context = ScaleContext::new();
        let mut scaler = context.builder(self.inner).hint(true).size(point).build();
        let image = self
            .render
            .format(Format::Subpixel)
            .offset(Vector::new(0., 0.))
            .render(&mut scaler, id)
            .ok_or(Error::CouldNotRender(codepoint))?;
        let advance_width = self.inner.glyph_metrics(&[]).advance_width(id);
        Ok(LoadedGlyph {
            image,
            advance_width,
        })
    }

    pub fn line_height(&self, point: f32) -> f32 {
        self.inner.metrics(&[]).scale(point).leading
    }
}
