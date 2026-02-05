use crate::gait::calculate_gait;
use crate::terrain::{Terrain, NODE_GAP, NODE_WIDTH, START_X};
use bevy::prelude::*;

#[derive(Component)]
pub struct Walker {
    pub hip_pos: Vec2,
    pub thigh_len: f32,
    pub shin_len: f32,

    // Gait state
    pub timer: f32,
    pub is_left_swing: bool,
    pub stride_start_x: f32, // X pos where current stride started

    // IK Targets (World Space)
    pub left_foot_target: Vec2,
    pub right_foot_target: Vec2,

    // Previous targets for interpolation
    pub left_foot_start: Vec2,
    pub right_foot_start: Vec2,

    // Visualization
    pub left_knee: Vec2,
    pub right_knee: Vec2,
    pub left_foot: Vec2,
    pub right_foot: Vec2,
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            hip_pos: Vec2::new(START_X, 0.0),
            thigh_len: 40.0,
            shin_len: 40.0,
            timer: 0.0,
            is_left_swing: true,
            stride_start_x: START_X,
            left_foot_target: Vec2::new(START_X, -100.0),
            right_foot_target: Vec2::new(START_X, -100.0),
            left_foot_start: Vec2::new(START_X - 20.0, -100.0),
            right_foot_start: Vec2::new(START_X, -100.0),
            left_knee: Vec2::ZERO,
            right_knee: Vec2::ZERO,
            left_foot: Vec2::ZERO,
            right_foot: Vec2::ZERO,
        }
    }
}

pub fn spawn_walker(mut commands: Commands) {
    commands.spawn(Walker::default());
}

pub fn walker_system(mut query: Query<&mut Walker>, terrain: Res<Terrain>, time: Res<Time>) {
    let dt = time.delta_seconds();

    for mut walker in &mut query {
        // Calculate current node index based on hip position
        let stride_width = NODE_WIDTH + NODE_GAP;
        let relative_x = walker.hip_pos.x - START_X;
        let idx = (relative_x / stride_width).floor() as usize;
        let current_node_idx = idx.clamp(0, terrain.nodes.len().saturating_sub(1));

        let gait = if !terrain.nodes.is_empty() {
            calculate_gait(&terrain.nodes[current_node_idx])
        } else {
            crate::gait::GaitParameters {
                speed: 50.0,
                step_height: 20.0,
                bounce: 5.0,
            }
        };

        // Move Hip
        walker.hip_pos.x += gait.speed * dt;

        // Bobbing
        walker.hip_pos.y = -60.0 + (walker.timer * 10.0).sin() * gait.bounce;

        // Gait Cycle
        // Swing phase duration approx 0.5s
        let swing_duration = 0.5;
        walker.timer += dt;

        // Stride is roughly based on speed * duration, but let's keep it visually matched to nodes?
        // Actually, let's make stride proportional to speed.
        let stride_len = gait.speed * swing_duration * 1.5;

        // Ground Y lookup
        let ground_y = if !terrain.nodes.is_empty() {
            match terrain.nodes[current_node_idx].file_type {
                crate::terrain::FileType::Directory => -50.0,
                crate::terrain::FileType::File => -100.0,
            }
        } else {
            -100.0
        };

        if walker.timer > swing_duration {
            // Swap legs
            walker.is_left_swing = !walker.is_left_swing;
            walker.timer = 0.0;

            if walker.is_left_swing {
                walker.left_foot_start = walker.left_foot_target;
            } else {
                walker.right_foot_start = walker.right_foot_target;
            }
        }

        let progress = (walker.timer / swing_duration).clamp(0.0, 1.0);

        // Swing Logic
        if walker.is_left_swing {
            // Left moves
            let target_x = walker.hip_pos.x + stride_len * 0.5; // Aim ahead of hip
            let start_x = walker.left_foot_start.x;

            let current_x = start_x + (target_x - start_x) * progress;
            let current_y = ground_y + (progress * std::f32::consts::PI).sin() * gait.step_height;

            walker.left_foot_target = Vec2::new(current_x, current_y);
        } else {
            // Right moves
            let target_x = walker.hip_pos.x + stride_len * 0.5;
            let start_x = walker.right_foot_start.x;

            let current_x = start_x + (target_x - start_x) * progress;
            let current_y = ground_y + (progress * std::f32::consts::PI).sin() * gait.step_height;

            walker.right_foot_target = Vec2::new(current_x, current_y);
        }

        // Solve IK
        if let Some((knee, foot)) = solve_leg(
            walker.hip_pos,
            walker.left_foot_target,
            walker.thigh_len,
            walker.shin_len,
        ) {
            walker.left_knee = knee;
            walker.left_foot = foot;
        } else {
            walker.left_foot = walker.left_foot_target; // Snap
        }

        if let Some((knee, foot)) = solve_leg(
            walker.hip_pos,
            walker.right_foot_target,
            walker.thigh_len,
            walker.shin_len,
        ) {
            walker.right_knee = knee;
            walker.right_foot = foot;
        } else {
            walker.right_foot = walker.right_foot_target;
        }
    }
}

/// Solves 2-bone IK. Returns (Knee Position, Foot Position).
pub fn solve_leg(hip: Vec2, target: Vec2, thigh_len: f32, shin_len: f32) -> Option<(Vec2, Vec2)> {
    let to_target = target - hip;
    let dist = to_target.length();

    if dist > thigh_len + shin_len {
        return None;
    }

    if dist < 0.001 {
        return Some((hip + Vec2::new(thigh_len, 0.0), target));
    }

    let cos_alpha =
        (thigh_len * thigh_len + dist * dist - shin_len * shin_len) / (2.0 * thigh_len * dist);
    let cos_alpha = cos_alpha.clamp(-1.0, 1.0);
    let alpha = cos_alpha.acos();

    let theta = to_target.y.atan2(to_target.x);

    // Knee forward bend (positive relative rotation)
    let knee_angle = theta + alpha;

    let knee_offset = Vec2::new(knee_angle.cos(), knee_angle.sin()) * thigh_len;
    let knee = hip + knee_offset;

    Some((knee, target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_leg_reach() {
        let hip = Vec2::new(0.0, 100.0);
        let target = Vec2::new(0.0, 0.0);
        let thigh = 50.0;
        let shin = 50.0;

        let result = solve_leg(hip, target, thigh, shin);
        assert!(result.is_some(), "Leg should reach the ground");

        let (knee, foot) = result.unwrap();
        assert!(foot.distance(target) < 0.001, "Foot should be at target");
        assert!(
            (knee.distance(hip) - thigh).abs() < 0.001,
            "Thigh length preserved"
        );
        assert!(
            (foot.distance(knee) - shin).abs() < 0.001,
            "Shin length preserved"
        );
    }
}
