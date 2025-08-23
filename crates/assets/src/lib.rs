pub mod fonts;
mod macros;
pub mod objects;

use rapier3d::prelude::ColliderBuilder;

type Color = [f32; 3];

const BLACK: Color = [0.0, 0.0, 0.0];
const GRAY: Color = [0.5, 0.5, 0.5];
const RED: Color = [1.0, 0.0, 0.0];
const GREEN: Color = [0.0, 1.0, 0.0];
const BLUE: Color = [0.0, 0.0, 1.0];

#[allow(non_upper_case_globals)]
pub trait GameObject {
    const render_meshes: &'static [RawMesh];
    const debug_lines: &'static [RawDebugLine];
    fn get_collision_box() -> ColliderBuilder;
}

#[derive(Clone, Copy)]
/// Represents the raw data for a mesh
pub struct RawMesh {
    pub material: RawMaterial,
    pub verts: &'static [RawVertex],
    pub indices: &'static [u16],
}

#[derive(Clone, Copy)]
pub struct RawVertex {
    pub pos: [f32; 3],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy)]
pub struct RawDebugLine {
    pub col: Color,
    pub pos1: [f32; 3],
    pub pos2: [f32; 3],
}

#[derive(Clone, Copy)]
pub struct RawMaterial {
    /// RGB
    pub color: Color,
}
