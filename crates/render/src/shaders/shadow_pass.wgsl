// shader to render the diffuse material in the scene
// Bind groups:
// 0: Once per scene render
//   0: view proj matrix
// 1: Once per model
//   0: model transform matrix

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> view_proj_matrix: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> model_transform: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vert_main(
    @location(0) v_position: vec3<f32>,
) -> VertexOutput {
    let pos = vec4<f32>(v_position, 1.0);
    let world_pos = model_transform * pos;

    var out: VertexOutput;
    out.clip_position = view_proj_matrix * world_pos;
    return out;
}
