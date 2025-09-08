// shader to render the skid lines
// Bind groups:
// 0: Once per scene render
//   0: camera matrix
//   1: sun direction vector
//   2: shadow map view proj matrix
//   3: shadow map texture view
//   4: shadow map sampler

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) index: f32,
}

@vertex
fn vert_main(
    @location(0) v_position: vec3<f32>,
    @builtin(vertex_index) v_index: u32,
) -> VertexOutput {
    let world_pos = vec4<f32>(v_position, 1.0);

    var out: VertexOutput;
    out.clip_position = camera_matrix * world_pos;
    out.index = f32(v_index);
    return out;
}

// frag shader ---------------------------------------

const col: f32 = 0.005;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(col, col, col, 0.8);
}