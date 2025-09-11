use assets::{GameObject, objects::Car};
use nalgebra::{Isometry3, Point3, Rotation3, UnitQuaternion, Vector2, Vector3};
use rapier3d::prelude::*;

use crate::{controller::CarController, physics::PhysicsHandler};

const MASS: f32 = 2400.0;

const ACCELERATION: f32 = 120.0;

const SLOW_FAST_THRESH: f32 = 22.0;

const TURN_RADIUS_SLOW: f32 = 17f32.to_radians();
const TURN_RADIUS_FAST: f32 = 11f32.to_radians();

const TURN_RESPONSIVENESS_SLOW: f32 = 2.5f32.to_radians();
const TURN_RESPONSIVENESS_FAST: f32 = 1.2f32.to_radians();

const THROTTLE_RESPONSIVENESS: f32 = 0.1;

/// max extension of the suspension
const SUSPENSION_MAX: f32 = 0.3;
const SUSPENSION_STIFFNESS: f32 = 1800.0;
const SUSPENSION_DAMPER: f32 = 50.0;

fn suspension_compression_curve(val: f32) -> f32 {
    // nonlinear spring force
    val.powf(2.5)
}

const DRAG_COEFFICIENT: f32 = 0.004;
const DOWNFORCE_COEFFICIENT: f32 = 17.0;

const MAX_FRICTION: f32 = 180.0;

const WHEEL_DIAMETER: f32 = 0.636653;
const WHEEL_RADIUS: f32 = WHEEL_DIAMETER / 2.0;
/// Tire grip coefficient
const WHEEL_GRIP: f32 = 800.0;

pub struct CarHandler {
    pub handle: RigidBodyHandle,
    pub(super) throttle: f32,
    pub(super) turn_angle: f32,

    wheels_slipping: [bool; 4],

    pub wheels_grounded: u32,
    pub drive_input: DriveInputState,
    pub turn_input: TurnInputState,
}
impl CarHandler {
    pub fn new(physics: &mut PhysicsHandler) -> CarHandler {
        let rbody = RigidBodyBuilder::dynamic()
            .additional_mass(MASS)
            .position(Isometry3::from_parts(
                Point3::new(0.0, 5.0, 8.0).into(),
                Rotation3::identity().into(),
            ))
            .can_sleep(false) // car doesn't sleep
            .build();
        let collider = Car::get_collision_box().build();
        let (handle, _) = physics.insert_object(rbody, Some(collider));

        CarHandler {
            handle,
            turn_angle: 0.0,
            throttle: 0.0,
            wheels_slipping: [false; 4],
            wheels_grounded: 0,
            drive_input: DriveInputState::Coasting,
            turn_input: TurnInputState::None,
        }
    }

