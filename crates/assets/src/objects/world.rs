use nalgebra::{DMatrix, Point3, Vector3};
use rapier3d::prelude::*;

use crate::*;

pub struct Ocean {}
impl Ocean {
    const HITBOX_SIZE: f32 = 1000.0;
    const WATER_HEIGHT: f32 = -2.96968;
}
impl GameObject for Ocean {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("ocean.obj");

    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::heightfield(
            DMatrix::from_element(2, 2, Self::WATER_HEIGHT),
            Vector3::new(Self::HITBOX_SIZE, 1.0, Self::HITBOX_SIZE),
        )
    }
}

pub struct Ground {}
impl GameObject for Ground {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("ground.obj");

    #[rustfmt::skip]
    const debug_lines: &'static [RawDebugLine] = &[
        // origin
        RawDebugLine { col: RED, pos1: [0.0, 0.0, 0.0], pos2: [1.0, 0.0, 0.0], },
        RawDebugLine { col: GREEN, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 1.0, 0.0], },
        RawDebugLine { col: BLUE, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, 1.0],  },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [-1.0, 0.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, -1.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, -1.0], },
    ];

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("ground_hitbox.obj")[0];
        let mut verts: Vec<Point3<f32>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for v in hitbox_mesh.verts {
            verts.push(Point3::from(v.pos));
        }

        let mut count = 0;
        let mut cur_face: [u32; 3] = [0; 3];
        for index in hitbox_mesh.indices {
            cur_face[count] = *index as u32;
            count += 1;
            if count == 3 {
                indices.push(cur_face);
                count = 0;
            }
        }
        ColliderBuilder::trimesh(verts, indices).expect("Failed to create trimesh collision box")
    }
}

pub struct Roads {}
impl GameObject for Roads {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("roads.obj");
}

pub struct WorldDecor {}
impl GameObject for WorldDecor {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("decor.obj");

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("decor_hitbox.obj")[0];
        let mut verts: Vec<Point3<f32>> = Vec::new();
        let mut indices: Vec<[u32; 3]> = Vec::new();

        for v in hitbox_mesh.verts {
            verts.push(Point3::from(v.pos));
        }

        let mut count = 0;
        let mut cur_face: [u32; 3] = [0; 3];
        for index in hitbox_mesh.indices {
            cur_face[count] = *index as u32;
            count += 1;
            if count == 3 {
                indices.push(cur_face);
                count = 0;
            }
        }
        ColliderBuilder::trimesh(verts, indices).expect("Failed to create trimesh collision box")
    }
}
