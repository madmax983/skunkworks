//! # Physics Engine
//!
//! This module handles the physical simulation of the Jenga tower using `rapier2d`.
//!
//! It manages the lifecycle of rigid bodies (blocks), colliders, and the physics pipeline.
//! The simulation is 2D, with gravity acting downwards.
//!
//! ## Coordinate System
//!
//! *   **Physics World:** Standard Cartesian coordinates. `+Y` is up. `(0,0)` is the center of the base.
//! *   **Render World:** TUI coordinates. `+Y` is down (row index).
//!
//! The translation between these two systems happens in `main.rs` during rendering and input handling.
//!
//! ## Key Components
//!
//! *   [`PhysicsWorld`]: The main container for all simulation state.
//! *   [`RenderBody`]: A snapshot of a body's position and shape, optimized for the renderer.

use crate::deps::CrateBlock;
use rapier2d::prelude::*;
use std::collections::HashMap;

/// The main simulation container.
///
/// Holds all the `rapier2d` structures required to step the simulation.
///
/// # Fields
///
/// *   `rigid_body_set`: Stores dynamic bodies (the blocks).
/// *   `collider_set`: Stores shapes attached to bodies.
/// *   `body_info`: Metadata (name, color) linked to physics handles.
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

    /// Maps physics handles to game-specific data (name, color).
    pub body_info: HashMap<RigidBodyHandle, BodyInfo>,
}

/// Metadata for a physics body.
///
/// This is used to render the block correctly (color) and display its name.
#[derive(Clone, Debug)]
pub struct BodyInfo {
    pub name: String,
    pub color: (u8, u8, u8),
}

/// A decoupled representation of a body for rendering.
///
/// This struct allows the rendering thread to draw the scene without holding
/// locks on the physics world or knowing about Rapier internals.
#[derive(Clone)]
pub struct RenderBody {
    /// The absolute position (translation + rotation) of the body.
    pub position: Isometry<Real>,
    /// The geometric shape of the body (usually a Cuboid).
    pub shape: SharedShape,
    /// Visual metadata (color, label), if available.
    pub info: Option<BodyInfo>,
}

impl PhysicsWorld {
    /// Creates a new, empty physics world.
    ///
    /// Initializes all Rapier pipelines and sets gravity to -9.81 on the Y axis.
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

    /// Spawns a static ground plane.
    ///
    /// The ground is a large cuboid positioned just below `y=0`.
    pub fn spawn_ground(&mut self) {
        let collider = ColliderBuilder::cuboid(100.0, 1.0)
            .translation(vector![0.0, -1.0]) // Top at 0.0
            .build();
        self.collider_set.insert(collider);
    }

    /// Spawns the Jenga tower from a list of crates.
    ///
    /// Blocks are stacked vertically starting from `y=0`.
    ///
    /// # Jitter
    ///
    /// A small random X-offset is applied to each block to simulate imperfection
    /// and make the tower mechanically interesting (unstable).
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

    /// Advances the simulation by one timestep.
    ///
    /// This should be called once per frame (or at a fixed timestep).
    /// It updates positions, velocities, and resolves collisions.
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

    /// Extracts a list of bodies for rendering.
    ///
    /// This flattens the physics hierarchy into a simple list of shapes and positions.
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

    /// Removes a physics body at the given coordinates (world space).
    ///
    /// Used for mouse interaction (clicking to remove a block).
    /// It queries the physics world to find if a collider exists at `(x, y)`.
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
