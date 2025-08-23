// shader to render debug lines
// Bind groups:
// 0: Once per scene render
//   0: camera matrix
// 1: Once per model
//   0: model position
//   1: model rotation

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> model_transform: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vert_main(
    @location(0) v_position: vec3<f32>,
    @location(1) v_color: vec3<f32>,
) -> VertexOutput {
    let pos = vec4<f32>(v_position, 1.0);
    let world_pos = model_transform * pos;

    var out: VertexOutput;
    out.clip_position = camera_matrix * world_pos;
    out.color = v_color;
    return out;
}


// frag shader ---------------------------------------

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}