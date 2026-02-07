use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AntState {
    Foraging,
    Bridging,
}

#[derive(Clone)]
pub struct Ant {
    pub pos: Vec2,
    pub state: AntState,
    pub velocity: Vec2,
}

impl Ant {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            state: AntState::Foraging,
            velocity: Vec2::ZERO,
        }
    }

    pub fn update<F, G>(&mut self, get_height: F, get_gradient: G, dt: f32)
    where
        F: Fn(Vec2) -> f32,
        G: Fn(Vec2) -> Vec2,
    {
        match self.state {
            AntState::Bridging => {
                // Bridging ants are static.
                // Maybe they decay back to foraging if crowding reduces?
                // For now, permanent bridges.
            }
            AntState::Foraging => {
                // 1. Get Gradient
                let grad = get_gradient(self.pos);
                let height = get_height(self.pos);

                // 2. Physics (Gradient Descent with Momentum)
                // We want to go DOWNHILL, so we go against the gradient (which points uphill).
                // However, our `gradient` function returns direction of ascent.
                // So force = -grad.

                let force = -grad;

                // If gradient is very small, we might be at a minimum (local or global).
                // Or a maximum.
                // If we are at a local minimum (valley), we want to bridge it if it's not deep enough.

                // Heuristic: If velocity is low and gradient is low, we are stuck.
                // Check if we should bridge.
                // "Deep enough" is hard to define generically.
                // Let's say if we are stuck and height > -5.0 (arbitrary threshold for "not the abyss").

                let speed = self.velocity.length();
                let grad_mag = force.length();

                if speed < 0.1 && grad_mag < 0.1 {
                    // We are stuck.
                    // Are we at the bottom?
                    // Let's assume the "Goal" is very low (e.g. -10 or lower).
                    // Most functions have minima around 0 or lower.
                    // Rastrigin (inverted) has global max at 0? No, I implemented it as returning negative values.
                    // Let's check `landscape.rs`:
                    // Rastrigin: -val * 0.2. Min of val is 0. So max of result is 0. Valleys are negative.
                    // Ackley: -val. Min val is 0. So max result is 0.
                    // Rosenbrock: -val. Max result 0.

                    // So we are climbing UP to 0?
                    // Wait, `gradient` points to steepest ASCENT.
                    // If we want to find the MINIMUM of the original function, we should go DOWNHILL.
                    // But in `landscape.rs`, `value` returns "Higher values represent hills".
                    // If I inverted the standard functions (which are minimization problems) to be negative, then the "Global Minimum" (original) is the "Global Maximum" (0) in my heightmap.
                    // And `gradient` points UPHILL towards 0.

                    // So ants should CLIMB (Gradient Ascent).
                    // So force = grad.

                    // Let's re-read `landscape.rs`.
                    // "Higher values represent hills (light), lower values represent valleys (water)."
                    // Rastrigin: returns `-val * 0.2`. Original Rastrigin has global min at 0. So `-val` has global max at 0.
                    // So the "Goal" is the Peak (0).
                    // So ants should perform Gradient Ascent (move towards light/hills).

                    // So if ants are "stuck" in a local maximum (which is actually a local minimum of the original function if it has many),
                    // wait. Rastrigin has many local minima.
                    // Inverted, it has many local maxima (hills). One global maximum at 0.
                    // Ants climbing will get stuck on a local peak.
                    // We want them to "Bridge" the "Gap" between peaks?
                    // No, usually bridging fills a valley.

                    // Let's flip the metaphor.
                    // Ants want to find the DEEP VALLEY (Global Minimum).
                    // The "Landscape" should return positive values for high cost, negative/zero for low cost.
                    // `landscape.rs` says: "Higher values represent hills".
                    // If we want to minimize cost, we go DOWNHILL (to valleys).
                    // `gradient` points UPHILL. So force = -grad.

                    // Let's check `gradient-garden` behavior.
                    // `optimizer.rs` probably does ascent or descent.
                    // In `main.rs`, `plant.update`:
                    // `optimizer.compute_step`.

                    // If I stick to `biomimetic-bridge` metaphor: Ants build bridges over GAPS.
                    // A "Gap" is usually a void or a deep place.
                    // But here, we want to bridge "Bad Places" (High Cost / Hills) to cross them?
                    // Or fill "Local Minima" (Valleys) so we can walk across to a deeper valley?

                    // "Filling Local Minima" is the standard "Basin Hopping" / "Flooding" approach.
                    // So we want to go DOWN.
                    // Goal: Deepest Valley.
                    // Obstacle: Shallow Valleys (Local Minima).
                    // Solution: Fill Shallow Valleys with bodies until we can walk out of them and find a deeper one.

                    // So:
                    // 1. Move Downhill (Force = -Grad).
                    // 2. If stuck (Grad ~ 0, Velocity ~ 0) AND Height > Global_Min_Target (e.g. -20.0).
                    // 3. Become Bridge.
                    // 4. Bridge Effect: Raises terrain height locally.
                    //    Since we want to go DOWNHILL, raising terrain makes it a "Hill", so we roll off it?
                    //    Yes! If we fill a hole, it becomes a flat surface or a hill, so we don't get stuck there.

                    let target_force = -grad * 10.0; // Acceleration

                    // Apply force
                    self.velocity += target_force * dt;

                    // Friction
                    self.velocity *= 0.9;

                    // Update pos
                    self.pos += self.velocity * dt;

                    // Random noise (Brownian motion)
                    self.pos.x += macroquad::rand::gen_range(-0.05, 0.05);
                    self.pos.y += macroquad::rand::gen_range(-0.05, 0.05);

                    // Check for bridging
                    // If gradient is small (flat) and we are not moving much.
                    if grad.length() < 0.1 && self.velocity.length() < 0.1 {
                        // Are we deep enough?
                        // Let's say anything below -15 is "Good Enough" (Global Min).
                        // If we are at -5, we are in a local trap.
                        if height > -15.0 {
                            // 1% chance to start bridging to avoid instant solidification
                            if macroquad::rand::gen_range(0.0, 1.0) < 0.01 {
                                self.state = AntState::Bridging;
                            }
                        }
                    }
                }
            }
        }
    }
}
