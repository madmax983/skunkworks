use ferrous_core::Platter;
use locus::{
    flocking::{compute_force, FlockingParams},
    Vec2,
};
use rand::Rng;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
        }
    }
}

pub struct Magnet {
    pub pos: Vec2,
    pub strength: f64,
    pub polarity: bool, // true = North (Pull), false = South (Push/Complex)
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub magnets: Vec<Magnet>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter, // Replaces raw grid
    pub flocking_params: FlockingParams,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn particles
        for _ in 0..600 {
            let mut p = Particle::new(
                rng.gen_range(width * 0.2..width * 0.8),
                rng.gen_range(height * 0.1..height * 0.5),
            );
            p.vel = Vec2::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
            particles.push(p);
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        let flocking_params = FlockingParams {
            view_radius: 15.0,
            separation_radius: 5.0,
            max_speed: 30.0,
            max_force: 15.0,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
            flocking_params,
        }
    }

    pub fn add_magnet(&mut self, x: f64, y: f64, polarity: bool) {
        self.magnets.push(Magnet {
            pos: Vec2::new(x, y),
            strength: 2000.0,
            polarity,
        });
    }

    pub fn update(&mut self, dt: f64) {
        let damping = 0.96;

        // 1. Clear Grid (Platter)
        self.platter.clear();

        // 2. Populate Grid (Density)
        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            // Use accumulate to add density (unbounded)
            self.platter.accumulate(gx, gy, 1.0);
        }

        // 3. Extract positions and velocities for locus flocking
        let positions: Vec<Vec2> = self.particles.iter().map(|p| p.pos).collect();
        let velocities: Vec<Vec2> = self.particles.iter().map(|p| p.vel).collect();

        // 4. Update Particles
        for i in 0..self.particles.len() {
            let mut force = Vec2::zero();

            // Flocking Force
            let flocking_force = compute_force(&positions, &velocities, i, &self.flocking_params);
            force += flocking_force;

            // Magnetism
            for mag in &self.magnets {
                let delta = mag.pos - self.particles[i].pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (mag.strength / dist_sq).min(200.0);

                    if mag.polarity {
                        force += dir * mag_force; // Attract
                    } else {
                        force += dir * -mag_force; // Repel
                    }
                }
            }

            // Fluid Pressure (from Platter)
            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0
                && gx < (self.platter.width() - 1)
                && gy > 0
                && gy < (self.platter.height() - 1)
            {
                // Gradient
                let left = self.platter.get_magnetism(gx - 1, gy);
                let right = self.platter.get_magnetism(gx + 1, gy);
                let down = self.platter.get_magnetism(gx, gy - 1);
                let up = self.platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 50.0; // Push away from high density
                force += pressure_force;
            }

            self.particles[i].acc = force;
        }

        // Integrate
        for p in &mut self.particles {
            p.vel += p.acc * dt;
            p.vel *= damping;

            // Limit speed
            if p.vel.magnitude_squared()
                > self.flocking_params.max_speed * self.flocking_params.max_speed
            {
                p.vel = p.vel.normalize() * self.flocking_params.max_speed;
            }

            p.pos += p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}
