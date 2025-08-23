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
var<uniform> model_pos: vec4<f32>;
@group(1) @binding(1)
var<uniform> model_rotation: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vert_main(
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
) -> VertexOutput {
    let translation: vec3<f32> = model_pos.xyz;
    let rotation = mat3x3<f32>(model_rotation[0].xyz, model_rotation[1].xyz, model_rotation[2].xyz);
    let world_position = (rotation * position) + translation;

    var out: VertexOutput;
    out.clip_position = camera_matrix * vec4<f32>(world_position, 1.0);
    out.color = color;
    return out;
}


// frag shader ---------------------------------------

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}