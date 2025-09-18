// Shader for rendering the ocean
// Bind groups: (many unused)
// 0: Once per scene render
//   0: camera matrix
//   1: sun direction vector
//   2: shadow map view proj matrix
//   3: shadow map texture view
//   4: shadow map sampler
// 1: Ocean specific stuff
//   0: water level + screen size
//   1: depth texture view
//   2: depth texture sampler

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
}

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> water_level_and_size: vec4<f32>;

@group(1) @binding(1)
var depth_tex: texture_depth_2d;

@group(1) @binding(2)
var depth_tex_sampler: sampler_comparison;

// size can't be too big or water level gets messed up cus fp error
const SIZE: f32 = 10000.0;

@vertex
fn vert_main(
	@builtin(vertex_index) index: u32
) -> VertexOutput {
	let i = f32(index);
	let water_level = water_level_and_size.x;

	// programatically generating a plane for the ocean 
	let x = (floor(i / 3.0) - 1.0) * SIZE;
	let z = (i - (floor(i / 3.0) * 3.0) - 1.0) * SIZE;
	let world_pos = vec4<f32>(x, water_level, z, 1.0);

	var out: VertexOutput;
	out.clip_pos = camera_matrix * world_pos;
	return out;
}


// frag shader -----------------------------------------------------------
// because i can't sample depth textures normally (only comparisons), interpolate
// between the deep and shallow color in `n_steps` steps.

const shallow_color: vec3<f32> = vec3(0.026240, 0.577581, 0.577581);
const deep_color: vec3<f32> = vec3(0.01, 0.4, 0.78);
const foam_color: vec3<f32> = vec3(0.93, 0.95, 1.0);

const n_steps: u32 = 4;
const step_depth: f32 = 0.75;
const foam_depth: f32 = 0.12;

const color_step_factor: f32 = 1.0 / f32(n_steps);

@fragment
fn frag_main(
	in: VertexOutput,
) -> @location(0) vec4<f32> {
	let w = water_level_and_size.y;
	let h = water_level_and_size.z;

	let screen_uv = vec2<f32>(in.clip_pos.x / w, in.clip_pos.y / h);

	// emulate depth stencil
	if (textureSampleCompareLevel(depth_tex, depth_tex_sampler, screen_uv, in.clip_pos.z) != 0.0) {
		return vec4<f32>(0.0);
	}
	
	let world_dist = z_dist_to_world_dist(in.clip_pos.z);

	var color: vec3<f32> = vec3(0.0);
	if (textureSampleCompareLevel(
		depth_tex,
		depth_tex_sampler,
		screen_uv,
		world_dist_to_z_dist(world_dist + foam_depth)
	) == 1.0) {
		// foam
		color = foam_color;
	} else {
		// not foam
		for (var test_depth: f32 = foam_depth; test_depth < (f32(n_steps) * step_depth + foam_depth); test_depth += step_depth) {
			let sample = textureSampleCompareLevel(
				depth_tex,
				depth_tex_sampler,
				screen_uv,
				world_dist_to_z_dist(world_dist + test_depth)
			);
			color += (shallow_color * sample + deep_color * (1.0 - sample)) * color_step_factor;
		}
	}
	
	return vec4<f32>(color, 1.0);

}

 
// z clip values
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 500.0;

fn z_dist_to_world_dist(d: f32) -> f32 {
	// d is now in [0, 1] range instead of [-1, 1]
	return (Z_NEAR * Z_FAR) / (Z_FAR - d * (Z_FAR - Z_NEAR));
}

fn world_dist_to_z_dist(d: f32) -> f32 {
	return (Z_FAR - Z_NEAR * Z_FAR / d) / (Z_FAR - Z_NEAR);
}