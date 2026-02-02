use rapier2d::prelude::*;
use nalgebra::vector;

pub struct PhysicsWorld {
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        // Gravity acting downwards (-Y)
        let gravity = vector![0.0, -9.81];

        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();

        Self {
            rigid_body_set,
            collider_set,
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
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

        self.query_pipeline.update(
            &self.collider_set,
        );
    }
}

// Helper struct for rendering data
#[derive(Debug, Clone)]
pub struct RenderBody {
    pub position: Isometry<Real>,
    pub shape: SharedShape,
}

impl PhysicsWorld {
    pub fn get_render_bodies(&self) -> Vec<RenderBody> {
        let mut bodies = Vec::new();
        for (_, collider) in self.collider_set.iter() {
            if let Some(parent_handle) = collider.parent() {
                if let Some(body) = self.rigid_body_set.get(parent_handle) {
                     let position = body.position() * collider.position();
                     bodies.push(RenderBody {
                         position: position.clone(),
                         shape: collider.shared_shape().clone(),
                     });
                }
            }
        }
        bodies
    }
}
