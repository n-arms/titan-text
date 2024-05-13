struct LineSize {
    start: u32,
    length: u32
}

struct GlyphData {
    texture_x: u32,
    texture_y: u32,
    width: u32,
    height: u32,
    left: i32,
    top: i32,
    advance_x: f32
}

struct Vertex {
    position: vec2<f32>,
    texture_position: vec2<f32>,
}

@group(0)
@binding(0)
var<storage, read_write> text: array<u16>;

@group(0)
@binding(1)
var<storage, read_write> size: array<LineSize>;

@group(0)
@binding(2)
var<storage, read_write> glyph_data: array<GlyphData>;

@group(0)
@binding(3)
var<storage, read_write> layout: array<f32>;

@group(0)
@binding(4)
var<storage, read_write> vertex: array<Vertex>;

@group(0)
@binding(5)
var<storage, read_write> index: array<u32>;

@compute
@workgroup_size(64, 1)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let lineSize = size[workgroup_id.y];
    let start = lineSize.start;
    let text_index = start + local_id.x;
    let glyph_index = text[text_index];
    let glyph = glyph_data[glyph_index];
}
