mod macros;
pub mod objects;

use nalgebra::Point3;
use rapier3d::prelude::ColliderBuilder;

type Color = [f32; 3];

const BLACK: Color = [0.0, 0.0, 0.0];
const GRAY: Color = [0.5, 0.5, 0.5];
const RED: Color = [1.0, 0.0, 0.0];
const GREEN: Color = [0.0, 1.0, 0.0];
const BLUE: Color = [0.0, 0.0, 1.0];

/// Describes a game object, provides a default implementation of a collision box which creates it from the render mesh
#[allow(non_upper_case_globals)]
pub trait GameObject {
    const render_meshes: &'static [RawMesh];
    const debug_lines: &'static [RawDebugLine] = &[];
    fn get_collision_box() -> ColliderBuilder {
        let mut verts: Vec<Point3<f32>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for mesh in Self::render_meshes {
            for v in mesh.verts {
                verts.push(Point3::from(v.pos));
            }
            let mut count = 0;
            let mut cur_face: [u32; 3] = [0; 3];
            for index in mesh.indices {
                cur_face[count] = *index as u32;
                count += 1;
                if count == 3 {
                    indices.push(cur_face);
                    count = 0;
                }
            }
        }
        ColliderBuilder::trimesh(verts, indices).expect("Failed to create trimesh collision box")
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
