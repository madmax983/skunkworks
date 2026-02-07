use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::{PI, TAU};
use tui_shared::math::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Strategy {
    Bull,
    Bear,
}

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub coupling_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub price_weight: f64, // New: How strongly they follow the price
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
    pub char_representation: char,
    pub strategy: Strategy,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let strategy = if rng.gen_bool(0.5) {
            Strategy::Bull
        } else {
            Strategy::Bear
        };

        let color = match strategy {
            Strategy::Bull => Color::Green,
            Strategy::Bear => Color::Red,
        };

        Self {
            max_speed: rng.gen_range(0.8..1.5),
            max_force: rng.gen_range(0.05..0.15),
            view_radius: rng.gen_range(10.0..20.0),
            coupling_radius: rng.gen_range(15.0..30.0),
            separation_weight: rng.gen_range(1.5..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            price_weight: rng.gen_range(0.5..1.0), // Seek price
            natural_freq: 0.005 + rng.r#gen::<f64>() * 0.02,
            coupling_strength: 0.005,
            color,
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
            strategy,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: Dna,
    pub phase: f64,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::random();

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            phase: rng.r#gen::<f64>(),
            flash_timer: 0,
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::zero();

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }

        // Bounce off top/bottom
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.velocity.y *= -1.0;
        }
        if self.position.y >= height {
            self.position.y = height - 0.1;
            self.velocity.y *= -1.0;
        }
    }

    pub fn update_flash(&mut self) {
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 5;
        }
    }
}

pub struct Flock {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl Flock {
    pub fn new(width: f64, height: f64, count: usize) -> Self {
        let mut boids = Vec::new();
        for _ in 0..count {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        Self {
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self, target_price_y: f64) {
        let count = self.boids.len();
        let mut physics_forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        for (i, nudge_out) in phase_nudges.iter_mut().enumerate() {
            let mut separation = Vec2::zero();
            let mut alignment = Vec2::zero();
            let mut cohesion = Vec2::zero();
            let mut price_seek = Vec2::zero();

            let mut sep_count = 0;
            let mut ali_count = 0;
            let mut coh_count = 0;

            let mut nudge = 0.0;

            let p1 = self.boids[i].position;
            let v1 = self.boids[i].velocity;
            let dna = &self.boids[i].dna;

            // Target Force
            let target_y = match dna.strategy {
                Strategy::Bull => target_price_y + 5.0, // Above (Higher Y)
                Strategy::Bear => target_price_y - 5.0, // Below (Lower Y)
            };

            // Just seek the Y coordinate, keep X loose (drift)
            // Or target a specific point? Let's target (p1.x + lookahead, target_y)
            let lookahead = 20.0;
            let target = Vec2::new(p1.x + lookahead, target_y);

            let mut desired = target - p1;
            // Wrap X handling for target not needed if we target local X

            if desired.magnitude_squared() > 0.0 {
                desired = desired.normalize() * dna.max_speed;
                let steer = desired - v1;
                price_seek = steer.limit(dna.max_force);
            }

            let view_radius_sq = dna.view_radius.powi(2);
            let coupling_radius_sq = dna.coupling_radius.powi(2);
            let separation_radius_sq = (dna.view_radius / 2.0).powi(2);

            for j in 0..count {
                if i == j {
                    continue;
                }

                let b2 = &self.boids[j];
                let d_sq = p1.distance_squared(b2.position);

                if d_sq == 0.0 {
                    continue;
                }

                if d_sq < view_radius_sq {
                    if d_sq < separation_radius_sq {
                        let diff = p1 - b2.position;
                        separation += diff / d_sq;
                        sep_count += 1;
                    }
                    alignment += b2.velocity;
                    ali_count += 1;
                    cohesion += b2.position;
                    coh_count += 1;
                }

                if d_sq < coupling_radius_sq && b2.flash_timer == 5 {
                    nudge += dna.coupling_strength;
                }
            }

            let mut total_force = Vec2::zero();

            if sep_count > 0 {
                if separation.magnitude_squared() > 0.0 {
                    separation = separation.normalize() * dna.max_speed;
                    separation -= v1;
                    separation = separation.limit(dna.max_force);
                    total_force += separation * dna.separation_weight;
                }
            }

            if ali_count > 0 {
                alignment /= ali_count as f64;
                if alignment.magnitude_squared() > 0.0 {
                    alignment = alignment.normalize() * dna.max_speed;
                    alignment -= v1;
                    alignment = alignment.limit(dna.max_force);
                    total_force += alignment * dna.alignment_weight;
                }
            }

            if coh_count > 0 {
                cohesion /= coh_count as f64;
                let mut desired = cohesion - p1;
                if desired.magnitude_squared() > 0.0 {
                    desired = desired.normalize() * dna.max_speed;
                    desired -= v1;
                    desired = desired.limit(dna.max_force);
                    total_force += desired * dna.cohesion_weight;
                }
            }

            total_force += price_seek * dna.price_weight;

            physics_forces.push(total_force);
            *nudge_out = nudge;
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(physics_forces[i]);
            boid.update_physics(self.width, self.height);
            boid.phase += boid.dna.natural_freq + phase_nudges[i];
            boid.update_flash();
        }
    }

    pub fn synchronization_index(&self) -> f64 {
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;
        for b in &self.boids {
            let theta = b.phase * 2.0 * PI;
            sum_sin += theta.sin();
            sum_cos += theta.cos();
        }
        let n = self.boids.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}
