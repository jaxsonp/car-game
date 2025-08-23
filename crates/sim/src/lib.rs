use assets::GameObject;
use nalgebra::{Point3, Rotation3, Vector3};
use rapier3d::prelude::*;

const GRAVITY: f32 = 9.81;

pub struct GameSimulation {
    rigid_bodies: RigidBodySet,
    colliders: ColliderSet,

    physics_pipeline: PhysicsPipeline,
    integration_params: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
}

impl GameSimulation {
    pub fn new() -> GameSimulation {
        let mut rigid_bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        let floor_rbody = RigidBodyBuilder::new(RigidBodyType::Fixed).build();
        let floor_rbody_handle = rigid_bodies.insert(floor_rbody);
        let floor_collider = ColliderBuilder::heightfield(
            DMatrix::repeat(2, 2, 0.0),
            Vector3::new(40.0f32, 1.0, 40.0),
        )
        .translation(Vector::new(-20.0, 0.0, -20.0))
        .build();
        colliders.insert_with_parent(floor_collider, floor_rbody_handle, &mut rigid_bodies);

        let car_rbody = RigidBodyBuilder::new(RigidBodyType::Dynamic)
            .additional_mass(100.0)
            .build();
        let car_rbody_handle = rigid_bodies.insert(car_rbody);
        let car_collider = assets::objects::Car::get_collision_box().build();
        colliders.insert_with_parent(car_collider, car_rbody_handle, &mut rigid_bodies);

        let physics_pipeline = PhysicsPipeline::new();
        let integration_params = IntegrationParameters::default();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        //let query_pipeline = QueryPipeline::new();
        //let physics_hooks = ();
        //let event_handler = ();

        GameSimulation {
            rigid_bodies,
            colliders,
            physics_pipeline,
            integration_params,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joints,
            multibody_joints,
            ccd_solver,
        }
    }

    pub fn step(&mut self, t_delta: f32) -> RenderSnapshot {
        self.integration_params.dt = t_delta;

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

        RenderSnapshot {
            car_pos: Point3::new(5.0, 5.0, 5.0),
            car_rotation: Rotation3::identity(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct RenderSnapshot {
    pub car_pos: Point3<f32>,
    pub car_rotation: Rotation3<f32>,
}
