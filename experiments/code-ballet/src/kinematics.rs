use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::skeleton::{Motor, DancerPart, Dancer};

pub struct KinematicsPlugin;

impl Plugin for KinematicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (balance_control, apply_motor_forces).chain());
    }
}

// System to apply the abstract Motor parameters to the physical ImpulseJoints
fn apply_motor_forces(
    mut query: Query<(&mut ImpulseJoint, &Motor)>,
) {
    for (mut joint, motor) in query.iter_mut() {
        if let Some(revolute) = joint.data.as_revolute_mut() {
            revolute.set_motor_position(motor.target_angle, motor.stiffness, motor.damping);
            revolute.set_motor_max_force(motor.max_torque);
        }
    }
}

// Simple balance controller: Tries to keep the torso upright by adjusting hip angles
fn balance_control(
    mut motor_query: Query<(&mut Motor, &DancerPart)>,
    dancer_query: Query<&Transform, With<Dancer>>,
) {
    if let Ok(torso_transform) = dancer_query.get_single() {
        let torso_angle = torso_transform.rotation.to_euler(EulerRot::XYZ).2;

        // P-Controller gain
        let k_p = 1.5;

        // If torso is tilting left (positive angle), we need to rotate legs left (positive) relative to torso to keep them vertical?
        // No, if Torso tilts +10deg, legs naturally rotate +10deg with it.
        // To keep legs vertical (0deg global), local angle must be -10deg.
        // So target_angle -= torso_angle.

        // But we also want to correct the torso.
        // If torso is +10deg, we want to push it back.
        // Pushing against the ground:
        // If we rotate hips *clockwise* (negative), the reaction torque on torso is *counter-clockwise* (positive).
        // Wait, if torso is +10 (left), we want -torque (right).
        // So we want to rotate hips *counter-clockwise* (positive)?

        // Let's just try to keep legs vertical first.
        // Leg Global = Leg Local + Torso Global.
        // We want Leg Global = Down (-PI/2).
        // Local = -PI/2 - Torso.

        for (mut motor, part) in motor_query.iter_mut() {
            if part.name == "L_Thigh" || part.name == "R_Thigh" {
                // Base pose for walking/standing
                let base_angle = if part.name == "L_Thigh" { -0.2 } else { 0.2 }; // Slight spread

                // Compensate for torso rotation to keep legs down
                motor.target_angle = base_angle - torso_angle;
            } else if part.name.contains("Arm") {
                // Arms hang down
                let base_angle = -1.57;
                motor.target_angle = base_angle - torso_angle;
            }
        }
    }
}
