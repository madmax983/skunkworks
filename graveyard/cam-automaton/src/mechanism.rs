use crate::physics::PhysicsWorld;
use nalgebra::Vector2;
use rapier2d::prelude::*;

pub struct Mechanism;

impl Mechanism {
    pub fn create_camshaft(world: &mut PhysicsWorld, position: Vector2<f32>) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::kinematic_position_based()
            .translation(position)
            .build();
        world.rigid_body_set.insert(rigid_body)
    }

    pub fn add_cam(
        world: &mut PhysicsWorld,
        shaft_handle: RigidBodyHandle,
        shape: SharedShape,
        offset: Vector2<f32>,
    ) {
        let collider = ColliderBuilder::new(shape)
            .position(Isometry::new(offset, 0.0))
            .friction(0.0)
            .build();

        world
            .collider_set
            .insert_with_parent(collider, shaft_handle, &mut world.rigid_body_set);
    }

    pub fn create_follower(world: &mut PhysicsWorld, x: f32, y: f32) -> RigidBodyHandle {
        // Dynamic body that moves vertically
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .lock_rotations() // Cannot rotate
            .build();

        let handle = world.rigid_body_set.insert(rigid_body);

        // Add a collider (the follower foot)
        let collider = ColliderBuilder::cuboid(0.2, 1.0).friction(0.0).build();
        world
            .collider_set
            .insert_with_parent(collider, handle, &mut world.rigid_body_set);

        // Create a static anchor for the joint
        let anchor = RigidBodyBuilder::fixed().translation(vector![x, y]).build();
        let anchor_handle = world.rigid_body_set.insert(anchor);

        // Prismatic joint allowing only Y movement
        let joint = PrismaticJointBuilder::new(Vector::y_axis()).build();

        world
            .impulse_joint_set
            .insert(anchor_handle, handle, joint, true);

        handle
    }
}
