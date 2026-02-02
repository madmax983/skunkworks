use crate::deps::CrateBlock;
use rapier2d::prelude::*;
use std::collections::HashMap;

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
    pub query_pipeline: QueryPipeline,
    pub gravity: Vector<Real>,

    // Metadata mapping
    pub body_info: HashMap<RigidBodyHandle, BodyInfo>,
}

#[derive(Clone, Debug)]
pub struct BodyInfo {
    pub name: String,
    pub color: (u8, u8, u8),
}

#[derive(Clone)]
pub struct RenderBody {
    pub position: Isometry<Real>,
    pub shape: SharedShape,
    pub info: Option<BodyInfo>,
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
            query_pipeline: QueryPipeline::new(),
            gravity: vector![0.0, -9.81],
            body_info: HashMap::new(),
        }
    }

    pub fn spawn_ground(&mut self) {
        let collider = ColliderBuilder::cuboid(100.0, 1.0)
            .translation(vector![0.0, -1.0]) // Top at 0.0
            .build();
        self.collider_set.insert(collider);
    }

    pub fn spawn_tower(&mut self, crates: &[CrateBlock]) {
        let mut y_offset = 0.5; // First block center height (0.0 + 0.5)

        for block in crates {
            let width = block.width;
            let height = block.height;
            let half_w = width / 2.0;
            let half_h = height / 2.0;

            // Introduce slight random jitter to x to make it realistic/unstable
            let jitter = (rand::random::<f32>() - 0.5) * 0.2;

            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![jitter, y_offset])
                .build();

            let collider = ColliderBuilder::cuboid(half_w, half_h)
                .restitution(0.1)
                .friction(0.8) // High friction for Jenga
                .density(1.0)
                .build();

            let body_handle = self.rigid_body_set.insert(rigid_body);
            self.collider_set
                .insert_with_parent(collider, body_handle, &mut self.rigid_body_set);

            self.body_info.insert(
                body_handle,
                BodyInfo {
                    name: block.name.clone(),
                    color: block.color,
                },
            );

            y_offset += height + 0.1; // Gap
        }
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
            None,
            &(),
            &(),
        );

        self.query_pipeline.update(&self.collider_set);
    }

    pub fn get_render_bodies(&self) -> Vec<RenderBody> {
        let mut bodies = Vec::new();
        for (handle, body) in self.rigid_body_set.iter() {
            let pos = *body.position();
            let info = self.body_info.get(&handle).cloned();

            for collider_handle in body.colliders() {
                if let Some(collider) = self.collider_set.get(*collider_handle) {
                    let shape = collider.shared_shape().clone();
                    bodies.push(RenderBody {
                        position: pos
                            * collider
                                .position_wrt_parent()
                                .unwrap_or(&Isometry::identity()),
                        shape,
                        info: info.clone(),
                    });
                }
            }
        }
        bodies
    }

    pub fn remove_body_at(&mut self, x: f32, y: f32) {
        let point = point![x, y];
        let mut handle_to_remove = None;

        // Use query pipeline for efficiency, or just iterate if simple.
        // Let's use simple iteration first to match previous logic logic, but correct the call.

        for (_handle, collider) in self.collider_set.iter() {
            if let Some(parent_handle) = collider.parent() {
                let parent_body = &self.rigid_body_set[parent_handle];
                let pos = parent_body.position();
                // Using SharedShape's contains_point via PointQuery trait
                if collider.shape().contains_point(pos, &point) {
                    handle_to_remove = Some(parent_handle);
                    break;
                }
            }
        }

        if let Some(h) = handle_to_remove {
            // Remove from metadata
            self.body_info.remove(&h);

            // Remove from physics
            self.rigid_body_set.remove(
                h,
                &mut self.island_manager,
                &mut self.collider_set,
                &mut self.impulse_joint_set,
                &mut self.multibody_joint_set,
                true,
            );
        }
    }
}
