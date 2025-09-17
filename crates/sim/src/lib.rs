mod car;
mod controller;
mod physics;

use car_game_assets::GameObject;
use car_game_utils::*;
use nalgebra::{Point3, Vector3};
use noise::{NoiseFn, Perlin};
use rapier3d::prelude::*;

use car::CarHandler;
use controller::CarController;

use crate::physics::PhysicsHandler;

// collision groups
const STATIC_OBJECTS: Group = Group::GROUP_1;
const DYNAMIC_OBJECTS: Group = Group::GROUP_2;

const WATER_LEVEL_BASE: f32 = -1.68847;
const WATER_LEVEL_MOVEMENT: f32 = 1.0;

pub struct GameSimulation {
    t: f64,
    physics_handler: PhysicsHandler,
    car_handler: CarHandler,
    pub controller: CarController,

    water_level_noise: Perlin,
}

impl GameSimulation {
    pub fn new() -> GameSimulation {
        let mut physics_handler = PhysicsHandler::new();

        // static hitboxes
        use car_game_assets::objects;
        for collider in [
            objects::Ground::get_collision_box(),
            objects::Roads::get_collision_box(),
            objects::Buildings::get_collision_box(),
            objects::Streetlights::get_collision_box(),
            objects::Trees1::get_collision_box(),
        ] {
            physics_handler.insert_object(
                RigidBodyBuilder::new(RigidBodyType::Fixed).build(),
                Some(
                    collider
                        .collision_groups(InteractionGroups::new(STATIC_OBJECTS, DYNAMIC_OBJECTS))
                        .build(),
                ),
            );
        }

        let car_handler = CarHandler::new(&mut physics_handler);

        let water_level_noise = Perlin::new(216);

        GameSimulation {
            t: 0.0,
            physics_handler,
            car_handler,
            controller: CarController::new(),
            water_level_noise,
        }
    }

    pub fn step(&mut self, adjusted_dt: f32, controller_activated: bool) -> RenderSnapshot {
        self.t += (adjusted_dt / 60.0) as f64;

        let water_level = WATER_LEVEL_BASE
            + WATER_LEVEL_MOVEMENT * (self.water_level_noise.get([self.t / 7.0]) as f32 - 0.5);

        let (wheel_transforms, skid_contact_points) = self.car_handler.step(
            adjusted_dt,
            &mut self.physics_handler,
            if controller_activated {
                Some(&self.controller)
            } else {
                None
            },
            water_level,
        );

        self.physics_handler.step(adjusted_dt);

        let car_rb = &self.physics_handler.rigid_bodies[self.car_handler.rb_handle];

        let car_transform = *car_rb.position();
        let car_speed = if self.car_handler.wheels_grounded > 0 {
            car_rb.linvel().magnitude()
        } else {
            0.0
        };

        RenderSnapshot {
            car_transform,
            car_speed,
            wheel_transforms,
            skid_contact_points,
            water_level,
        }
    }

    pub fn update_camera(&mut self, adjusted_dt: f32, cam: &mut Camera) {
        const CAM_EYE_LERP: f32 = 0.06;
        const CAM_TARGET_LERP: f32 = 0.3;

        //const CAM_EYE_OFFSET: Vector3<f32> = Vector3::new(0.0, 5.0, -8.0);
        const CAM_EYE_HEIGHT: f32 = 5.0;
        const CAM_EYE_DIST: f32 = 6.25;
        const CAM_TARGET_HEIGHT: f32 = 2.0;

        let car_transform =
            *self.physics_handler.rigid_bodies[self.car_handler.rb_handle].position();
        let car_linear_vel =
            *self.physics_handler.rigid_bodies[self.car_handler.rb_handle].linvel();

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
            .create_query_pipeline(
                QueryFilter::new().exclude_rigid_body(self.car_handler.rb_handle),
            )
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
            self.physics_handler.rigid_bodies[self.car_handler.rb_handle]
                .linvel()
                .magnitude(),
        )
    }
}
