use nalgebra::Vector2;
use rapier2d::prelude::*;

pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub gravity: Vector2<f32>,

    pub sensor_handle: RigidBodyHandle,
    pub wheel_handle: RigidBodyHandle,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        // 1. Create Sensor (Lever)
        // A dynamic body that knots will collide with.
        let sensor_rb = RigidBodyBuilder::dynamic()
            .translation(Vector2::new(0.0, 0.0))
            .linear_damping(5.0) // High damping to stop it bouncing too much
            .lock_rotations() // Lock rotation, just slide X/Y
            .build();
        let sensor_handle = rigid_body_set.insert(sensor_rb);

        // Sensor Collider: A Ball (to allow smooth sliding)
        let sensor_collider = ColliderBuilder::ball(1.5).restitution(0.5).build();
        collider_set.insert_with_parent(sensor_collider, sensor_handle, &mut rigid_body_set);

        // 2. Create Integrator Wheel
        // A kinematic body that rotates based on integration result.
        let wheel_rb = RigidBodyBuilder::kinematic_velocity_based()
            .translation(Vector2::new(10.0, 0.0))
            .build();
        let wheel_handle = rigid_body_set.insert(wheel_rb);
        // Using sensor=true so it doesn't collide physically with anything, just spins visually
        let wheel_collider = ColliderBuilder::ball(4.0).sensor(true).build();
        collider_set.insert_with_parent(wheel_collider, wheel_handle, &mut rigid_body_set);

        Self {
            rigid_body_set,
            collider_set,
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity: Vector2::new(0.0, 0.0),
            sensor_handle,
            wheel_handle,
        }
    }

    pub fn create_knot_body(&mut self, x: f32, y: f32, radius: f32) -> RigidBodyHandle {
        let rb = RigidBodyBuilder::kinematic_velocity_based()
            .translation(Vector2::new(x, y))
            .linvel(Vector2::new(0.0, -5.0)) // Moving down
            .build();
        let handle = self.rigid_body_set.insert(rb);
        let collider = ColliderBuilder::ball(radius).build();
        self.collider_set
            .insert_with_parent(collider, handle, &mut self.rigid_body_set);
        handle
    }

    pub fn remove_body(&mut self, handle: RigidBodyHandle) {
        self.rigid_body_set.remove(
            handle,
            &mut self.island_manager,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            &(), // physics_hooks
            &(), // event_handler
        );
    }
}
