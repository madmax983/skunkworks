use crate::laban::Director;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Actor {
    pub velocity: Vec2,
    pub target: Vec2,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            velocity: Vec2::ZERO,
            target: Vec2::ZERO,
        }
    }
}

pub fn actor_move_system(
    mut query: Query<(&mut Transform, &mut Actor)>,
    director: Res<Director>,
    time: Res<Time>,
    window_query: Query<&Window>,
) {
    let effort = &director.current_effort;
    let window = window_query.single();
    let win_size = Vec2::new(window.width(), window.height());

    for (mut transform, mut actor) in query.iter_mut() {
        let pos = transform.translation.truncate();

        // 1. Pick a target if close
        if pos.distance(actor.target) < 10.0 {
            let mut rng = rand::thread_rng();
            // Random target within window bounds (centered at 0,0)
            let half_w = win_size.x / 2.0 - 50.0;
            let half_h = win_size.y / 2.0 - 50.0;
            actor.target = Vec2::new(
                rng.gen_range(-half_w..half_w),
                rng.gen_range(-half_h..half_h),
            );
        }

        // 2. Calculate desired velocity
        let to_target = actor.target - pos;
        let dist = to_target.length();
        let _dir = if dist > 0.0 {
            to_target / dist
        } else {
            Vec2::ZERO
        };

        // 3. Apply Laban "Space" (Direct vs Indirect)
        // Indirect adds noise to the direction
        let noise_strength = effort.space * 2.0; // 0.0 to 2.0
        let time_now = time.elapsed_seconds();
        let noise =
            Vec2::new((time_now * 5.0).sin(), (time_now * 4.0).cos()) * noise_strength * 100.0;

        // Target point is effectively shifted by noise for Indirect movement
        let effective_target = actor.target + noise;
        let to_effective = effective_target - pos;
        let effective_dir = to_effective.normalize_or_zero();

        // 4. Apply Laban "Time" (Sudden vs Sustained)
        // Sudden = High Acceleration, snappy. Sustained = Low Accel, smooth.
        // Time 0.0 (Sudden) -> Accel 2000.0
        // Time 1.0 (Sustained) -> Accel 200.0
        let accel_mag = 2000.0 - (effort.time * 1800.0);
        let max_speed = 500.0; // Constant max speed? Or maybe Time affects this too?

        let target_vel = effective_dir * max_speed;

        // 5. Apply Laban "Weight" (Strong vs Light)
        // Strong = High drag/damping (stops precisely). Light = Low drag (springy/overshoot).
        // Weight 0.0 (Strong) -> Damping 10.0
        // Weight 1.0 (Light) -> Damping 1.0
        let damping = 10.0 - (effort.weight * 9.0);

        // Physics update
        let delta_v = target_vel - actor.velocity;
        let accel = delta_v * damping * time.delta_seconds(); // Proportional control

        // Clamp acceleration magnitude
        let accel_len = accel.length();
        let final_accel = if accel_len > accel_mag * time.delta_seconds() {
            accel.normalize() * accel_mag * time.delta_seconds()
        } else {
            accel
        };

        actor.velocity += final_accel;
        transform.translation += actor.velocity.extend(0.0) * time.delta_seconds();
    }
}
