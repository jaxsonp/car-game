use rapier3d::prelude::*;

const GRAVITY: f32 = 9.81;

pub struct PhysicsHandler {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    physics_pipeline: PhysicsPipeline,
    integration_params: IntegrationParameters,
    island_manager: IslandManager,
    pub broad_phase: BroadPhaseBvh,
    pub narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
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
        }
    }

    pub fn step(&mut self, adjusted_td: f32) {
        self.integration_params.dt = adjusted_td / 60.0;

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
}
