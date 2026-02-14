use crate::qubit::Qubit;
use flocking::PhysicsState;
use locus::Vec2;
use rand::Rng;
use std::f64::consts::PI;

const MAX_SPEED: f64 = 0.5;
pub const PERCEPTION_RADIUS: f64 = 10.0;
pub const ENTANGLEMENT_RADIUS: f64 = 5.0;

#[derive(Clone, Debug)]
pub struct Boid {
    pub physics: PhysicsState,
    pub qubit: Qubit,
    pub entangled_partner: Option<usize>,
    pub current_max_speed: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let speed = rng.gen_range(0.1..MAX_SPEED);

        let mut physics = PhysicsState::new(x, y);
        physics.velocity = Vec2::new(speed * angle.cos(), speed * angle.sin());

        let mut qubit = Qubit::new();
        // Initialize with random quantum state (Hadamard + random phase)
        qubit.h();
        if rng.gen_bool(0.5) {
            qubit.z();
        }

        Self {
            physics,
            qubit,
            entangled_partner: None,
            current_max_speed: MAX_SPEED,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.physics.position
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.physics.apply_force(force);
    }

    pub fn update(&mut self, width: f64, height: f64) {
        // Quantum behavior: phase affects max speed
        let phase = self.qubit.phase();
        // Map phase (-PI to PI) to speed multiplier (0.5 to 1.5)
        let speed_mod = 1.0 + (phase / PI) * 0.5;
        self.current_max_speed = MAX_SPEED * speed_mod;

        self.physics.update(self.current_max_speed);

        // Wrap around (toroidal world)
        if self.physics.position.x < 0.0 {
            self.physics.position.x += width;
        }
        if self.physics.position.x > width {
            self.physics.position.x -= width;
        }
        if self.physics.position.y < 0.0 {
            self.physics.position.y += height;
        }
        if self.physics.position.y > height {
            self.physics.position.y -= height;
        }
    }
}
