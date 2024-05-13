mod command;
mod layout;
mod publish;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GpuGlyphData {
    /// the x position of the start of the glyph in the atlas texture
    pub texture_x: u32,
    /// the y position of the start of the glyph in the atlas texture
    pub texture_y: u32,
    /// the width of the glyph in the atlas texture
    pub width: u32,
    /// the height of the glyph in the atlas texture
    pub height: u32,
    /// god knows what this means
    pub left: i32,
    /// god knows what this means
    pub top: i32,
    /// how much to move the cursor forwards after printing the glyph
    pub advance_x: f32,
}
