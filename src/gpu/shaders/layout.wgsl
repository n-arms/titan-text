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

const wgsize: u32 = 4;
const n: u32 = wgsize * 2;

var<workgroup> shared_data: array<f32, wgsize>;

fn glyph(id: u32) -> f32 {
    let data_id = text[id];
    return glyph_data[u32(data_id)];
}

@compute
@workgroup_size(4, 1)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) workgroup_id: vec3<u32>
) {
    let lineSize = size[workgroup_id.y];
    let start = lineSize.start;
    let tid = local_id.x;
    let offset: u32 = 1;
    shared_data[2*tid] = glyph(start + 2*tid);
    shared_data[2*tid + 1] = glyph(start + 2*tid + 1);

    for (var d: u32 = wgsize; d > 0; d = d >> 1) {
        workgroupBarrier();
        if (tid < d) {
            let a = offset*(2*tid + 1) - 1;
            let b = offset*(2*tid + 2) - 1;
            shared_data[b] += shared_data[a];
        }
        offset = offset * 2;
    }

    if (tid == 0) {
        shared_data[n - 1] = 0.0;
    }

    for (var d: u32 = 1; d < n; d = d * 2) {
        offset = offset / 2;
        workgroupBarrier();
        if (tid < d) {
            let a = offset*(2*tid + 1) - 1;
            let b = offset*(2*tid + 2) - 1;
            let temp = shared_data[a];
            shared_data[a] = shared_data[b];
            shared_data[b] += temp;
        }
    }

    workgroupBarrier();

    layout[start + 2*tid] = shared_data[2*tid];
    layout[start + 2*tid + 1] = shared_data[2*tid + 1];
}