mod car;
mod controller;
mod physics;

use assets::GameObject;
use nalgebra::{Point3, UnitQuaternion, Vector3};
use rapier3d::prelude::*;
use utils::*;

use car::CarHandler;
use controller::CarController;

use crate::physics::PhysicsHandler;

pub struct GameSimulation {
    physics_handler: PhysicsHandler,
    car_handler: CarHandler,
    pub controller: CarController,
}

impl GameSimulation {
    pub fn new() -> GameSimulation {
        let mut physics_handler = PhysicsHandler::new();

        let floor_rbody = RigidBodyBuilder::new(RigidBodyType::Fixed).build();
        let floor_collider = assets::objects::Ground::get_collision_box().build();
        physics_handler.insert_object(floor_rbody, Some(floor_collider));

        let car_handler = CarHandler::new(&mut physics_handler);

        GameSimulation {
            physics_handler,
            car_handler,
            controller: CarController::new(),
        }
    }

    pub fn step(&mut self, adjusted_dt: f32, controller_activated: bool) -> RenderSnapshot {
        self.physics_handler.step(adjusted_dt);

        let wheel_transforms = self.car_handler.step(
            adjusted_dt,
            &mut self.physics_handler,
            if controller_activated {
                Some(&self.controller)
            } else {
                None
            },
        );

        RenderSnapshot {
            car_transform: *self.physics_handler.rigid_bodies[self.car_handler.handle].position(),
            wheel_transforms,
            debug_string: Some("test".to_owned()),
        }
    }

    pub fn update_camera(&self, adjusted_dt: f32, cam: &mut Camera) {
        const CAM_EYE_LERP: f32 = 0.12;
        const CAM_TARGET_LERP: f32 = 0.3;

        const CAM_EYE_OFFSET: Vector3<f32> = Vector3::new(0.0, 5.0, -8.0);
        const CAM_TARGET_OFFSET: Vector3<f32> = Vector3::new(0.0, 2.0, 0.0);

        let car_rb = &self.physics_handler.rigid_bodies[self.car_handler.handle];
        let car_transform = car_rb.position();

        let forward_dir: Vector3<f32> = {
            // NOTE using car direction or velocity direction as forward? idk yet
            let mut v = car_transform.rotation.transform_vector(&Vector3::z());
            //let mut v: Vector3<f32> = *car_rb.linvel();
            v.y = 0.0;
            v.normalize()
        };
        let target_eye_offset: Point3<f32> =
            UnitQuaternion::face_towards(&forward_dir, &Vector3::y())
                .transform_point(&CAM_EYE_OFFSET.into());
        let target_eye: Point3<f32> = car_transform
            .translation
            .transform_point(&target_eye_offset);
        cam.eye = cam.eye.lerp(&target_eye, CAM_EYE_LERP * adjusted_dt);

        let target_target: Point3<f32> =
            (car_transform.translation.vector + CAM_TARGET_OFFSET).into();
        cam.target = cam
            .target
            .lerp(&target_target, CAM_TARGET_LERP * adjusted_dt);

        cam.up = cam.up.lerp(&Vector3::y(), CAM_TARGET_LERP * adjusted_dt);
    }
}