    pub fn step(
        &mut self,
        adjusted_dt: f32,
        physics: &mut PhysicsHandler,
        controller: Option<&CarController>,
    ) -> ([Isometry3<f32>; 4], [Option<Point3<f32>>; 4]) {
        use assets::objects::Car;

        let car_transform = *physics.rigid_bodies[self.handle].position();
        let car_up_dir: Vector3<f32> = (car_transform.rotation * Vector3::y()).normalize();
        let car_forward_dir: Vector3<f32> = (car_transform.rotation * Vector3::z()).normalize();

        // cast rays to see if tires are touching the ground
        self.wheels_grounded = 0;
        let hits = {
            let query_pipeline =
                physics.create_query_pipeline(QueryFilter::new().exclude_rigid_body(self.handle));
            Car::WHEEL_OFFSETS.map(|wheel_offset| {
                let ray_origin = car_transform * Point3::from(wheel_offset);
                let ray = Ray::new(ray_origin, -car_up_dir);
                if let Some((_collider, hit_dist)) = query_pipeline.cast_ray_and_get_normal(
                    &ray,
                    SUSPENSION_MAX + WHEEL_RADIUS,
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
        let car_linvel = *car_rb.linvel();

        // parsing player input
        self.drive_input = if let Some(controller) = controller {
            if controller.shift_pressed || (controller.w_pressed && controller.s_pressed) {
                DriveInputState::HardBraking
            } else if controller.w_pressed {
                DriveInputState::Accelerating
            } else if controller.s_pressed {
                DriveInputState::Reversing
            } else {
                DriveInputState::Coasting
            }
        } else {
            DriveInputState::Coasting
        };
        self.turn_input = if let Some(controller) = controller {
            if controller.a_pressed && !controller.d_pressed {
                TurnInputState::Left
            } else if !controller.a_pressed && controller.d_pressed {
                TurnInputState::Right
            } else {
                TurnInputState::None
            }
        } else {
            TurnInputState::None
        };

        // lerp turn angle
        let (max_turn_radius, turn_response) =
            if car_linvel.magnitude() * adjusted_dt > SLOW_FAST_THRESH {
                (TURN_RADIUS_FAST, TURN_RESPONSIVENESS_FAST * adjusted_dt)
            } else {
                (TURN_RADIUS_SLOW, TURN_RESPONSIVENESS_SLOW * adjusted_dt)
            };
        self.turn_angle = match self.turn_input {
            TurnInputState::Left => {
                self.turn_angle * (1.0 - turn_response) + (max_turn_radius * turn_response)
            }
            TurnInputState::Right => {
                self.turn_angle * (1.0 - turn_response) + (-max_turn_radius * turn_response)
            }
            TurnInputState::None => self.turn_angle * (1.0 - (turn_response * 1.5)),
        };
        // lerp throttle
        let throttle_response = THROTTLE_RESPONSIVENESS * adjusted_dt;
        let target_throttle = match self.drive_input {
            DriveInputState::Accelerating => ACCELERATION,
            DriveInputState::HardBraking => 0.0,
            DriveInputState::Reversing => -ACCELERATION * 0.8,
            DriveInputState::Coasting => 0.0,
        };
        self.throttle =
            self.throttle * (1.0 - throttle_response) + (target_throttle * throttle_response);

        let turned_wheel_forward_dir = car_rb.position().rotation.transform_vector(
            &UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.turn_angle)
                .transform_vector(&Vector3::z()),
        );

        // calculate and apply forces from wheels
        let mut wheel_positions: [Point3<f32>; 4] = [Point3::origin(); 4];
        let mut skid_contact_points: [Option<Point3<f32>>; 4] = [None; 4];
        for wheel_i in 0..4 {
            let (ray, maybe_hit) = hits[wheel_i];
            if let Some(intersection) = maybe_hit {
                // tire is on the ground

                let hit_dist = intersection.time_of_impact;
                let contact_point = ray.point_at(hit_dist);

                // suspension forces
                let compression =
                    ((SUSPENSION_MAX + WHEEL_RADIUS) - hit_dist) / (SUSPENSION_MAX + WHEEL_RADIUS);
                let spring_impulse =
                    suspension_compression_curve(compression) * SUSPENSION_STIFFNESS;
                let spring_velocity = car_rb.velocity_at_point(&ray.origin).dot(&ray.dir);
                let damper_impulse = spring_velocity * SUSPENSION_DAMPER;
                let suspension_impulse = car_up_dir * (spring_impulse + damper_impulse);
                car_rb.apply_impulse_at_point(suspension_impulse * adjusted_dt, ray.origin, false);

                // calculating tire orientation
                let wheel_forward_dir = if wheel_i < 2 && self.turn_angle.abs() > 0.01 {
                    turned_wheel_forward_dir
                } else {
                    car_forward_dir
                };
                let wheel_right_dir: Vector3<f32> = wheel_forward_dir.cross(&car_up_dir);
                let tire_velocity: Vector3<f32> = car_rb.velocity_at_point(&contact_point);

                // friction forces
                let lat_force = tire_velocity.normalize().dot(&wheel_right_dir) * -WHEEL_GRIP;
                let long_force = if wheel_i >= 2 {
                    // rwd
                    self.throttle
                } else {
                    0.0
                };
                let mut wheel_forces = Vector2::new(lat_force, long_force);
                let wheel_forces_mag_squared = wheel_forces.magnitude_squared();

                self.wheels_slipping[wheel_i] = wheel_forces_mag_squared > MAX_FRICTION.powi(2);
                if self.wheels_slipping[wheel_i] {
                    // wheel is slipping clamping forces
                    wheel_forces = wheel_forces.normalize() * MAX_FRICTION * 0.95;
                    // boost acceleration when drifting
                    wheel_forces.y *= 1.1;

                    if car_linvel.magnitude() > 1.5 {
                        skid_contact_points[wheel_i] = Some(contact_point);
                    }
                }
                car_rb.apply_impulse_at_point(
                    wheel_right_dir * wheel_forces.x * adjusted_dt,
                    contact_point,
                    false,
                );
                car_rb.apply_impulse_at_point(
                    wheel_forward_dir * wheel_forces.y * adjusted_dt,
                    contact_point,
                    false,
                );
            }

            wheel_positions[wheel_i] =
                ray.point_at(maybe_hit.map_or(SUSPENSION_MAX, |intersection| {
                    intersection.time_of_impact - WHEEL_RADIUS
                }));
        }

        // drag force
        car_rb.apply_impulse(
            car_linvel.scale(-car_linvel.magnitude_squared() * DRAG_COEFFICIENT * adjusted_dt),
            false,
        );

        // apply downforce if car is grounded and moving fast
        if self.wheels_grounded > 0 {
            let downforce = car_linvel.magnitude() * DOWNFORCE_COEFFICIENT * adjusted_dt;
            car_rb.apply_impulse(-car_up_dir.scale(downforce), false);
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

        (wheel_transforms, skid_contact_points)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TurnInputState {
    Left,
    None,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DriveInputState {
    Coasting,
    Accelerating,
    HardBraking,
    Reversing,
}
