use nalgebra::Vector2;
use rapier2d::prelude::*;

pub struct MechanicalIntegrator {
    pub disk_handle: RigidBodyHandle,
    pub ball_handle: RigidBodyHandle,
    pub output_handle: RigidBodyHandle,
    pub disk_center: Vector2<f32>,
}

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
    pub integrators: Vec<MechanicalIntegrator>,
    pub couplings: Vec<(usize, usize, f32)>, // (source_idx, target_idx, gain)
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity: Vector2::new(0.0, 0.0),
            integrators: Vec::new(),
            couplings: Vec::new(),
        }
    }

    pub fn add_integrator(&mut self, position: Vector2<f32>) -> usize {
        // Disk (Input) - Kinematic, driven by constant speed usually
        let disk_rb = RigidBodyBuilder::kinematic_velocity_based()
            .translation(position)
            .angvel(1.0) // Default constant speed
            .build();
        let disk_handle = self.rigid_body_set.insert(disk_rb);
        let disk_collider = ColliderBuilder::ball(10.0).sensor(true).build(); // Sensor so ball doesn't collide physically
        self.collider_set
            .insert_with_parent(disk_collider, disk_handle, &mut self.rigid_body_set);

        // Ball (Value) - Dynamic, driven by couplings or external force
        // We constrain it to Y=0 relative to disk center (only moving in X)
        let ball_rb = RigidBodyBuilder::dynamic()
            .translation(position + Vector2::new(5.0, 0.0))
            .linear_damping(5.0) // High damping to simulate friction/control
            .lock_rotations() // Prevent ball from spinning
            .build();
        // Wait, lock_translations locks all. We want to lock Y only.
        // RigidBodyBuilder doesn't have partial lock easily?
        // We can use lock_translations and manually set position? Or use PrismaticJoint.
        // Let's use PrismaticJoint to ground.

        let ball_handle = self.rigid_body_set.insert(ball_rb);
        // We need to re-configure ball to allow X movement.
        // Actually, just set `lock_translations` to false and use a joint.
        // But for now, let's just set linear velocity in X and 0 in Y every frame.

        let ball_collider = ColliderBuilder::ball(1.0).build();
        self.collider_set
            .insert_with_parent(ball_collider, ball_handle, &mut self.rigid_body_set);

        // Output Cylinder - Dynamic, driven by integrator logic
        let output_rb = RigidBodyBuilder::dynamic()
            .translation(position + Vector2::new(0.0, 15.0)) // Above disk
            .angular_damping(1.0)
            .build();
        let output_handle = self.rigid_body_set.insert(output_rb);
        let output_collider = ColliderBuilder::cuboid(2.0, 5.0).build();
        self.collider_set.insert_with_parent(
            output_collider,
            output_handle,
            &mut self.rigid_body_set,
        );

        self.integrators.push(MechanicalIntegrator {
            disk_handle,
            ball_handle,
            output_handle,
            disk_center: position,
        });

        self.integrators.len() - 1
    }

    pub fn add_coupling(&mut self, source_idx: usize, target_idx: usize, gain: f32) {
        self.couplings.push((source_idx, target_idx, gain));
    }

    pub fn update_mechanics(&mut self) {
        // 1. Integrator Logic: Drive Output Cylinder based on Disk and Ball
        let mut cylinder_updates = Vec::new();
        for integrator in &self.integrators {
            if let (Some(disk), Some(ball)) = (
                self.rigid_body_set.get(integrator.disk_handle),
                self.rigid_body_set.get(integrator.ball_handle),
            ) {
                let w_disk = disk.angvel();
                let r_ball = ball.translation().x - integrator.disk_center.x;

                // Integrator Equation: w_cyl = k * w_disk * r
                // k = 1.0 for simplicity
                let w_cyl = w_disk * r_ball * 0.5; // Gain factor

                cylinder_updates.push((integrator.output_handle, w_cyl));
            }
        }

        // Apply cylinder updates
        for (handle, w) in cylinder_updates {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                body.set_angvel(w, true);
            }
        }

        // 2. Coupling Logic: Drive Target Ball based on Source Output
        let mut ball_updates = Vec::new();
        for (source_idx, target_idx, gain) in &self.couplings {
            if let Some(source_body) = self
                .rigid_body_set
                .get(self.integrators[*source_idx].output_handle)
            {
                let w_source = source_body.angvel();
                // Target ball velocity = w_source * gain
                let v_target = w_source * *gain;
                ball_updates.push((self.integrators[*target_idx].ball_handle, v_target));
            }
        }

        // Apply ball updates
        for (handle, v_x) in ball_updates {
            if let Some(body) = self.rigid_body_set.get_mut(handle) {
                // Constrain Y to 0 relative to disk center?
                // We just set linear velocity (v_x, 0.0) to keep it on track physically.
                // Assuming the ball is at the correct Y.
                // We also need to correct Y drift if any.
                // Find which integrator this ball belongs to, to correct Y?
                // Too expensive to search. We assume it stays on Y due to 0 Y-velocity.
                body.set_linvel(Vector2::new(v_x, 0.0), true);
            }
        }
    }

    pub fn step(&mut self) {
        // Apply mechanics constraints before physics step
        self.update_mechanics();

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
