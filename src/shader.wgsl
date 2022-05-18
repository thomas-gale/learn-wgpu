// Vertex shader

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] colour: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(1 - i32(in_vertex_index)) * 0.5;
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.colour = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

// Brown Fragment shader

[[stage(fragment)]]
fn fs_brown(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}


// Position Coloured Fragment shader

[[stage(fragment)]]
fn fs_colour(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.colour; 
}
 