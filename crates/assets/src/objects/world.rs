use nalgebra::{Isometry3, Point3, Translation3};
use rapier3d::prelude::*;

use crate::*;

pub struct Ground {}
impl GameObject for Ground {
    const render_meshes: &'static [RawMesh] = preloaded_file!("ground.obj");

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
        let hitbox_mesh: RawMesh = preloaded_file!("ground_hitbox.obj")[0];
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
    const render_meshes: &'static [RawMesh] = preloaded_file!("roads.obj");
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

pub struct Buildings {}
impl Buildings {}
impl GameObject for Buildings {
    const render_meshes: &'static [RawMesh] = preloaded_file!("buildings.obj");

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = preloaded_file!("buildings_hitbox.obj")[0];
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

pub struct Streetlights {}
impl GameObject for Streetlights {
    const render_meshes: &'static [RawMesh] = preloaded_file!("streetlight.obj");

    const instances: &'static [[[f32; 4]; 4]] = preloaded_file!("streetlight_instances.csv");
    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::compound(
            Self::instances
                .into_iter()
                .map(|transform| {
                    (
                        Isometry3::from(Translation3::new(
                            transform[3][0],
                            transform[3][1],
                            transform[3][2],
                        )),
                        SharedShape::cylinder(3.0, 0.23),
                    )
                })
                .collect(),
        )
    }
}

pub struct Trees1 {}
impl GameObject for Trees1 {
    const render_meshes: &'static [RawMesh] = preloaded_file!("tree1.obj");

    const instances: &'static [[[f32; 4]; 4]] = preloaded_file!("tree1_instances.csv");

    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::compound(
            Self::instances
                .into_iter()
                .map(|transform| {
                    (
                        Isometry3::from(Translation3::new(
                            transform[3][0],
                            transform[3][1],
                            transform[3][2],
                        )),
                        SharedShape::cylinder(3.0, 0.23),
                    )
                })
                .collect(),
        )
    }
}

pub struct GrassTufts1 {}
impl GameObject for GrassTufts1 {
    const render_meshes: &'static [RawMesh] = preloaded_file!("grasstuft1.obj");
    const instances: &'static [[[f32; 4]; 4]] = preloaded_file!("grasstuft1_instances.csv");
}

pub struct GrassTufts2 {}
impl GameObject for GrassTufts2 {
    const render_meshes: &'static [RawMesh] = preloaded_file!("grasstuft2.obj");
    const instances: &'static [[[f32; 4]; 4]] = preloaded_file!("grasstuft2_instances.csv");
}
