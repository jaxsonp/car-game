// Shader for rendering the ocean
// Bind groups: (many unused)
// 0: Once per scene render
//   0: camera matrix
//   1: sun direction vector
//   2: shadow map view proj matrix
//   3: shadow map texture view
//   4: shadow map sampler
// 1: Ocean specific stuff
//   0: water level

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
	@location(0) world_pos: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> camera_matrix: mat4x4<f32>;

@group(1) @binding(0)
var<uniform> water_level: vec4<f32>;

// size can't be too big or water level gets messed up cus fp error
const SIZE: f32 = 10000.0;

@vertex
fn vert_main(
    @builtin(vertex_index) index: u32
) -> VertexOutput {
	let i = f32(index);
	let x = (floor(i / 3.0) - 1.0) * SIZE;
	let z = (i - (floor(i / 3.0) * 3.0) - 1.0) * SIZE;
    let world_pos = vec4<f32>(x, water_level.x, z, 1.0);

    var out: VertexOutput;
    out.clip_pos = camera_matrix * world_pos;
	out.world_pos = world_pos.xyz;
    return out;
}

const SHALLOW_COLOR: vec3<f32> = vec3(0.026240, 0.577581, 0.577581);
const DEEP_COLOR: vec3<f32> = vec3(37.0/255.0, 151.0/255.0, 208.0/255.0);
const CLOSE_CUTOFF: f32 = 400.0;
const FAR_CUTOFF: f32 = 1000.0;

@fragment
fn frag_main(
	in: VertexOutput,
) -> @location(0) vec4<f32> {

	let dist: f32 = sqrt(in.world_pos.x * in.world_pos.x + in.world_pos.z * in.world_pos.z);
	if (dist < CLOSE_CUTOFF) {
		return vec4<f32>(SHALLOW_COLOR, 1.0);
	} else if (dist < FAR_CUTOFF) {
		let fac: f32 = (dist - CLOSE_CUTOFF) / (FAR_CUTOFF - CLOSE_CUTOFF);
		return vec4<f32>(DEEP_COLOR * fac + SHALLOW_COLOR * (1.0-fac), 1.0);
	} else {
		return vec4<f32>(DEEP_COLOR, 1.0);
	}
}