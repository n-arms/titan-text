mod font;
mod preproc;

use anyhow::Result;

fn main() -> Result<()> {
    let mut buf = Vec::new();
    let mut loader = font::Loader::system(&mut buf);
    let query = fontdb::Query {
        families: &[fontdb::Family::SansSerif],
        weight: fontdb::Weight::NORMAL,
        stretch: fontdb::Stretch::Normal,
        style: fontdb::Style::Normal,
    };
    let font = loader.load_font(&query)?;
    let atlas = preproc::Atlas::new(1024, 1024);
    let mut proc = preproc::Preprocessor::new(font, atlas, 12.);
    proc.add_str("Hello World!")?;

    Ok(())
}
