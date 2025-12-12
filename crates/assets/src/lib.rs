mod macros;
pub mod objects;

use rapier3d::prelude::ColliderBuilder;

type Color = [f32; 3];
/// Transform matrix
type InstanceDescription = [[f32; 4]; 4];

/*const BLACK: Color = [0.0, 0.0, 0.0];
const GRAY: Color = [0.5, 0.5, 0.5];
const RED: Color = [1.0, 0.0, 0.0];
const GREEN: Color = [0.0, 1.0, 0.0];
const BLUE: Color = [0.0, 0.0, 1.0];*/

/// Describes a game object, provides a default implementation of a collision box which creates it from the render mesh
#[allow(non_upper_case_globals)]
pub trait GameObject {
    const render_meshes: &'static [RawMesh];
    #[rustfmt::skip]
    const instances: &'static [InstanceDescription] =
        &[[
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]];
    const debug_lines: &'static [RawDebugLine] = &[];
    fn get_collision_box() -> ColliderBuilder {
        unimplemented!();
    }
}

#[derive(Clone, Copy)]
/// Represents the raw data for a mesh
pub struct RawMesh {
    pub material: RawMaterial,
    pub verts: &'static [RawVertex],
    pub indices: &'static [u32],
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
