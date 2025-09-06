use assets::{GameObject, objects::Car};
use nalgebra::{Isometry3, Point3, Rotation3, UnitQuaternion, Vector3};
use rapier3d::prelude::*;

use crate::{controller::CarController, physics::PhysicsHandler};

pub struct CarHandler {
    pub handle: RigidBodyHandle,
    turn_angle: f32,
    pub wheels_grounded: u32,
}
impl CarHandler {
    pub fn new(physics: &mut PhysicsHandler) -> CarHandler {
        let rbody = RigidBodyBuilder::dynamic()
            .additional_mass(Car::MASS)
            .linvel(Vector3::new(6.0, 3.0, 0.0))
            //.angvel(Vector3::new(0.0, -0.8, 0.1))
            .position(Isometry3::from_parts(
                Point3::new(0.0, 7.0, 0.0).into(),
                Rotation3::identity().into(),
            ))
            .can_sleep(false) // car doesn't sleep
            .build();
        let collider = Car::get_collision_box().build();
        let (handle, _) = physics.insert_object(rbody, Some(collider));

        CarHandler {
            handle,
            turn_angle: 0.0,
            wheels_grounded: 0,
        }
    }

    pub fn step(
        &mut self,
        adjusted_dt: f32,
        physics: &mut PhysicsHandler,
        controller: Option<&CarController>,
    ) -> [Isometry3<f32>; 4] {
        use assets::objects::Car;

        let car_transform = *physics.rigid_bodies[self.handle].position();
        let car_up_direction = (car_transform.rotation * Vector3::y()).normalize();

        // cast rays to see if tires are touching the ground
        self.wheels_grounded = 0;
        let hits = {
            let query_pipeline =
                physics.create_query_pipeline(QueryFilter::new().exclude_rigid_body(self.handle));
            Car::WHEEL_OFFSETS.map(|wheel_offset| {
                let ray_origin = car_transform * Point3::from(wheel_offset);
                let ray = Ray::new(ray_origin, -car_up_direction);
                if let Some((_collider, hit_dist)) = query_pipeline.cast_ray_and_get_normal(
                    &ray,
                    Car::SUSPENSION_MAX + Car::WHEEL_RADIUS,
                    false,
                ) {
                    self.wheels_grounded += 1;
                    (ray, Some(hit_dist))
                } else {
                    (ray, None)
                }
            })
        };

        let car_rb = &mut physics.rigid_bodies[self.handle];

        // turning wheels
        let (max_turn_radius, turn_speed) = if car_rb.linvel().magnitude() * adjusted_dt > 18.0 {
            (Car::TURN_RADIUS_FAST, Car::TURN_SPEED_FAST * adjusted_dt)
        } else {
            (Car::TURN_RADIUS_SLOW, Car::TURN_SPEED_SLOW * adjusted_dt)
        };
        let mut turn_inputted = false;
        if let Some(controller) = controller {
            if controller.a_pressed && !controller.d_pressed {
                // left
                self.turn_angle =
                    self.turn_angle * (1.0 - turn_speed) + (max_turn_radius * turn_speed);
                turn_inputted = true;
            } else if !controller.a_pressed && controller.d_pressed {
                // right
                self.turn_angle =
                    self.turn_angle * (1.0 - turn_speed) + (-max_turn_radius * turn_speed);
                turn_inputted = true;
            }
        }
        if !turn_inputted {
            // returning to center
            self.turn_angle = self.turn_angle * (1.0 - (turn_speed * 1.5));
        }

        let forward_dir = car_rb.position().rotation.transform_vector(&Vector3::z());
        let turned_forward_dir = car_rb.position().rotation.transform_vector(
            &UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.turn_angle)
                .transform_vector(&Vector3::z()),
        );
        let up_dir = car_rb.position().rotation.transform_vector(&Vector3::y());

        // calculate and apply forces from wheels
        let mut wheel_i: i32 = -1;
        let wheel_positions = hits.map(|(ray, maybe_hit)| {
            wheel_i += 1;
            if let Some(intersection) = maybe_hit {
                // tire is on the ground

                let hit_dist = intersection.time_of_impact;
                let contact_point = ray.point_at(hit_dist - Car::WHEEL_RADIUS);

                // suspension force
                let mut compression = ((Car::SUSPENSION_MAX + Car::WHEEL_RADIUS) - hit_dist)
                    / (Car::SUSPENSION_MAX + Car::WHEEL_RADIUS);
                compression = Car::suspension_compression_curve(compression);
                let spring_impulse = compression * Car::SUSPENSION_STIFFNESS;

                let spring_velocity = car_rb.velocity_at_point(&ray.origin).dot(&ray.dir);
                let damper_impulse = spring_velocity * Car::SUSPENSION_DAMPER;

                let suspension_impulse = car_up_direction * (spring_impulse + damper_impulse);
                car_rb.apply_impulse_at_point(suspension_impulse * adjusted_dt, ray.origin, false);

                // tire direction
                let forward_dir = if wheel_i < 2 && self.turn_angle.abs() > 0.01 {
                    turned_forward_dir
                } else {
                    forward_dir
                };
                let right_dir: Vector3<f32> = forward_dir.cross(&up_dir);

                // grip force
                let tire_velocity: Vector3<f32> = car_rb.velocity_at_point(&contact_point);
                let lateral_velocity: Vector3<f32> = right_dir.scale(tire_velocity.dot(&right_dir));
                let grip_impulse = -lateral_velocity * Car::WHEEL_GRIP;
                car_rb.apply_impulse_at_point(grip_impulse * adjusted_dt, contact_point, false);

                // drive force
                if let Some(controller) = controller {
                    if controller.w_pressed {
                        car_rb.apply_impulse_at_point(
                            forward_dir.scale(Car::ACCELERATION * adjusted_dt),
                            contact_point,
                            false,
                        );
                    } else if controller.s_pressed {
                        car_rb.apply_impulse_at_point(
                            forward_dir.scale(-Car::ACCELERATION * 0.9 * adjusted_dt),
                            contact_point,
                            false,
                        );
                    }
                }

                return contact_point;
            } else {
                // tire is not on the ground
                return ray.point_at(Car::SUSPENSION_MAX);
            }
        });

        // apply downforce if car is grounded and moving fast
        if self.wheels_grounded > 0 {
            let downforce = car_rb.linvel().magnitude() * Car::DOWNFORCE_COEFFICIENT * adjusted_dt;
            car_rb.apply_impulse(-up_dir.scale(downforce), false);
        }

        let mut wheel_transforms =
            wheel_positions.map(|pos| Isometry3::from_parts(pos.into(), *car_rb.rotation()));

        // turning front wheels
        wheel_transforms[0].append_rotation_wrt_center_mut(&UnitQuaternion::from_axis_angle(
            &Vector3::y_axis(),
            self.turn_angle,
        ));
        wheel_transforms[1].append_rotation_wrt_center_mut(&UnitQuaternion::from_axis_angle(
            &Vector3::y_axis(),
            self.turn_angle,
        ));

        wheel_transforms
    }
}
