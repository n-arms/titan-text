struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texture_position: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vs_main(
    vertex: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(vertex.position.x, vertex.position.y, 0, 1);
    return out;
}

@fragment
fn fs_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1, 0, 0, 1);
}