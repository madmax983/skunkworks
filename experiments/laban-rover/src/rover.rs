use crate::laban::LabanEffort;
use locus::Vec2;
use rand::Rng;
use std::f64::consts::PI;

pub struct Rover {
    pub pos: Vec2,
    pub vel: Vec2,
    pub angle: f64, // Radians, 0 pointing Right (positive X)
    pub max_speed: f64,
}

impl Rover {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            angle: 0.0,
            max_speed: 2.0,
        }
    }

    pub fn update(&mut self, effort: &LabanEffort) {
        // Laban Weight: 0.0 (Strong/Heavy) -> 1.0 (Light/Gentle)
        // Heavy objects have high friction/drag. Light objects glide.
        // Friction factor: 0.0 = stops instantly, 1.0 = no friction
        // Heavy (0.0) -> 0.85
        // Light (1.0) -> 0.98
        let friction = 0.85 + (effort.weight as f64 * 0.13);

        self.pos += self.vel;
        self.vel *= friction;

        // Stop completely if very slow
        if self.vel.magnitude_squared() < 0.001 {
            self.vel = Vec2::zero();
        }
    }

    pub fn thrust(&mut self, amount: f64, effort: &LabanEffort) {
        // Laban Time: 0.0 (Sudden) -> 1.0 (Sustained)
        // Sudden movements have high initial impulse (acceleration) but maybe lower top speed?
        // Sustained movements build up slowly.

        let accel_mult = 1.0 + (1.0 - effort.time as f64) * 1.5; // 1.0 to 2.5x burst

        // Laban Space: 0.0 (Direct) -> 1.0 (Indirect)
        // Indirect movements wander. Direct movements are precise.
        let mut jitter = 0.0;
        if effort.space > 0.2 {
            let mut rng = rand::thread_rng();
            // Jitter up to +/- 0.5 radians if totally Indirect
            jitter = rng.gen_range(-0.5..0.5) * effort.space as f64;
        }

        let effective_angle = self.angle + jitter;
        let thrust_vec =
            Vec2::new(effective_angle.cos(), effective_angle.sin()) * amount * accel_mult;

        self.vel += thrust_vec;

        // Laban Flow: 0.0 (Bound) -> 1.0 (Free)
        // This is harder to map to physics directly without control input state,
        // but we can map it to Max Speed cap.
        // Free flow allows going faster?

        let flow_mod = 0.8 + (effort.flow as f64 * 0.4); // 0.8 to 1.2
        let time_mod = 0.5 + effort.time as f64; // 0.5 (Sudden) to 1.5 (Sustained)

        let current_max_speed = self.max_speed * flow_mod * time_mod;

        if self.vel.magnitude() > current_max_speed {
            self.vel = self.vel.normalize() * current_max_speed;
        }
    }

    pub fn rotate(&mut self, amount: f64, effort: &LabanEffort) {
        // Space (Direct vs Indirect) affects turning speed too?
        // Direct = Sharp turns. Indirect = Lazy turns.
        // Actually, let's map it inversely. Direct = Responsive.

        let responsiveness = 0.5 + (1.0 - effort.space as f64) * 0.5; // 0.5 to 1.0
        self.angle += amount * responsiveness;
        self.angle %= 2.0 * PI;
    }
}
