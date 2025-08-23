// shader to render debug lines
// Bind groups:
// 0: Once per scene render
//   0: camera matrix
// 1: Once per model

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vert_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera_matrix * vec4<f32>(position, 1.0);
    out.color = color;
    return out;
}


// frag shader ---------------------------------------

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}