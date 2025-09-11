mod car;
mod controller;
mod physics;

use assets::GameObject;
use nalgebra::{Point3, Vector3};
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

        // ground
        physics_handler.insert_object(
            RigidBodyBuilder::new(RigidBodyType::Fixed).build(),
            Some(assets::objects::Ground::get_collision_box().build()),
        );
        // roads
        physics_handler.insert_object(
            RigidBodyBuilder::new(RigidBodyType::Fixed).build(),
            Some(assets::objects::Roads::get_collision_box().build()),
        );
        // decor
        physics_handler.insert_object(
            RigidBodyBuilder::new(RigidBodyType::Fixed).build(),
            Some(assets::objects::WorldDecor::get_collision_box().build()),
        );

        let car_handler = CarHandler::new(&mut physics_handler);

        GameSimulation {
            physics_handler,
            car_handler,
            controller: CarController::new(),
        }
    }

    pub fn step(&mut self, adjusted_dt: f32, controller_activated: bool) -> RenderSnapshot {
        self.physics_handler.step(adjusted_dt);

        let (wheel_transforms, skid_contact_points) = self.car_handler.step(
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
            skid_contact_points,
        }
    }

    pub fn update_camera(&mut self, adjusted_dt: f32, cam: &mut Camera) {
        const CAM_EYE_LERP: f32 = 0.06;
        const CAM_TARGET_LERP: f32 = 0.3;

        //const CAM_EYE_OFFSET: Vector3<f32> = Vector3::new(0.0, 5.0, -8.0);
        const CAM_EYE_HEIGHT: f32 = 5.0;
        const CAM_EYE_DIST: f32 = 6.25;
        const CAM_TARGET_HEIGHT: f32 = 2.0;

        let car_transform = *self.physics_handler.rigid_bodies[self.car_handler.handle].position();
        let car_linear_vel = *self.physics_handler.rigid_bodies[self.car_handler.handle].linvel();

        let forward_dir: Vector3<f32> = {
            let mut car_forward = car_transform.rotation.transform_vector(&Vector3::z());
            car_forward.y = 0.0;
            let mut linvel_forward = car_linear_vel;
            linvel_forward.y = 0.0;

            // use linear velocity as forward direction if not grounded
            if self.car_handler.wheels_grounded > 1 || linvel_forward.magnitude() < 0.5 {
                car_forward
            } else {
                linvel_forward
            }
            .normalize()
        };
        let mut target_eye: Point3<f32> =
            car_transform.translation * Point3::new(0.0, CAM_EYE_HEIGHT, 0.0);
        // casting ray backwards
        let dist = if let Some((_, dist)) = self
            .physics_handler
            .create_query_pipeline(QueryFilter::new().exclude_rigid_body(self.car_handler.handle))
            .cast_ray(&Ray::new(target_eye, -forward_dir), CAM_EYE_DIST, true)
        {
            dist
        } else {
            CAM_EYE_DIST
        };
        target_eye -= forward_dir * dist;

        cam.eye = cam.eye.lerp(&target_eye, CAM_EYE_LERP * adjusted_dt);

        let target_target: Point3<f32> =
            (car_transform.translation.vector + Vector3::new(0.0, CAM_TARGET_HEIGHT, 0.0)).into();
        cam.target = cam
            .target
            .lerp(&target_target, CAM_TARGET_LERP * adjusted_dt);

        cam.up = cam.up.lerp(&Vector3::y(), CAM_TARGET_LERP * adjusted_dt);
    }

    pub fn get_debug_string(&self) -> String {
        format!(
            "throttle input: {:?}\nsteer input: {:?}\nthrottle: {:.2}\nsteer: {:.2}\nspeed: {:.2}\n",
            self.car_handler.drive_input,
            self.car_handler.turn_input,
            self.car_handler.throttle,
            self.car_handler.turn_angle,
            self.physics_handler.rigid_bodies[self.car_handler.handle]
                .linvel()
                .magnitude(),
        )
    }
}
