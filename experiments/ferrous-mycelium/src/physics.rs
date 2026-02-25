use macroquad::prelude::*;

pub const G_REPULSE: f32 = 500.0; // Reduced for macroquad scale potentially
pub const CENTER_PULL: f32 = 0.05;
pub const DRAG: f32 = 0.95;
pub const MAG_STRENGTH: f32 = 1000.0;
pub const MAG_WRITE: f32 = 0.5;
pub const DECAY_RATE: f32 = 0.99;

pub fn calculate_magnetic_force(pos1: Vec2, pos2: Vec2, m1: f32, m2: f32) -> Vec2 {
    let delta = pos2 - pos1;
    let dist_sq = delta.length_squared().max(10.0);
    let dir = if delta.length_squared() > 0.0 {
        delta.normalize()
    } else {
        Vec2::ZERO
    };

    // Like poles repel, opposite attract.
    // m is 0.0 to 1.0. Neutral is 0.5.
    let mag1 = m1 - 0.5;
    let mag2 = m2 - 0.5;

    // If m1 and m2 have same sign (both North or both South), product is positive -> Repulsion.
    // If opposite signs, product is negative -> Attraction.
    let force_val = (mag1 * mag2 * MAG_STRENGTH) / dist_sq;

    // Force direction:
    // If Repulsion (positive force_val), we want to push AWAY.
    // If pos2 is the source, force on pos1 is -dir * force?
    // Wait. calculate_magnetic_force(me, other).
    // If I am pushed away from other, force is in direction (me - other) = -delta.
    // So -dir * force_val.

    -dir * force_val
}
