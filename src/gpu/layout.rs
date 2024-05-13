use super::Text;

pub struct LayoutPass<'a> {
    glyph_data: &'a wgpu::Buffer,
    atlas_texture: &'a wgpu::Texture,
    text: Text,
}
