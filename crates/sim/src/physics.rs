use std::cell::OnceCell;

use nalgebra::Vector3;
use rapier3d::prelude::*;

const GRAVITY: f32 = 9.81;

const UNFLIP_DURATION: f32 = 2.1;
const UNFLIP_UP_FORCE: f32 = 40_000.0;

pub struct PhysicsHandler {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    integration_params: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,

    /// remember for convenience
    pub car_rb_handle: OnceCell<RigidBodyHandle>,
    car_unflipping_time_left: f32,
}
impl PhysicsHandler {
    pub fn new() -> PhysicsHandler {
        PhysicsHandler {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            physics_pipeline: PhysicsPipeline::new(),
            integration_params: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),

            car_rb_handle: OnceCell::new(),
            car_unflipping_time_left: 0.0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.integration_params.dt = dt;

        self.physics_pipeline.step(
            &vector![0.0, -GRAVITY, 0.0],
            &self.integration_params,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &(),
        );

        if self.car_unflipping_time_left > 0.0 {
            if let Some(rb_handle) = self.car_rb_handle.get() {
                let car_rb = &mut self.rigid_bodies[*rb_handle];

                // applying a torque to self-right car
                let up = Vector3::y();
                let car_up: Vector3<f32> = car_rb.rotation() * up;
                if car_up.dot(&up).acos() < 0.02 {
                    // if upright, stop self-righting
                    self.car_unflipping_time_left = 0.0;
                    car_rb.set_angvel(Vector3::zeros(), true);
                } else {
                    let torque_axis: Vector3<f32> = car_up.cross(&up).normalize();
                    car_rb.apply_torque_impulse(torque_axis * 1000.0, true);

                    // angular damping
                    car_rb.apply_torque_impulse(car_rb.angvel() * -250.0, true);
                }
            }

            self.car_unflipping_time_left -= dt;
        } else if self.car_unflipping_time_left < 0.0 {
            self.car_unflipping_time_left = 0.0;
            // diminish angular velocity when done unflipping
            if let Some(rb_handle) = self.car_rb_handle.get() {
                let car_rb = &mut self.rigid_bodies[*rb_handle];
                car_rb.set_angvel(car_rb.angvel() * 0.5, true);
            }
        }
    }

    /// Insert a rigid body and optionally an associated collider into the scene. Returns the respective handle(s)
    pub fn insert_object(
        &mut self,
        rb: RigidBody,
        collider: Option<Collider>,
    ) -> (RigidBodyHandle, Option<ColliderHandle>) {
        let rb_handle = self.rigid_bodies.insert(rb);
        let collider_handle = collider.map(|collider| {
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies)
        });
        return (rb_handle, collider_handle);
    }

    pub fn create_query_pipeline<'a>(&'a mut self, filter: QueryFilter<'a>) -> QueryPipeline<'a> {
        self.broad_phase.as_query_pipeline(
            self.narrow_phase.query_dispatcher(),
            &self.rigid_bodies,
            &self.colliders,
            filter,
        )
    }

    pub fn unflip_car(&mut self) {
        log::debug!("Unflipping car");
        let car_rb_handle = self.car_rb_handle.get();
        if car_rb_handle.is_none() {
            log::error!("car handle not set");
            return;
        }
        let car_rb_handle = *car_rb_handle.unwrap();
        let car_rb = &mut self.rigid_bodies[car_rb_handle];

        car_rb.apply_impulse(Vector3::y() * UNFLIP_UP_FORCE, true);

        self.car_unflipping_time_left = UNFLIP_DURATION;
    }
}
