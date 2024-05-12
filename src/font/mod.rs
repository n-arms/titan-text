use std::cell::OnceCell;

use anyhow::Result;
use swash::{
    scale::{image::Image, Render, ScaleContext, Scaler, Source, StrikeWith},
    zeno::{Format, Vector},
    Charmap, FontRef,
};
use thiserror::Error;

mod loader;

pub struct Font<'a> {
    inner: FontRef<'a>,
    charmap: Charmap<'a>,
    render: Render<'a>,
}

pub struct LoadedGlyph {
    pub image: Image,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Swash couldn't render codepoint {0}")]
    CouldNotRender(u32),
}

impl<'a> Font<'a> {
    pub fn new(inner: FontRef<'a>, point: f32) -> Self {
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

    pub fn load_glyph(
        &mut self,
        codepoint: impl Into<u32> + Clone,
        point: f32,
    ) -> Result<LoadedGlyph> {
        let id = self.charmap.map(codepoint.clone());
        let mut context = ScaleContext::new();
        let mut scaler = context.builder(self.inner).hint(true).size(point).build();
        let image = self
            .render
            .format(Format::Subpixel)
            .offset(Vector::new(0., 0.))
            .render(&mut scaler, id)
            .ok_or(Error::CouldNotRender(codepoint.into()))?;
        Ok(LoadedGlyph { image })
    }
}
