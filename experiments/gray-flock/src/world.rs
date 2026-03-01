use crate::boid::Boid;
use gray_scott::GrayScott;
use locus::Vec2;
use locus::flocking::{compute_force, FlockingParams};

pub struct World {
    pub gray_scott: GrayScott,
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let w_int = width as usize;
        let h_int = height as usize;
        let mut gray_scott = GrayScott::new(w_int, h_int);

        // Seed gray scott
        for y in (h_int / 2 - 5)..(h_int / 2 + 5) {
            for x in (w_int / 2 - 5)..(w_int / 2 + 5) {
                gray_scott.add_chemical(x, y, 1.0);
            }
        }

        let mut boids = Vec::new();
        // Spawn 200 boids in center
        for _ in 0..200 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }

        Self {
            gray_scott,
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        // 1. Update GrayScott
        // Parameters for "Spots": f=0.055, k=0.062, dt=1.0
        self.gray_scott.update(0.055, 0.062, 1.0);

        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let w_int = self.width as usize;
        let h_int = self.height as usize;

        // 2. Update Boids
        for i in 0..self.boids.len() {
            // Flocking Forces
            let flock_params = FlockingParams {
                view_radius: self.boids[i].view_radius,
                separation_radius: self.boids[i].view_radius * 0.5,
                max_speed: self.boids[i].max_speed,
                max_force: self.boids[i].max_force,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 0.8,
            };

            let flocking_force = compute_force(&positions, &velocities, i, &flock_params);

            // Chemotaxis Force: Sample gradient of chemical V
            let boid_pos = self.boids[i].position;
            let bx = boid_pos.x.floor() as isize;
            let by = boid_pos.y.floor() as isize;

            let mut best_dx = 0;
            let mut best_dy = 0;
            let mut max_v = -1.0;

            let gs_v = self.gray_scott.v();

            for dy in -2..=2 {
                for dx in -2..=2 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    // Wrap coordinates
                    let nx = (bx + dx).rem_euclid(w_int as isize) as usize;
                    let ny = (by + dy).rem_euclid(h_int as isize) as usize;

                    let idx = self.gray_scott.get_index(nx, ny);
                    if idx < gs_v.len() {
                        let val = gs_v[idx];
                        if val > max_v {
                            max_v = val;
                            best_dx = dx;
                            best_dy = dy;
                        }
                    }
                }
            }

            let mut chemo_force = Vec2::zero();
            if max_v > 0.1 {
                let target = Vec2::new(
                    boid_pos.x + best_dx as f64,
                    boid_pos.y + best_dy as f64,
                );

                // Steering: desired - velocity
                let desired = (target - boid_pos).normalize() * self.boids[i].max_speed;
                chemo_force = desired - self.boids[i].velocity;

                // Limit force
                if chemo_force.magnitude_squared() > self.boids[i].max_force * self.boids[i].max_force {
                    chemo_force = chemo_force.normalize() * self.boids[i].max_force;
                }
            }

            // Weight chemotaxis
            chemo_force *= 1.2;

            self.boids[i].apply_force(flocking_force + chemo_force);
        }

        // Update physics and deposit pheromone (V)
        for boid in &mut self.boids {
            boid.update_physics(self.width, self.height);

            let x = boid.position.x as usize;
            let y = boid.position.y as usize;
            self.gray_scott.add_chemical(x, y, 0.5); // Boids deposit V
        }
    }
}
