// shader to render the diffuse material in the scene
// Bind groups:
// 0: Once per scene render
//   0: camera matrix
// 1: Once per model
//   0: pos
//   1: rotation
// 2: Once per mesh/material
//   0: mesh diffuse color

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
}

@vertex
fn vert_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera_matrix * vec4<f32>(position, 1.0);
    out.normal = normal;
    return out;
}


// frag shader ---------------------------------------

@group(2) @binding(0)
var<uniform> diffuse_color: vec4<f32>;

const sun_direction = vec3<f32>(1.0, -2.0, 1.0);
const ambient_shade_multiplier: f32 = 0.5;
const ambient_shade_threshold: f32 = 0.45;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {

    var color = diffuse_color.rgb;

    let normal = normalize(in.normal);
    let angle_from_sun = acos(dot(normal, normalize(sun_direction)));
    if (angle_from_sun < (3.141592 * ambient_shade_threshold)) {
        // in ambient shade
        color *= ambient_shade_multiplier; 
    }

    return vec4<f32>(color, 1.0);
}