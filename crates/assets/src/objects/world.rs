use nalgebra::{DMatrix, Isometry3, Point3, Vector3};
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

pub struct Buildings {}
impl Buildings {}
impl GameObject for Buildings {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("buildings.obj");

    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("buildings_hitbox.obj")[0];
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
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("streetlights.obj");
    fn get_collision_box() -> ColliderBuilder {
        let hitbox_mesh: RawMesh = load_obj_mesh!("streetlights_hitbox.obj")[0];
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

pub struct Trees1 {}
impl GameObject for Trees1 {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("tree1.obj");

    fn get_collision_box() -> ColliderBuilder {
        ColliderBuilder::compound(
            Self::instances
                .unwrap()
                .into_iter()
                .map(|pos| {
                    (
                        Isometry3::translation(pos.x, pos.y, pos.z),
                        SharedShape::cylinder(3.0, 0.23),
                    )
                })
                .collect(),
        )
    }

    const instances: Option<&'static [Point3<f32>]> = Some(&[
        Point3::new(-168.0408172607422, 3.084705114364624, 2.0454177856445312),
        Point3::new(-112.27576446533203, 3.084705114364624, 196.3356475830078),
        Point3::new(-18.013534545898438, 3.084705114364624, -4.992973327636719),
        Point3::new(-27.114788055419922, 3.084705114364624, -4.6475830078125),
        Point3::new(-108.63613891601562, 3.084705114364624, 98.21630859375),
        Point3::new(-108.63613891601562, 3.084705114364624, 77.91064453125),
        Point3::new(-108.63613891601562, 3.084705114364624, 107.40689086914062),
        Point3::new(-108.63613891601562, 3.084705114364624, 136.14610290527344),
        Point3::new(5.148284912109375, 3.084705114364624, 107.90838623046875),
        Point3::new(5.148284912109375, 3.084705114364624, 148.2598114013672),
        Point3::new(5.148284912109375, 3.084705114364624, 127.95414733886719),
        Point3::new(-108.5023422241211, 3.084705114364624, 45.505714416503906),
        Point3::new(-108.5023422241211, 3.084705114364624, 25.416824340820312),
        Point3::new(-137.37991333007812, 3.084705114364624, -95.98348236083984),
        Point3::new(-143.5357208251953, 3.084705114364624, -75.4641342163086),
        Point3::new(-119.83587646484375, 3.084705114364624, -80.49138641357422),
        Point3::new(-108.5023422241211, 3.084705114364624, 16.226242065429688),
        Point3::new(-108.5023422241211, 3.084705114364624, 36.31513214111328),
        Point3::new(-108.5023422241211, 3.084705114364624, 56.300514221191406),
        Point3::new(-96.49781036376953, 3.084705114364624, 196.50071716308594),
        Point3::new(-108.63613891601562, 3.084705114364624, 87.10122680664062),
        Point3::new(-108.63613891601562, 3.084705114364624, 117.99885559082031),
        Point3::new(-108.63613891601562, 3.084705114364624, 127.18943786621094),
        Point3::new(-87.69541931152344, 3.084705114364624, 196.43365478515625),
        Point3::new(-64.58572387695312, 3.084705114364624, 196.80908203125),
        Point3::new(-104.74945068359375, 3.084705114364624, 196.41036987304688),
        Point3::new(-79.44682312011719, 3.084705114364624, 196.66156005859375),
        Point3::new(-71.92049407958984, 3.084705114364624, 196.7362823486328),
        Point3::new(-49.162052154541016, 3.084705114364624, 196.50071716308594),
        Point3::new(-41.42338562011719, 3.084705114364624, 196.61094665527344),
        Point3::new(-19.377410888671875, 3.084705114364624, 196.80908203125),
        Point3::new(-34.06122589111328, 3.084705114364624, 196.66156005859375),
        Point3::new(-26.71218490600586, 3.084705114364624, 196.7362823486328),
        Point3::new(-23.483901977539062, 3.084705114364624, -13.782264709472656),
        Point3::new(-30.804275512695312, 3.084705114364624, -19.201133728027344),
        Point3::new(-46.52943801879883, 3.084705114364624, -17.843772888183594),
        Point3::new(-42.62527847290039, 3.084705114364624, -26.072357177734375),
        Point3::new(-148.78001403808594, 3.084705114364624, -29.524314880371094),
        Point3::new(-145.68362426757812, 3.084705114364624, -21.199745178222656),
        Point3::new(-159.65809631347656, 3.084705114364624, -31.0931396484375),
        Point3::new(-164.78744506835938, 3.084705114364624, -20.677223205566406),
        Point3::new(-174.38462829589844, 3.084705114364624, -8.575782775878906),
        Point3::new(-175.51593017578125, 3.084705114364624, -14.784629821777344),
    ]);
}
