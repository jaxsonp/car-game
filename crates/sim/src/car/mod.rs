mod controller;

use car_game_assets::{GameObject, objects::Car};
use car_game_utils::SkidContact;
use nalgebra::{Isometry3, Point3, Rotation3, UnitQuaternion, Vector2, Vector3};
use rapier3d::prelude::*;

use crate::physics::PhysicsHandler;

pub use controller::CarController;

const MASS: f32 = 3000.0;

const ACCELERATION: f32 = 110.0;

const SLOW_FAST_THRESH: f32 = 22.0;

const TURN_RADIUS_SLOW: f32 = 17f32.to_radians();
const TURN_RADIUS_FAST: f32 = 11f32.to_radians();

const TURN_RESPONSIVENESS_SLOW: f32 = 2.5f32.to_radians();
const TURN_RESPONSIVENESS_FAST: f32 = 1.4f32.to_radians();

const THROTTLE_RESPONSIVENESS: f32 = 0.1;

/// max extension of the suspension
const SUSPENSION_MAX: f32 = 0.28;
const SUSPENSION_STIFFNESS: f32 = 2250.0;
const SUSPENSION_DAMPER: f32 = 54.0;

fn suspension_compression_curve(val: f32) -> f32 {
    // nonlinear spring force
    val.powf(2.5)
}

const DRAG_COEFFICIENT: f32 = 0.0039;
const DOWNFORCE_COEFFICIENT: f32 = 19.0;

const MAX_FRICTION: f32 = 240.0;
//const MAX_FRICTION_PER_SPEED: f32 = 180.0;

const WHEEL_DIAMETER: f32 = 0.636653;
const WHEEL_RADIUS: f32 = WHEEL_DIAMETER / 2.0;
const WHEEL_THICKNESS: f32 = 0.292154;
/// Tire grip coefficient
const WHEEL_GRIP: f32 = 1100.0;

pub struct CarHandler {
    pub rb_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub throttle: f32,
    pub turn_angle: f32,

    pub wheels: [WheelInfo; 4],
    pub n_wheels_grounded: u32,
    pub drive_input: DriveInputState,
    pub turn_input: TurnInputState,
}
impl CarHandler {
    pub fn new(physics: &mut PhysicsHandler) -> CarHandler {
        let rbody = RigidBodyBuilder::dynamic()
            .additional_mass(MASS)
            .position(Isometry3::from_parts(
                Point3::new(0.0, 5.0, 250.0).into(),
                Rotation3::identity().into(),
            ))
            .can_sleep(false) // car doesn't sleep
            .build();
        let collider = Car::get_collision_box().build();
        let (rb_handle, collider_handle) = physics.insert_object(rbody, Some(collider));

        CarHandler {
            rb_handle,
            collider_handle: collider_handle.unwrap(),
            turn_angle: 0.0,
            throttle: 0.0,
            wheels: [0, 1, 2, 3].map(|i| WheelInfo::new(i)),
            n_wheels_grounded: 0,
            drive_input: DriveInputState::Coasting,
            turn_input: TurnInputState::None,
        }
    }

    pub fn step(
        &mut self,
        dt: f32,
        physics: &mut PhysicsHandler,
        controller: Option<&CarController>,
        water_level: f32,
    ) {
        let adjusted_dt = dt * 60.0;

        let car_transform = *physics.rigid_bodies[self.rb_handle].position();
        let car_up_dir: Vector3<f32> = (car_transform.rotation * Vector3::y()).normalize();
        let car_forward_dir: Vector3<f32> = (car_transform.rotation * Vector3::z()).normalize();

        let car_rb = &mut physics.rigid_bodies[self.rb_handle];
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
        for wheel in self.wheels.iter_mut() {
            if let Some((toi, _contact_normal)) = wheel.contact {
                // tire is on the ground

                let ray = wheel.ray.unwrap();
                let contact_pos = ray.point_at(toi);

                // suspension forces
                let spring_impulse = suspension_compression_curve(wheel.suspension_compression)
                    * SUSPENSION_STIFFNESS;
                let spring_velocity = car_rb.velocity_at_point(&ray.origin).dot(&ray.dir);
                let damper_impulse = spring_velocity * SUSPENSION_DAMPER;
                let suspension_impulse = car_up_dir * (spring_impulse + damper_impulse);
                car_rb.apply_impulse_at_point(suspension_impulse * adjusted_dt, ray.origin, false);

                // calculating tire orientation
                let wheel_forward_dir = if wheel.i < 2 && self.turn_angle.abs() > 0.01 {
                    turned_wheel_forward_dir
                } else {
                    car_forward_dir
                };
                let wheel_right_dir: Vector3<f32> = wheel_forward_dir.cross(&car_up_dir);
                let tire_velocity: Vector3<f32> = car_rb.velocity_at_point(&contact_pos);

                // friction forces
                let lat_force = tire_velocity.normalize().dot(&wheel_right_dir) * -WHEEL_GRIP;
                let long_force = if wheel.i < 2 {
                    0.0
                } else {
                    // rwd
                    self.throttle
                };
                let mut wheel_forces = Vector2::new(lat_force, long_force);
                let wheel_forces_mag_squared = wheel_forces.magnitude_squared();

                wheel.skidding = false;
                if wheel_forces_mag_squared > MAX_FRICTION.powi(2) {
                    // wheel is slipping, clamping forces
                    wheel_forces = wheel_forces.normalize() * MAX_FRICTION * 0.95;

                    // boost acceleration when drifting
                    //wheel_forces.y *= 1.1;

                    if car_linvel.magnitude() > 1.5 {
                        // only skid above a certain speed
                        wheel.skidding = true;
                    }
                }
                car_rb.apply_impulse_at_point(
                    wheel_right_dir * wheel_forces.x * adjusted_dt,
                    contact_pos,
                    false,
                );
                car_rb.apply_impulse_at_point(
                    wheel_forward_dir * wheel_forces.y * adjusted_dt,
                    contact_pos,
                    false,
                );
            }
        }

        // drag force
        car_rb.apply_impulse(
            car_linvel.scale(-car_linvel.magnitude_squared() * DRAG_COEFFICIENT * adjusted_dt),
            false,
        );

        // apply downforce if car is grounded and moving fast
        if self.n_wheels_grounded > 0 {
            let downforce = car_linvel.magnitude() * DOWNFORCE_COEFFICIENT * adjusted_dt;
            car_rb.apply_impulse(-car_up_dir.scale(downforce), false);
        }

        // ocean float force
        let water_depth = car_rb.center_of_mass().y - water_level;
        if water_depth < 0.0 {
            // underwater
            car_rb.apply_impulse(Vector3::y() * -water_depth * adjusted_dt * 1000.0, false);
            car_rb.set_linear_damping(0.35);
            car_rb.set_angular_damping(0.3);
        } else {
            car_rb.set_linear_damping(0.0);
            car_rb.set_angular_damping(0.0);
        }
    }

