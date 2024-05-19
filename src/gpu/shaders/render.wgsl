struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texture_position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) texture_position: vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    vertex: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(vertex.position.x / 64, vertex.position.y / 64, 0, 1);
    out.texture_position = vertex.texture_position;
    return out;
}

@group(0) @binding(0)
var atlas_texture: texture_2d<f32>;
@group(0) @binding(1)
var atlas_sampler: sampler;

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(atlas_texture, atlas_sampler, vertex.texture_position);
}