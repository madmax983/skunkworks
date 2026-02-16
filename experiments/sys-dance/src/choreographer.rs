use bevy::prelude::*;
use crate::laban::LabanState;
use crate::skeleton::IKChain;
use rand::Rng;

#[derive(Component)]
pub struct DancerLimb {
    pub side: f32, // -1.0 (Left) or 1.0 (Right)
    pub limb_type: LimbType,
    pub phase: f32,
    pub base_offset: Vec2,
}

#[derive(Clone, Copy, PartialEq)]
pub enum LimbType {
    Arm,
    Leg,
}

pub fn choreograph_system(
    laban: Res<LabanState>,
    time: Res<Time>,
    mut query: Query<(&mut IKChain, &mut DancerLimb)>,
) {
    let mut rng = rand::thread_rng();

    // Speed depends on Time effort (High Time = Slow, Low Time = Fast)
    // Laban.time is 0.0 (Fast) to 1.0 (Slow)
    let speed_mult = 2.0 + (1.0 - laban.time) * 5.0;
    let dt = time.delta_seconds() * speed_mult;

    for (mut chain, mut limb) in query.iter_mut() {
        limb.phase += dt;

        let mut target = limb.base_offset;

        match limb.limb_type {
            LimbType::Arm => {
                // Arms reach out based on Space (Indirect = Wide)
                // High Space = Wide arcs
                let amp = 30.0 + laban.space * 100.0;

                // Vertical movement based on Weight (Light = Up, Heavy = Down)
                // Laban.weight 0.0 (Light) to 1.0 (Heavy)
                let y_bias = (0.5 - laban.weight) * 50.0;

                // Oscillation
                // Add limb.side to phase to desync left/right if needed
                let x_osc = (limb.phase * 0.5).sin() * amp * limb.side;
                let y_osc = (limb.phase * 0.3).cos() * amp;

                target.x += x_osc;
                target.y += y_osc + y_bias;

                // Jitter for High Energy (Low Time)
                if laban.time < 0.2 {
                    target.x += rng.gen_range(-10.0..10.0);
                    target.y += rng.gen_range(-10.0..10.0);
                }
            }
            LimbType::Leg => {
                // Legs step
                // Heavy = Wide stance, slow steps
                // Light = Narrow stance, bouncy

                let stride = 20.0 + laban.weight * 40.0;
                let lift = 10.0 + (1.0 - laban.weight) * 20.0;

                // Simple walk cycle
                // Left and right are 180 deg out of phase
                let cycle_phase = limb.phase + if limb.side > 0.0 { 0.0 } else { std::f32::consts::PI };
                let cycle = cycle_phase.sin();

                target.x += cycle * stride;
                // Lift foot when moving forward (cycle > 0?)
                // Usually lift is |sin| or max(0, sin)
                if cycle > 0.0 {
                    target.y += cycle * lift;
                }
            }
        }

        // Smooth interpolation based on Flow
        // High Flow (Free) = High smoothing (low lerp factor)
        // Low Flow (Bound) = Low smoothing (high lerp factor, snappy)
        // Laban.flow 0.0 (Bound) to 1.0 (Free)
        // If Flow is 1.0 (Free), factor is 0.05 (Very smooth/slow to react)
        // If Flow is 0.0 (Bound), factor is 0.5 (Fast snap)
        let smooth_factor = 0.5 - (laban.flow * 0.45);
        chain.target = chain.target.lerp(target, smooth_factor);
    }
}
