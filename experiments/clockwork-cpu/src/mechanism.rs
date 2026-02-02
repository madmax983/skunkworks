use crate::physics::PhysicsWorld;
use nalgebra::{point, vector};
use rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct Clockwork {
    pub wheel_handle: RigidBodyHandle,
    pub anchor_handle: RigidBodyHandle,
}

impl Clockwork {
    pub fn build(world: &mut PhysicsWorld) -> Self {
        let wheel_radius = 2.0;
        let tooth_len = 0.4;
        let num_teeth = 15;

        // 1. Create Escape Wheel
        // Compound shape for teeth
        let mut shapes = Vec::new();
        let tooth_shape = SharedShape::cuboid(tooth_len / 2.0, 0.1); // thin tooth

        for i in 0..num_teeth {
            let angle = (i as f32) * 2.0 * PI / (num_teeth as f32);
            // Position on rim
            let x = wheel_radius * angle.cos();
            let y = wheel_radius * angle.sin();
            // Rotate to point outward
            let rotation = angle;

            let pos = Isometry::new(vector![x, y], rotation);
            shapes.push((pos, tooth_shape.clone()));
        }

        // Add a central rim
        shapes.push((Isometry::identity(), SharedShape::ball(wheel_radius - 0.1)));

        let wheel_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 0.0])
            .ccd_enabled(true)
            .angular_damping(0.5)
            .linear_damping(0.0)
            .build();

        let wheel_handle = world.rigid_body_set.insert(wheel_body);

        let wheel_collider = ColliderBuilder::compound(shapes)
            .collision_groups(InteractionGroups::new(Group::GROUP_1, Group::GROUP_2)) // Group 1, interacts with 2
            .density(1.0)
            .build();

        world.collider_set.insert_with_parent(
            wheel_collider,
            wheel_handle,
            &mut world.rigid_body_set,
        );

        // Fix wheel to center with hinge
        let wheel_joint = RevoluteJointBuilder::new()
            .local_anchor1(point![0.0, 0.0])
            .local_anchor2(point![0.0, 0.0])
            .build();
        world.impulse_joint_set.insert(
            wheel_handle,
            world
                .rigid_body_set
                .insert(RigidBodyBuilder::fixed().build()),
            wheel_joint,
            true,
        );

        // 2. Create Anchor + Pendulum
        let anchor_pivot_y = wheel_radius + 1.5;

        // Pallets geometry (approximated)
        let pallet_shape = SharedShape::cuboid(0.3, 0.6);

        let anchor_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, anchor_pivot_y])
            .ccd_enabled(true)
            .angular_damping(0.1)
            .linear_damping(0.0)
            .build();

        let anchor_handle = world.rigid_body_set.insert(anchor_body);

        // Collider for Anchor
        // We define shapes relative to the body center (pivot)
        let mut anchor_shapes = Vec::new();

        // Pallet 1 (Left)
        // Positioned relative to pivot (0,0 in local space)
        anchor_shapes.push((
            Isometry::new(vector![-1.5, -1.5], -0.5),
            pallet_shape.clone(),
        ));
        // Pallet 2 (Right)
        anchor_shapes.push((Isometry::new(vector![1.5, -1.5], 0.5), pallet_shape.clone()));

        // Pendulum Rod (Visual mainly, but mass matters)
        let rod_len = 6.0;
        anchor_shapes.push((
            Isometry::new(vector![0.0, -rod_len / 2.0], 0.0),
            SharedShape::cuboid(0.1, rod_len / 2.0),
        ));

        // Pendulum Bob
        anchor_shapes.push((
            Isometry::new(vector![0.0, -rod_len], 0.0),
            SharedShape::ball(0.5),
        ));

        let anchor_collider = ColliderBuilder::compound(anchor_shapes)
            .collision_groups(InteractionGroups::new(Group::GROUP_2, Group::GROUP_1)) // Group 2, interacts with 1
            .density(2.0) // Heavier
            .build();

        world.collider_set.insert_with_parent(
            anchor_collider,
            anchor_handle,
            &mut world.rigid_body_set,
        );

        // Hinge for Anchor at its origin
        let anchor_joint = RevoluteJointBuilder::new()
            .local_anchor1(point![0.0, 0.0]) // Center of Anchor Body
            .local_anchor2(point![0.0, 0.0]) // Center of Pivot Body
            .build();

        // We need a fixed body at the pivot point
        let pivot_body = RigidBodyBuilder::fixed()
            .translation(vector![0.0, anchor_pivot_y])
            .build();
        let pivot_handle = world.rigid_body_set.insert(pivot_body);

        world
            .impulse_joint_set
            .insert(anchor_handle, pivot_handle, anchor_joint, true);

        Self {
            wheel_handle,
            anchor_handle,
        }
    }
}
