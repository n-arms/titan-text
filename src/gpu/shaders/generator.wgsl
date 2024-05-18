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

struct IndexData {
    next_vertex: atomic<u32>,
    next_index: atomic<u32>
}

@group(0)
@binding(0)
var<storage, read_write> text: array<u32>;

@group(0)
@binding(1)
var<storage, read_write> size: array<LineSize>;

@group(0)
@binding(2)
var<storage, read_write> glyph_data: array<GlyphData>;

@group(0)
@binding(3)
var<storage, read_write> layout_offset: array<f32>;

@group(0)
@binding(4)
var<storage, read_write> font_data: FontData;

@group(0)
@binding(5)
var<storage, read_write> vertex: array<Vertex>;

@group(0)
@binding(6)
var<storage, read_write> index: array<u32>;

@group(0)
@binding(7)
var<storage, read_write> num_indices: IndexData;

fn allocate_vertex() -> u32 {
    return atomicAdd(&num_indices.next_vertex, 1u);
}

fn allocate_triangle_indices() -> u32 {
    return atomicAdd(&num_indices.next_index, 3u);
}

@compute
@workgroup_size(64, 1)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let line_size = size[workgroup_id.y];
    if (local_id.x >= line_size.length) {
        return;
    }

    let start = line_size.start;
    let text_id = start + local_id.x;
    let glyph_id = text[text_id];
    let glyph = glyph_data[glyph_id];

    let offset = layout_offset[text_id];

    let a = vec2<f32>(offset, 10);
    let b = vec2<f32>(offset + 5, 10);
    let c = vec2<f32>(offset, 15);
    let d = vec2<f32>(offset + 5, 15);

    let a_t = vec2<f32>(f32(glyph.texture_x), f32(glyph.texture_y));
    let b_t = vec2<f32>(f32(glyph.texture_x), f32(glyph.texture_y) + f32(glyph.width));
    let c_t = vec2<f32>(f32(glyph.texture_x) + f32(glyph.height), f32(glyph.texture_y));
    let d_t = vec2<f32>(f32(glyph.texture_x) + f32(glyph.height), f32(glyph.texture_y) + f32(glyph.width));

    let a_i = allocate_vertex();
    let b_i = allocate_vertex();
    let c_i = allocate_vertex();
    let d_i = allocate_vertex();

    vertex[a_i].position = a;
    vertex[a_i].texture_position = a_t;
    vertex[b_i].position = b;
    vertex[b_i].texture_position = b_t;
    vertex[c_i].position = c;
    vertex[c_i].texture_position = c_t;
    vertex[d_i].position = d;
    vertex[d_i].texture_position = d_t;

    let first = allocate_triangle_indices();
    let second = allocate_triangle_indices();

    index[first] = a_i;
    index[first + 1] = c_i;
    index[first + 2] = b_i;

    index[second] = b_i;
    index[second + 1] = c_i;
    index[second + 2] = d_i;
}
