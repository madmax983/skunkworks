use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use poincare_disk::{hyperbolic_dist, mobius_add, mobius_sub, Point};

#[derive(Clone, Copy)]
pub struct FlockingParams {
    pub view_radius: f64,
    pub separation_radius: f64,
    pub max_speed: f64,
    pub max_force: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
}

impl Default for FlockingParams {
    fn default() -> Self {
        Self {
            view_radius: 1.5,
            separation_radius: 0.3,
            max_speed: 0.02,
            max_force: 0.005,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        }
    }
}

#[derive(Clone)] // Added Clone
pub struct Boid {
    pub vm: ChimeraVM,
    pub pos: Point,
    pub vel: Point,
    pub color: Color,
    pub params: FlockingParams,
}

impl Boid {
    pub fn new(dna: Dna) -> Self {
        let mut rng = ::rand::thread_rng();
        // Start near origin
        let re = rng.gen_range(-0.5..0.5);
        let im = rng.gen_range(-0.5..0.5);
        let pos = Point::new(re, im);

        // Random velocity
        let vel_re = rng.gen_range(-0.01..0.01);
        let vel_im = rng.gen_range(-0.01..0.01);
        let vel = Point::new(vel_re, vel_im);

        let r = rng.gen_range(0.5..1.0);
        let g = rng.gen_range(0.5..1.0);
        let b = rng.gen_range(0.5..1.0);
        let color = Color::new(r, g, b, 1.0);

        Self {
            vm: ChimeraVM::new(dna),
            pos,
            vel,
            color,
            params: FlockingParams::default(),
        }
    }

    pub fn update_physics(&mut self) {
        // Limit speed (velocity magnitude)
        let speed = self.vel.norm();
        if speed > self.params.max_speed {
            self.vel = (self.vel / speed) * self.params.max_speed;
        }

        // Apply movement
        self.pos = mobius_add(self.pos, self.vel);

        // Boundary check
        if self.pos.norm_sqr() > 0.98 {
            self.vel = -self.vel;
            self.pos = self.pos * 0.95;
        }

        // Friction
        self.vel = self.vel * 0.99;
    }

    pub fn calculate_flocking(&mut self, neighbors: &[Boid]) {
        let mut separation = Point::new(0.0, 0.0);
        let mut alignment = Point::new(0.0, 0.0);
        let mut cohesion = Point::new(0.0, 0.0);
        let mut count = 0;

        for other in neighbors {
            // Distance check
            let dist = hyperbolic_dist(self.pos, other.pos);
            if dist > 1e-6 && dist < self.params.view_radius {
                // Separation
                if dist < self.params.separation_radius {
                    let relative_pos = mobius_sub(other.pos, self.pos);
                    let diff = -relative_pos; // Away from neighbor
                    separation = separation + (diff / (dist * dist));
                }

                // Alignment
                alignment = alignment + other.vel;

                // Cohesion
                let relative_pos = mobius_sub(other.pos, self.pos);
                cohesion = cohesion + relative_pos;

                count += 1;
            }
        }

        if count > 0 {
            // Average and Weight
            if separation.norm() > 0.0 {
                separation = (separation / separation.norm()) * self.params.max_force;
            }
            if alignment.norm() > 0.0 {
                alignment = (alignment / alignment.norm()) * self.params.max_force;
            }

            // Cohesion
            cohesion = cohesion / (count as f64);
            // Steering towards target
            if cohesion.norm() > 0.0 {
                let desired = (cohesion / cohesion.norm()) * self.params.max_speed;
                let steer = desired - self.vel;
                if steer.norm() > self.params.max_force {
                    cohesion = (steer / steer.norm()) * self.params.max_force;
                } else {
                    cohesion = steer;
                }
            }
        }

        // Apply forces
        let total_force = separation * self.params.separation_weight
            + alignment * self.params.alignment_weight
            + cohesion * self.params.cohesion_weight;

        self.vel = self.vel + total_force;
    }
}
