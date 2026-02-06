use rapier2d::prelude::*;
use crate::physics::PhysicsWorld;

pub struct Puppet {
    pub torso: RigidBodyHandle,
    pub left_leg: RigidBodyHandle,
    pub right_leg: RigidBodyHandle,
}

impl Puppet {
    pub fn spawn(world: &mut PhysicsWorld, x: f32, y: f32) -> Self {
        // Torso
        let torso_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .build();
        let torso = world.rigid_body_set.insert(torso_rb);
        let torso_coll = ColliderBuilder::cuboid(1.0, 2.0).build();
        world.collider_set.insert_with_parent(torso_coll, torso, &mut world.rigid_body_set);

        // Legs
        let leg_shape = ColliderBuilder::cuboid(0.3, 1.5).build();

        // Left Leg
        let l_leg_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x - 0.5, y - 2.0])
            .build();
        let left_leg = world.rigid_body_set.insert(l_leg_rb);
        world.collider_set.insert_with_parent(leg_shape.clone(), left_leg, &mut world.rigid_body_set);

        // Joint Torso-LeftLeg (Hip)
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![-0.5, -2.0])
            .local_anchor2(point![0.0, 1.5])
            .build();
        world.impulse_joint_set.insert(torso, left_leg, joint, true);

        // Right Leg
        let r_leg_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x + 0.5, y - 2.0])
            .build();
        let right_leg = world.rigid_body_set.insert(r_leg_rb);
        world.collider_set.insert_with_parent(leg_shape, right_leg, &mut world.rigid_body_set);

        // Joint Torso-RightLeg
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(point![0.5, -2.0])
            .local_anchor2(point![0.0, 1.5])
            .build();
        world.impulse_joint_set.insert(torso, right_leg, joint, true);

        // Hang the puppet from a static point so it dangles
        let anchor = RigidBodyBuilder::fixed().translation(vector![x, y + 5.0]).build();
        let anchor_h = world.rigid_body_set.insert(anchor);
        let hanger = RopeJointBuilder::new(5.0)
             .local_anchor1(point![0.0, 0.0])
             .local_anchor2(point![0.0, 2.0])
             .build();
        world.impulse_joint_set.insert(anchor_h, torso, hanger, true);

        Puppet { torso, left_leg, right_leg }
    }

    pub fn attach_rod(world: &mut PhysicsWorld, follower: RigidBodyHandle, limb: RigidBodyHandle) {
        // A stiff rod connecting follower to limb
        // We get the current positions to set the rest length
        let p1 = world.rigid_body_set[follower].translation();
        let p2 = world.rigid_body_set[limb].translation();
        let dist = (p1 - p2).magnitude();

        let joint = SpringJointBuilder::new(dist, 10000.0, 10.0) // Stiff spring
             .local_anchor1(point![0.0, 1.0]) // Top of follower
             .local_anchor2(point![0.0, -1.5]) // Bottom of leg
             .build();
        world.impulse_joint_set.insert(follower, limb, joint, true);
    }
}
