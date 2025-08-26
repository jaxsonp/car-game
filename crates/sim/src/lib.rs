use assets::GameObject;
use nalgebra::{Isometry3, Point3, Rotation3, Vector3};
use rapier3d::{na::UnitQuaternion, prelude::*};
use utils::*;

const GRAVITY: f32 = 9.81;

pub struct GameSimulation {
    rigid_bodies: RigidBodySet,
    colliders: ColliderSet,

    car_rbody_handle: RigidBodyHandle,

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
        use assets::objects;

        let mut rigid_bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        let floor_rbody = RigidBodyBuilder::new(RigidBodyType::Fixed).build();
        let floor_rbody_handle = rigid_bodies.insert(floor_rbody);
        let floor_collider = objects::TestFloor::get_collision_box();
        colliders.insert_with_parent(floor_collider, floor_rbody_handle, &mut rigid_bodies);

        let car_rbody = RigidBodyBuilder::dynamic()
            .additional_mass(objects::Car::MASS)
            .linvel(Vector3::new(4.0, 6.0, 0.0))
            .angvel(Vector3::new(0.0, -0.8, 0.1))
            .position(Isometry3::from_parts(
                Point3::new(0.0, 7.0, 0.0).into(),
                Rotation3::identity().into(),
            ))
            .build();
        let car_rbody_handle = rigid_bodies.insert(car_rbody);
        let car_collider = objects::Car::get_collision_box().build();
        colliders.insert_with_parent(car_collider, car_rbody_handle, &mut rigid_bodies);

        let physics_pipeline = PhysicsPipeline::new();
        let integration_params = IntegrationParameters::default();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        GameSimulation {
            rigid_bodies,
            colliders,

            car_rbody_handle,

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
        // applying time delta
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

        let wheel_transforms = self.do_car_physics();

        RenderSnapshot {
            car_transform: *self.rigid_bodies[self.car_rbody_handle].position(),
            debug_string: Some("test".to_owned()),
            wheel_transforms,
        }
    }

    /// doin car physics
    fn do_car_physics(&mut self) -> [Isometry3<f32>; 4] {
        use assets::objects::Car;

        let car_transform = *self.rigid_bodies[self.car_rbody_handle].position();
        let car_up_direction = (car_transform.rotation * Vector3::y()).normalize();

        // cast rays to see if tires are touching the ground
        let hits = {
            let query_pipeline = self.broad_phase.as_query_pipeline(
                self.narrow_phase.query_dispatcher(),
                &self.rigid_bodies,
                &self.colliders,
                QueryFilter::new().exclude_rigid_body(self.car_rbody_handle),
            );
            Car::WHEEL_OFFSETS.map(|wheel_offset| {
                let ray_origin = car_transform * Point3::from(wheel_offset);
                let ray = Ray::new(ray_origin, -car_up_direction);
                if let Some((_collider, hit_dist)) =
                    query_pipeline.cast_ray(&ray, Car::SUSPENSION_MAX + Car::WHEEL_RADIUS, false)
                {
                    (ray, Some(hit_dist))
                } else {
                    (ray, None)
                }
            })
        };

        let car_rb = &mut self.rigid_bodies[self.car_rbody_handle];
        car_rb.wake_up(false);

        // calculate and apply forces
        let wheel_positions = hits.map(|(ray, maybe_hit)| {
            if let Some(hit_dist) = maybe_hit {
                // how far the spring is compressed
                let mut compression = ((Car::SUSPENSION_MAX + Car::WHEEL_RADIUS) - hit_dist)
                    / (Car::SUSPENSION_MAX + Car::WHEEL_RADIUS);
                // apply curve
                compression = Car::suspension_compression_curve(compression);

                // velocity of the suspension
                let spring_velocity = car_rb.velocity_at_point(&ray.origin).dot(&ray.dir);

                let spring_impulse = compression * Car::SUSPENSION_STIFFNESS;
                let damper_impulse = spring_velocity * Car::SUSPENSION_DAMPER;
                let suspension_impulse = car_up_direction * (spring_impulse + damper_impulse);

                car_rb.apply_impulse_at_point(suspension_impulse, ray.origin, false);

                ray.point_at(hit_dist - Car::WHEEL_RADIUS)
            } else {
                ray.point_at(Car::SUSPENSION_MAX)
            }
        });

        wheel_positions.map(|pos| Isometry3::from_parts(pos.into(), *car_rb.rotation()))
    }

    pub fn update_camera(&self, t_delta: f32, cam: &mut Camera) {
        // time delta adjusted lerp scalars
        const CAM_LERP: f32 = 4.0;

        const CAM_EYE_OFFSET: Vector3<f32> = Vector3::new(0.0, 5.0, -8.0);
        // camera target offset in world space
        const CAM_TARGET_OFFSET: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);

        let car_rb = &self.rigid_bodies[self.car_rbody_handle];
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
        cam.eye = cam.eye.lerp(&target_eye, CAM_LERP * t_delta);

        let target_target: Point3<f32> =
            (car_transform.translation.vector + CAM_TARGET_OFFSET).into();
        cam.target = cam.target.lerp(&target_target, CAM_LERP * t_delta);

        cam.up = cam.up.lerp(&Vector3::y(), CAM_LERP * t_delta);
    }
}
