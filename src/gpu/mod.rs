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

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineSize {
    /// the index in the text buffer where the line starts
    pub start: u32,
    /// the length of the line
    pub length: u32,
}

// TODO: implement more fine grained line length systems
pub struct Text {
    /// a buffer of u16 values pointing into the glyph data buffer
    text: wgpu::Buffer,
    /// a buffer of LineSize's
    size: wgpu::Buffer,
    lines: u32,
    line_length: u32,
}
