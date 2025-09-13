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

struct InstanceInput {
    @location(5) transform_matrix_0: vec4<f32>,
    @location(6) transform_matrix_1: vec4<f32>,
    @location(7) transform_matrix_2: vec4<f32>,
    @location(8) transform_matrix_3: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
}

@vertex
fn vert_main(
    @location(0) vert_pos: vec3<f32>,
    instance: InstanceInput,
) -> VertexOutput {
    let instance_transform = mat4x4<f32>(
        instance.transform_matrix_0,
        instance.transform_matrix_1,
        instance.transform_matrix_2,
        instance.transform_matrix_3,
    );

    let pos = vec4<f32>(vert_pos, 1.0);
    let world_pos = instance_transform * model_transform * pos;

    var out: VertexOutput;
    out.clip_position = view_proj_matrix * world_pos;
    return out;
}
