use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct DNA {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub color: Color,
    pub char_representation: char,
}

impl DNA {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.5..1.5),
            max_force: rng.gen_range(0.02..0.1),
            view_radius: rng.gen_range(5.0..15.0),
            separation_weight: rng.gen_range(1.0..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            color: Color::White,
            char_representation: '*',
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub dna: DNA,
    pub energy: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = DNA::random();

        Self {
            position: (x, y),
            velocity: (angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: (0.0, 0.0),
            dna,
            energy: 100.0,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;

        // Limit speed
        let speed = (self.velocity.0.powi(2) + self.velocity.1.powi(2)).sqrt();
        if speed > self.dna.max_speed {
            self.velocity.0 = (self.velocity.0 / speed) * self.dna.max_speed;
            self.velocity.1 = (self.velocity.1 / speed) * self.dna.max_speed;
        }

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Reset acceleration
        self.acceleration = (0.0, 0.0);

        // Wrap around edges
        if self.position.0 < 0.0 {
            self.position.0 += width;
        }
        if self.position.0 >= width {
            self.position.0 -= width;
        }
        if self.position.1 < 0.0 {
            self.position.1 += height;
        }
        if self.position.1 >= height {
            self.position.1 -= height;
        }

        // Decay energy
        self.energy -= 0.05;
    }

    pub fn apply_force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0;
        self.acceleration.1 += force.1;
    }

    // Returns the force vector to be applied
    pub fn calculate_flocking_force(&self, boids: &[Boid]) -> (f64, f64) {
        let mut separation = (0.0, 0.0);
        let mut alignment = (0.0, 0.0);
        let mut cohesion = (0.0, 0.0);

        let mut sep_count = 0;
        let mut ali_count = 0;
        let mut coh_count = 0;

        let view_radius_sq = self.dna.view_radius.powi(2);
        let separation_radius_sq = (self.dna.view_radius / 2.0).powi(2);

        for other in boids {
            let d_sq = distance_squared(self.position, other.position);

            // Avoid self (and exact overlaps, unlikely but possible)
            if d_sq == 0.0 {
                continue;
            }

            if d_sq < view_radius_sq {
                // Separation
                if d_sq < separation_radius_sq {
                    let diff = (
                        self.position.0 - other.position.0,
                        self.position.1 - other.position.1,
                    );
                    // Weight by distance squared inversely
                    separation.0 += diff.0 / d_sq;
                    separation.1 += diff.1 / d_sq;
                    sep_count += 1;
                }

                // Alignment
                alignment.0 += other.velocity.0;
                alignment.1 += other.velocity.1;
                ali_count += 1;

                // Cohesion
                cohesion.0 += other.position.0;
                cohesion.1 += other.position.1;
                coh_count += 1;
            }
        }

        let mut total_force = (0.0, 0.0);

        if sep_count > 0 {
            // Steering for separation
            let len = (separation.0.powi(2) + separation.1.powi(2)).sqrt();
            if len > 0.0 {
                separation.0 = (separation.0 / len) * self.dna.max_speed;
                separation.1 = (separation.1 / len) * self.dna.max_speed;
                separation.0 -= self.velocity.0;
                separation.1 -= self.velocity.1;
                separation = limit(separation, self.dna.max_force);

                total_force.0 += separation.0 * self.dna.separation_weight;
                total_force.1 += separation.1 * self.dna.separation_weight;
            }
        }

        if ali_count > 0 {
            alignment.0 /= ali_count as f64;
            alignment.1 /= ali_count as f64;
            let len = (alignment.0.powi(2) + alignment.1.powi(2)).sqrt();
            if len > 0.0 {
                alignment.0 = (alignment.0 / len) * self.dna.max_speed;
                alignment.1 = (alignment.1 / len) * self.dna.max_speed;
                alignment.0 -= self.velocity.0;
                alignment.1 -= self.velocity.1;
                alignment = limit(alignment, self.dna.max_force);

                total_force.0 += alignment.0 * self.dna.alignment_weight;
                total_force.1 += alignment.1 * self.dna.alignment_weight;
            }
        }

        if coh_count > 0 {
            cohesion.0 /= coh_count as f64;
            cohesion.1 /= coh_count as f64;

            // Cohesion is steering towards the target position
            let mut desired = (cohesion.0 - self.position.0, cohesion.1 - self.position.1);
            let len = (desired.0.powi(2) + desired.1.powi(2)).sqrt();
            if len > 0.0 {
                desired.0 = (desired.0 / len) * self.dna.max_speed;
                desired.1 = (desired.1 / len) * self.dna.max_speed;

                desired.0 -= self.velocity.0;
                desired.1 -= self.velocity.1;
                desired = limit(desired, self.dna.max_force);

                total_force.0 += desired.0 * self.dna.cohesion_weight;
                total_force.1 += desired.1 * self.dna.cohesion_weight;
            }
        }

        total_force
    }
}

pub fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    distance_squared(p1, p2).sqrt()
}

pub fn distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)
}

pub fn limit(vector: (f64, f64), max: f64) -> (f64, f64) {
    let len_sq = vector.0.powi(2) + vector.1.powi(2);
    if len_sq > max.powi(2) {
        let len = len_sq.sqrt();
        ((vector.0 / len) * max, (vector.1 / len) * max)
    } else {
        vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boid_movement() {
        let mut boid = Boid::new(0.0, 0.0);
        boid.dna.max_speed = 2.0; // Ensure speed isn't capped
        boid.velocity = (1.0, 0.0);
        boid.acceleration = (0.0, 0.0);
        boid.update(100.0, 100.0);

        assert!((boid.position.0 - 1.0).abs() < 1e-6);
        assert!((boid.position.1 - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_boundary_wrapping() {
        let mut boid = Boid::new(99.5, 50.0);
        boid.dna.max_speed = 2.0; // Ensure speed isn't capped
        boid.velocity = (1.0, 0.0);
        boid.update(100.0, 100.0);

        // Should wrap to 0.5
        assert!((boid.position.0 - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_distance_squared() {
        let p1 = (0.0, 0.0);
        let p2 = (3.0, 4.0);
        assert!((distance_squared(p1, p2) - 25.0).abs() < 1e-6);
        assert!((distance(p1, p2) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_flocking_force_zero_alone() {
        let boid = Boid::new(50.0, 50.0);
        let flock = vec![];
        let force = boid.calculate_flocking_force(&flock);
        assert_eq!(force, (0.0, 0.0));
    }

    #[test]
    fn test_flocking_force_separation() {
        // Create a boid at (50, 50)
        let mut boid1 = Boid::new(50.0, 50.0);
        boid1.velocity = (0.0, 0.0);
        boid1.dna.view_radius = 10.0;
        boid1.dna.max_speed = 2.0;
        boid1.dna.max_force = 0.1;
        boid1.dna.separation_weight = 1.0;
        boid1.dna.alignment_weight = 0.0; // Isolate separation
        boid1.dna.cohesion_weight = 0.0;

        // Create another boid very close (50.1, 50.0)
        let boid2 = Boid::new(50.1, 50.0);

        // This should trigger separation force pushing boid1 to the LEFT (negative X)
        // boid1 is at 50, boid2 is at 50.1. Diff is 50 - 50.1 = -0.1.
        let force = boid1.calculate_flocking_force(&[boid2]);

        assert!(
            force.0 < 0.0,
            "Force X should be negative (separation), got {}",
            force.0
        );
        assert_eq!(force.1, 0.0, "Force Y should be zero");
    }
}
