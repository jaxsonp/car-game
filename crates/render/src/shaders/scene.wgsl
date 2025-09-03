// shader to render the diffuse material in the scene
// Bind groups:
// 0: Once per scene render
//   0: camera matrix
//   1: shadow map view proj matrix
//   2: shadow map texture view
//   3: shadow map sampler
// 1: Once per model
//   0: model transform matrix
// 2: Once per mesh/material
//   0: mesh diffuse color

// vert shader ---------------------------------------

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

@group(0) @binding(1)
var<uniform> shadow_map_view_proj_matrix: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> model_transform: mat4x4<f32>;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) shadow_map_pos: vec4<f32>,
}

@vertex
fn vert_main(
    @location(0) v_position: vec3<f32>,
    @location(1) v_normal: vec3<f32>,
) -> VertexOutput {
    let pos = vec4<f32>(v_position, 1.0);
    let world_pos = model_transform * pos;

    var out: VertexOutput;
    out.clip_position = camera_matrix * world_pos;
    out.normal = v_normal;
    out.shadow_map_pos = shadow_map_view_proj_matrix * world_pos;
    return out;
}


// frag shader ---------------------------------------

@group(0) @binding(2)
var shadow_map_tex: texture_depth_2d;

@group(0) @binding(3)
var shadow_map_sampler: sampler_comparison;

@group(2) @binding(0)
var<uniform> diffuse_color: vec4<f32>;

const sun_direction = vec3<f32>(1.0, -2.0, 1.0);
const ambient_shade_threshold: f32 = 0.45;
const ambient_light_strength: f32 = 0.4;

const sun_light_strength: f32 = 0.5;

@fragment
fn frag_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let shadow_coords = in.shadow_map_pos.xyz / in.shadow_map_pos.w;
    var shadow_uv: vec2<f32> = shadow_coords.xy * vec2(0.5, -0.5) + vec2(0.5, 0.5); // map from [-1, 1] range to [0, 1] texture coordinate range
    let shadow_depth = shadow_coords.z;

    var sun_light_factor: f32 = 0.0;
    let texel_size = 1.0 / vec2<f32>(2048.0, 2048.0);

    // Take 9 samples in a 3x3 grid
    for (var y: u32 = 0; y < 3; y = y + 1) {
        for (var x: u32 = 0; x < 3; x = x + 1) {
            let offset = vec2<f32>(f32(x) - 1.0, f32(y) - 1.0);
            sun_light_factor += textureSampleCompare(
                shadow_map_tex,
                shadow_map_sampler,
                shadow_uv + offset * texel_size,
                shadow_depth - 0.0001
            );
        }
    }
    sun_light_factor /= 16.0;
    
    var color = diffuse_color.rgb;
    var light: f32 = 0.2;

    let normal = normalize(in.normal);
    let angle_from_sun = acos(dot(normal, normalize(sun_direction)));
    if (angle_from_sun > (3.141592 * ambient_shade_threshold)) {
        // has ambient shade
        light += ambient_light_strength;
    }
    light += sun_light_strength * sun_light_factor;

    return vec4<f32>(color * light, 1.0);
}