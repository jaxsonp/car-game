use rapier3d::prelude::*;

use crate::*;

pub struct Car {}
impl Car {
    // wheel positions
    const WHEEL_OFFSET_FRONT_DRIVER: [f32; 3] = [0.586441, 0.311319, 1.10437];
    const WHEEL_OFFSET_FRONT_PASSENGER: [f32; 3] = [
        -Self::WHEEL_OFFSET_FRONT_DRIVER[0],
        Self::WHEEL_OFFSET_FRONT_DRIVER[1],
        Self::WHEEL_OFFSET_FRONT_DRIVER[2],
    ];
    const WHEEL_OFFSET_REAR_DRIVER: [f32; 3] = [
        Self::WHEEL_OFFSET_FRONT_DRIVER[0],
        Self::WHEEL_OFFSET_FRONT_DRIVER[1],
        -1.06343,
    ];
    const WHEEL_OFFSET_REAR_PASSENGER: [f32; 3] = [
        -Self::WHEEL_OFFSET_REAR_DRIVER[0],
        Self::WHEEL_OFFSET_REAR_DRIVER[1],
        Self::WHEEL_OFFSET_REAR_DRIVER[2],
    ];
    pub const WHEEL_OFFSETS: [[f32; 3]; 4] = [
        Self::WHEEL_OFFSET_FRONT_DRIVER,
        Self::WHEEL_OFFSET_FRONT_PASSENGER,
        Self::WHEEL_OFFSET_REAR_DRIVER,
        Self::WHEEL_OFFSET_REAR_PASSENGER,
    ];

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
        Self::HITBOX_PARTS.0[0] => Self::HITBOX_PARTS.0[1] => Self::HITBOX_PARTS.0[2] => Self::HITBOX_PARTS.0[3] => Self::HITBOX_PARTS.0[0];
        // bottom of top part
        Self::HITBOX_PARTS.0[4] => Self::HITBOX_PARTS.0[5] => Self::HITBOX_PARTS.0[7] => Self::HITBOX_PARTS.0[6] => Self::HITBOX_PARTS.0[4];
        // connecting top and bottom
        Self::HITBOX_PARTS.0[0] => Self::HITBOX_PARTS.0[7];
        Self::HITBOX_PARTS.0[1] => Self::HITBOX_PARTS.0[6];
        Self::HITBOX_PARTS.0[2] => Self::HITBOX_PARTS.0[4];
        Self::HITBOX_PARTS.0[3] => Self::HITBOX_PARTS.0[5];
        // top of bottom part
        Self::HITBOX_PARTS.1[0] => Self::HITBOX_PARTS.1[1] => Self::HITBOX_PARTS.1[2] => Self::HITBOX_PARTS.1[3] => Self::HITBOX_PARTS.1[4] => Self::HITBOX_PARTS.1[5] => Self::HITBOX_PARTS.1[0];
        // bottom of bottom part
        Self::HITBOX_PARTS.1[6] => Self::HITBOX_PARTS.1[7] => Self::HITBOX_PARTS.1[8] => Self::HITBOX_PARTS.1[9] => Self::HITBOX_PARTS.1[10] => Self::HITBOX_PARTS.1[11] => Self::HITBOX_PARTS.1[6];
        // connecting top and bottom
        Self::HITBOX_PARTS.1[0] => Self::HITBOX_PARTS.1[6];
        Self::HITBOX_PARTS.1[1] => Self::HITBOX_PARTS.1[7];
        Self::HITBOX_PARTS.1[2] => Self::HITBOX_PARTS.1[8];
        Self::HITBOX_PARTS.1[3] => Self::HITBOX_PARTS.1[9];
        Self::HITBOX_PARTS.1[4] => Self::HITBOX_PARTS.1[10];
        Self::HITBOX_PARTS.1[5] => Self::HITBOX_PARTS.1[11];
        Self::HITBOX_PARTS.1[0] => Self::HITBOX_PARTS.1[6];
        // wheel axes
        Self::WHEEL_OFFSET_FRONT_DRIVER =>      {let mut p = Self::WHEEL_OFFSET_FRONT_DRIVER; p[1] -= 0.5; p};
        Self::WHEEL_OFFSET_FRONT_PASSENGER =>   {let mut p = Self::WHEEL_OFFSET_FRONT_PASSENGER; p[1] -= 0.5; p};
        Self::WHEEL_OFFSET_REAR_DRIVER =>       {let mut p = Self::WHEEL_OFFSET_REAR_DRIVER; p[1] -= 0.5; p};
        Self::WHEEL_OFFSET_REAR_PASSENGER =>    {let mut p = Self::WHEEL_OFFSET_REAR_PASSENGER; p[1] -= 0.5; p};

        // marker
        [0.0, 0.0, 0.0] => [0.0, 2.0, 0.0];

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

pub struct Wheel {}
impl GameObject for Wheel {
    const render_meshes: &'static [RawMesh] = load_obj_mesh!("wheel.obj");

    const debug_lines: &'static [RawDebugLine] = &debug_lines!(
        [0.0, 0.0, 0.0] => [0.0, 0.5, 0.0];
    );

    fn get_collision_box() -> ColliderBuilder {
        // wheel collision is handled artificially
        unimplemented!()
    }
}
