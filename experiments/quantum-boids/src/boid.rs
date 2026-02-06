use rand::Rng;
use crate::qubit::Qubit;
use std::f64::consts::PI;

const MAX_FORCE: f64 = 0.05;
const MAX_SPEED: f64 = 0.5;
const PERCEPTION_RADIUS: f64 = 10.0;
pub const ENTANGLEMENT_RADIUS: f64 = 5.0;

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub qubit: Qubit,
    pub entangled_partner: Option<usize>, // Index of partner
    pub id: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64, id: usize) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let speed = rng.gen_range(0.1..MAX_SPEED);

        let mut qubit = Qubit::new();
        // Initialize with random quantum state (Hadamard + random phase)
        qubit.h();
        if rng.gen_bool(0.5) {
            qubit.z();
        }

        Self {
            position: (x, y),
            velocity: (speed * angle.cos(), speed * angle.sin()),
            acceleration: (0.0, 0.0),
            qubit,
            entangled_partner: None,
            id,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        // Quantum behavior: phase affects max speed
        let phase = self.qubit.phase();
        // Map phase (-PI to PI) to speed multiplier (0.5 to 1.5)
        let speed_mod = 1.0 + (phase / PI) * 0.5;
        let current_max_speed = MAX_SPEED * speed_mod;

        // Apply acceleration
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;

        // Limit speed
        let speed = (self.velocity.0.powi(2) + self.velocity.1.powi(2)).sqrt();
        if speed > current_max_speed {
            let scale = current_max_speed / speed;
            self.velocity.0 *= scale;
            self.velocity.1 *= scale;
        }

        // Update position
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Wrap around (toroidal world)
        if self.position.0 < 0.0 { self.position.0 += width; }
        if self.position.0 > width { self.position.0 -= width; }
        if self.position.1 < 0.0 { self.position.1 += height; }
        if self.position.1 > height { self.position.1 -= height; }

        // Reset acceleration
        self.acceleration = (0.0, 0.0);
    }

    pub fn apply_force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0;
        self.acceleration.1 += force.1;
    }

    // Flocking behaviors
    pub fn flock(&mut self, boids: &[Boid]) {
        let mut alignment = (0.0, 0.0);
        let mut cohesion = (0.0, 0.0);
        let mut separation = (0.0, 0.0);
        let mut total = 0;

        for other in boids {
            if other.id == self.id { continue; }

            let d = distance(self.position, other.position);

            if d < PERCEPTION_RADIUS && d > 0.0 {
                // Alignment
                alignment.0 += other.velocity.0;
                alignment.1 += other.velocity.1;

                // Cohesion
                cohesion.0 += other.position.0;
                cohesion.1 += other.position.1;

                // Separation
                let diff = (
                    (self.position.0 - other.position.0) / d,
                    (self.position.1 - other.position.1) / d,
                );
                separation.0 += diff.0;
                separation.1 += diff.1;

                total += 1;
            }
        }

        if total > 0 {
            let count = total as f64;

            // Average alignment
            alignment.0 /= count;
            alignment.1 /= count;
            alignment = set_mag(alignment, MAX_SPEED);
            alignment = sub(alignment, self.velocity);
            alignment = limit(alignment, MAX_FORCE);

            // Average cohesion
            cohesion.0 /= count;
            cohesion.1 /= count;
            cohesion = sub(cohesion, self.position);
            cohesion = set_mag(cohesion, MAX_SPEED);
            cohesion = sub(cohesion, self.velocity);
            cohesion = limit(cohesion, MAX_FORCE);

            // Average separation
            separation.0 /= count;
            separation.1 /= count;
            separation = set_mag(separation, MAX_SPEED);
            separation = sub(separation, self.velocity);
            separation = limit(separation, MAX_FORCE);
        }

        // Quantum Weighting
        // If Prob(|1>) is high, prefer Separation (Scatter)
        // If Prob(|0>) is high, prefer Cohesion (Gather)
        let p_one = self.qubit.prob_one();

        let align_w = 1.0;
        let coh_w = 1.0 + (1.0 - p_one); // More cohesion if closer to |0>
        let sep_w = 1.0 + p_one * 2.0;   // More separation if closer to |1>

        self.apply_force((alignment.0 * align_w, alignment.1 * align_w));
        self.apply_force((cohesion.0 * coh_w, cohesion.1 * coh_w));
        self.apply_force((separation.0 * sep_w, separation.1 * sep_w));
    }
}

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn set_mag(v: (f64, f64), mag: f64) -> (f64, f64) {
    let len = (v.0.powi(2) + v.1.powi(2)).sqrt();
    if len == 0.0 { return (0.0, 0.0); }
    (v.0 * mag / len, v.1 * mag / len)
}

fn limit(v: (f64, f64), max: f64) -> (f64, f64) {
    let len_sq = v.0.powi(2) + v.1.powi(2);
    if len_sq > max.powi(2) {
        set_mag(v, max)
    } else {
        v
    }
}

fn sub(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    (a.0 - b.0, a.1 - b.1)
}
