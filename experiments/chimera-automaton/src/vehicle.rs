use crate::physics::PhysicsWorld;
use rapier2d::prelude::*;

pub struct Vehicle {
    pub chassis: RigidBodyHandle,
    pub left_wheel: RigidBodyHandle,
    pub right_wheel: RigidBodyHandle,
    pub left_joint: ImpulseJointHandle,
    pub right_joint: ImpulseJointHandle,
}

impl Vehicle {
    pub fn spawn(world: &mut PhysicsWorld, x: f32, y: f32) -> Self {
        // Chassis
        let chassis_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .build();
        let chassis = world.rigid_body_set.insert(chassis_rb);
        let chassis_coll = ColliderBuilder::cuboid(2.0, 0.5).density(1.0).build();
        world
            .collider_set
            .insert_with_parent(chassis_coll, chassis, &mut world.rigid_body_set);

        // Wheels
        let wheel_shape = ColliderBuilder::ball(0.8)
            .friction(1.5)
            .density(2.0)
            .build();

        // Left Wheel
        let l_wheel_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x - 1.5, y - 0.5])
            .build();
        let left_wheel = world.rigid_body_set.insert(l_wheel_rb);
        world.collider_set.insert_with_parent(
            wheel_shape.clone(),
            left_wheel,
            &mut world.rigid_body_set,
        );

        // Right Wheel
        let r_wheel_rb = RigidBodyBuilder::dynamic()
            .translation(vector![x + 1.5, y - 0.5])
            .build();
        let right_wheel = world.rigid_body_set.insert(r_wheel_rb);
        world
            .collider_set
            .insert_with_parent(wheel_shape, right_wheel, &mut world.rigid_body_set);

        // Joints with Motors
        let left_joint_data = RevoluteJointBuilder::new()
            .local_anchor1(point![-1.5, -0.5])
            .local_anchor2(point![0.0, 0.0])
            .motor_velocity(0.0, 1000.0) // Enabled by default with 0 speed
            .build();
        let left_joint = world
            .impulse_joint_set
            .insert(chassis, left_wheel, left_joint_data, true);

        let right_joint_data = RevoluteJointBuilder::new()
            .local_anchor1(point![1.5, -0.5])
            .local_anchor2(point![0.0, 0.0])
            .motor_velocity(0.0, 1000.0)
            .build();
        let right_joint =
            world
                .impulse_joint_set
                .insert(chassis, right_wheel, right_joint_data, true);

        Vehicle {
            chassis,
            left_wheel,
            right_wheel,
            left_joint,
            right_joint,
        }
    }

    pub fn set_motor_speeds(&self, world: &mut PhysicsWorld, left: f32, right: f32) {
        if let Some(joint) = world.impulse_joint_set.get_mut(self.left_joint) {
            if let Some(revolute) = joint.data.as_revolute_mut() {
                revolute.set_motor_velocity(left, 10000.0);
            }
        }
        if let Some(joint) = world.impulse_joint_set.get_mut(self.right_joint) {
            if let Some(revolute) = joint.data.as_revolute_mut() {
                revolute.set_motor_velocity(right, 10000.0);
            }
        }

        // Wake up bodies to ensure they respond
        if let Some(body) = world.rigid_body_set.get_mut(self.left_wheel) {
            body.wake_up(true);
        }
        if let Some(body) = world.rigid_body_set.get_mut(self.right_wheel) {
            body.wake_up(true);
        }
    }
}
