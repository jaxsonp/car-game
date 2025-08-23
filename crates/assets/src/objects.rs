use rapier3d::prelude::*;

use crate::*;

pub struct Car {}
impl Car {
    /// Front left wheel position
    //const WHEEL_OFFSET: [f32; 3] = [0.5, -0.05, 0.5];

    /// The convex hulls that make up the hitbox of the car
    const HITBOX_PARTS: ([[f32; 3]; 8], [[f32; 3]; 12]) = (
        [
            [-0.599627, 1.280740, 0.288304],
            [0.599627, 1.280740, 0.288304],
            [0.599627, 1.264557, -0.465441],
            [-0.599627, 1.264557, -0.465441],
            [0.794255, 0.890058, -1.553367],
            [-0.794255, 0.890058, -1.553367],
            [0.772028, 0.890058, 1.012859],
            [-0.772028, 0.890058, 1.012859],
        ],
        [
            [0.809219, 0.208427, -1.970972],
            [0.772028, 0.208427, 1.929723],
            [0.344185, 0.208427, 2.101376],
            [-0.344185, 0.208427, 2.101376],
            [-0.772028, 0.208427, 1.929723],
            [-0.809219, 0.208427, -1.970972],
            [0.809219, 0.911109, -1.970972],
            [0.772028, 0.911109, 1.929723],
            [0.344185, 0.911109, 2.101376],
            [-0.344185, 0.911109, 2.101376],
            [-0.772028, 0.911109, 1.929723],
            [-0.809219, 0.911109, -1.970972],
        ],
    );
}
impl GameObject for Car {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("car.obj");

    const debug_lines: &'static [RawDebugLine] = &debug_lines! {
        // top of toppart
        Self::HITBOX_PARTS.0[0]=>Self::HITBOX_PARTS.0[1]=>Self::HITBOX_PARTS.0[2]=>Self::HITBOX_PARTS.0[3]=>Self::HITBOX_PARTS.0[0];
        // bottom of top part
        Self::HITBOX_PARTS.0[4]=>Self::HITBOX_PARTS.0[5]=>Self::HITBOX_PARTS.0[7]=>Self::HITBOX_PARTS.0[6]=>Self::HITBOX_PARTS.0[4];
        // connecting top and bottom
        Self::HITBOX_PARTS.0[0]=>Self::HITBOX_PARTS.0[7];
        Self::HITBOX_PARTS.0[1]=>Self::HITBOX_PARTS.0[6];
        Self::HITBOX_PARTS.0[2]=>Self::HITBOX_PARTS.0[4];
        Self::HITBOX_PARTS.0[3]=>Self::HITBOX_PARTS.0[5];
        // top of bottom part
        Self::HITBOX_PARTS.1[0]=>Self::HITBOX_PARTS.1[1]=>Self::HITBOX_PARTS.1[2]=>Self::HITBOX_PARTS.1[3]=>Self::HITBOX_PARTS.1[4]=>Self::HITBOX_PARTS.1[5]=>Self::HITBOX_PARTS.1[0];
        // bottom of bottom part
        Self::HITBOX_PARTS.1[6]=>Self::HITBOX_PARTS.1[7]=>Self::HITBOX_PARTS.1[8]=>Self::HITBOX_PARTS.1[9]=>Self::HITBOX_PARTS.1[10]=>Self::HITBOX_PARTS.1[11]=>Self::HITBOX_PARTS.1[6];
        // connecting top and bottom
        Self::HITBOX_PARTS.1[0]=>Self::HITBOX_PARTS.1[6];
        Self::HITBOX_PARTS.1[1]=>Self::HITBOX_PARTS.1[7];
        Self::HITBOX_PARTS.1[2]=>Self::HITBOX_PARTS.1[8];
        Self::HITBOX_PARTS.1[3]=>Self::HITBOX_PARTS.1[9];
        Self::HITBOX_PARTS.1[4]=>Self::HITBOX_PARTS.1[10];
        Self::HITBOX_PARTS.1[5]=>Self::HITBOX_PARTS.1[11];
        Self::HITBOX_PARTS.1[0]=>Self::HITBOX_PARTS.1[6];
    };

    fn get_collision_box() -> ColliderBuilder {
        let top_points: Vec<Point<f32>> = Self::HITBOX_PARTS
            .0
            .iter()
            .map(|p| Point::from(*p))
            .collect();
        let bottom_points: Vec<Point<f32>> = Self::HITBOX_PARTS
            .1
            .iter()
            .map(|p| Point::from(*p))
            .collect();
        ColliderBuilder::compound(vec![
            (
                Isometry::identity(),
                SharedShape::convex_hull(top_points.as_slice())
                    .expect("Failed to make convex hull"),
            ),
            (
                Isometry::identity(),
                SharedShape::convex_hull(bottom_points.as_slice())
                    .expect("Failed to make convex hull"),
            ),
        ])
    }
}

pub struct Floor {}
impl GameObject for Floor {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("floor.obj");

    #[rustfmt::skip]
    const debug_lines: &'static [RawDebugLine] = &[
        // floor plane
        RawDebugLine { col: BLACK, pos1: [20.0, 0.0, 20.0], pos2: [20.0, 0.0, -20.0], },
        RawDebugLine { col: BLACK, pos1: [20.0, 0.0, -20.0], pos2: [-20.0, 0.0, -20.0], },
        RawDebugLine { col: BLACK, pos1: [-20.0, 0.0, -20.0], pos2: [-20.0, 0.0, 20.0], },
        RawDebugLine { col: BLACK, pos1: [-20.0, 0.0, 20.0], pos2: [20.0, 0.0, 20.0], },

        // origin
        RawDebugLine { col: RED, pos1: [0.0, 0.0, 0.0], pos2: [1.0, 0.0, 0.0], },
        RawDebugLine { col: GREEN, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 1.0, 0.0], },
        RawDebugLine { col: BLUE, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, 1.0],  },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [-1.0, 0.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, -1.0, 0.0], },
        RawDebugLine { col: GRAY, pos1: [0.0, 0.0, 0.0], pos2: [0.0, 0.0, -1.0], },
    ];

    fn get_collision_box() -> ColliderBuilder {
        todo!()
    }
}
