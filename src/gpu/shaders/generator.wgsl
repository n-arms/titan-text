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

struct FontData {
    line_height: f32
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
var<storage, read_write> font_data: FontData;

@group(0)
@binding(5)
var<storage, read_write> vertex: array<Vertex>;

@group(0)
@binding(6)
var<storage, read_write> index: array<u32>;

var next_vertex: atomic<u32> = 0;
var next_index: atomic<u32> = 0;

fn allocate_vertex() -> u32 {
    return atomicAdd(&next_vertex, 1);
}

fn allocate_triangle_indices() -> u32 {
    return atomicAdd(&next_index, 3);
}

@compute
@workgroup_size(64, 1)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let line_size = size[workgroup_id.y];
    let preceeding_lines = workgroup_id.y;
    let start = line_size.start;
    let text_index = start + local_id.x;
    let glyph_index = text[text_index];
    let glyph = glyph_data[glyph_index];
    let layout_offset = layout[text_index];

    let top = f32(preceeding_lines + 1) * font_data.line_height - f32(glyph.top);
    let bottom = top + f32(glyph.height);

    let left = layout_offset + glyph.left;
    let right = left + glyph.width;

    let a = vec2<f32>(top, left);
    let b = vec2<f32>(top, right);
    let c = vec2<f32>(bottom, left);
    let d = vec2<f32>(bottom, right);

    let a_i = allocate_vertex();
    let b_i = allocate_vertex();
    let c_i = allocate_vertex();
    let d_i = allocate_vertex();

    vertex[a_i] = a;
    vertex[b_i] = b;
    vertex[c_i] = c;
    vertex[d_i] = d;

    let first = allocate_triangle_indices();
    let second = allocate_triangle_indices();

    index[first] = a;
    index[first + 1] = c;
    index[first + 2] = b;

    index[second] = b;
    index[second + 1] = c;
    index[second + 2] = d;
}