    /// Calculate wheel data after stepping world physics for a good reason
    pub fn calculate_wheel_data(&mut self, physics: &mut PhysicsHandler) {
        let car_transform = *physics.rigid_bodies[self.rb_handle].position();
        // cast rays to see if tires are touching the ground
        let query_pipeline = physics.create_query_pipeline(
            QueryFilter::new()
                .exclude_rigid_body(self.rb_handle)
                .groups(physics.colliders[self.collider_handle].collision_groups()),
        );
        self.n_wheels_grounded = 0;
        for i in 0..4 {
            self.wheels[i].calculate_contact(&query_pipeline, &car_transform);
            if self.wheels[i].contact.is_some() {
                self.n_wheels_grounded += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct WheelInfo {
    i: u32,
    /// offset of wheel origin FROM CAR
    offset: Vector3<f32>,
    /// offset of ray origin FROM WHEEL ORIGIN
    ray_origin_offset: Vector3<f32>,
    /// ray used for last contact calculation
    ray: Option<Ray>,
    /// Contact TOI and normal
    pub contact: Option<(f32, Vector3<f32>)>,
    /// 0 is totally uncompressed, 1 is bottomed out
    pub suspension_compression: f32,
    pub skidding: bool,
}
impl WheelInfo {
    fn new(i: usize) -> Self {
        let offset = Vector3::from(Car::WHEEL_OFFSETS[i]);
        let ray_origin_offset = Vector3::new(
            if i % 2 == 0 {
                WHEEL_THICKNESS
            } else {
                -WHEEL_THICKNESS
            },
            0.0,
            0.0,
        );
        WheelInfo {
            i: i as u32,
            offset,
            ray_origin_offset,
            ray: None,
            contact: None,
            suspension_compression: 0.0,
            skidding: false,
        }
    }

    /// Returns the mesh position for this wheel
    fn calculate_contact(
        &mut self,
        query_pipeline: &QueryPipeline,
        car_transform: &Isometry3<f32>,
    ) -> Point3<f32> {
        let car_up_dir: Vector3<f32> = (car_transform.rotation * Vector3::y()).normalize();
        let ray_origin: Point3<f32> =
            car_transform * Point3::from(self.offset + self.ray_origin_offset);
        let ray = Ray::new(ray_origin, -car_up_dir);
        self.ray = Some(ray);
        if let Some((_collider, intersection)) =
            query_pipeline.cast_ray_and_get_normal(&ray, SUSPENSION_MAX + WHEEL_RADIUS, false)
        {
            self.contact = Some((intersection.time_of_impact, intersection.normal));
            self.suspension_compression = ((SUSPENSION_MAX + WHEEL_RADIUS)
                - intersection.time_of_impact)
                / (SUSPENSION_MAX + WHEEL_RADIUS);

            car_transform * ray.point_at(intersection.time_of_impact - WHEEL_RADIUS)
        } else {
            self.contact = None;
            self.suspension_compression = 0.0;

            car_transform * ray.point_at(SUSPENSION_MAX)
        }
    }

    pub fn get_mesh_transform(
        &self,
        car_transform: &Isometry3<f32>,
        turn_angle: f32,
    ) -> Isometry3<f32> {
        Isometry3::from_parts(
            (self
                .ray
                .unwrap()
                .point_at(
                    self.contact
                        .map_or(SUSPENSION_MAX, |(toi, _)| toi - WHEEL_RADIUS),
                )
                .coords
                - (car_transform * self.ray_origin_offset))
                .into(),
            if self.i < 2 {
                car_transform.rotation
                    * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), turn_angle)
            } else {
                car_transform.rotation
            },
        )
    }

    pub fn get_skid_contact(&self) -> Option<SkidContact> {
        if self.skidding {
            self.contact.map(|(toi, normal)| SkidContact {
                pos: self.ray.unwrap().point_at(toi),
                normal,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TurnInputState {
    Left,
    None,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriveInputState {
    Coasting,
    Accelerating,
    HardBraking,
    Reversing,
}
