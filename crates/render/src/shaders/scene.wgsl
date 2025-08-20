// shader to render the diffuse material in the scene
// Bind groups:
// 0: Once per render
// 1: Once per mesh/material

// vert shader ---------------------------------------

struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    //@location(1) color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    //@location(0) color: vec3<f32>,
}

@vertex
fn vert_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    //out.color = model.color;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0); // 2.
    return out;
}


// frag shader ---------------------------------------

@group(1) @binding(0)
var<uniform> mesh_color: vec4<f32>;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return mesh_color;
}