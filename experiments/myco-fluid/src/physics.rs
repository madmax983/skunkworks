use locus::Vec2;
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
    pub grid_w: usize,
    pub grid_h: usize,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn particles
        for _ in 0..1000 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.2..width * 0.8),
                rng.gen_range(height * 0.1..height * 0.5),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            grid_w,
            grid_h,
            trails: vec![0.0; grid_w * grid_h],
            next_trails: vec![0.0; grid_w * grid_h],
        }
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        if x < self.grid_w && y < self.grid_h {
            self.trails[y * self.grid_w + x]
        } else {
            0.0
        }
    }

    pub fn add_magnet(&mut self, x: f64, y: f64, polarity: bool) {
        self.magnets.push(Magnet {
            pos: Vec2::new(x, y),
            strength: 3000.0,
            polarity,
        });
    }

    pub fn update(&mut self, dt: f64) {
        let gravity = Vec2::new(0.0, -10.0);
        let damping = 0.94;
        let decay = 0.98; // Pheromone decay rate

        // 1. Evaporate and diffuse trails
        for y in 1..self.grid_h - 1 {
            for x in 1..self.grid_w - 1 {
                let idx = y * self.grid_w + x;

                // Diffuse 3x3 kernel
                let sum = self.trails[(y - 1) * self.grid_w + x - 1]
                    + self.trails[(y - 1) * self.grid_w + x]
                    + self.trails[(y - 1) * self.grid_w + x + 1]
                    + self.trails[y * self.grid_w + x - 1]
                    + self.trails[idx]
                    + self.trails[y * self.grid_w + x + 1]
                    + self.trails[(y + 1) * self.grid_w + x - 1]
                    + self.trails[(y + 1) * self.grid_w + x]
                    + self.trails[(y + 1) * self.grid_w + x + 1];

                let blurred = sum / 9.0;
                self.next_trails[idx] = blurred * decay;
            }
        }
        std::mem::swap(&mut self.trails, &mut self.next_trails);

        // 2. Deposit Pheromones (Particles leave a trail based on their position)
        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;

            if gx < self.grid_w && gy < self.grid_h {
                self.trails[gy * self.grid_w + gx] = (self.trails[gy * self.grid_w + gx] + 0.5).min(1.0);
            }
        }

        // 3. Update Particles
        for i in 0..self.particles.len() {
            let mut force = gravity;
            let p_pos = self.particles[i].pos;

            // Magnetism
            for mag in &self.magnets {
                let delta = mag.pos - p_pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (mag.strength / dist_sq).min(300.0);

                    if mag.polarity {
                        force = force + dir * mag_force; // Attract
                    } else {
                        force = force + dir * -mag_force; // Repel
                    }
                }
            }

            // Pheromone Guidance (Particles are weakly attracted to high pheromone concentration)
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0 && gx < (self.grid_w - 1) && gy > 0 && gy < (self.grid_h - 1) {
                // Gradient of pheromones
                let left = self.get_trail(gx - 1, gy);
                let right = self.get_trail(gx + 1, gy);
                let down = self.get_trail(gx, gy - 1);
                let up = self.get_trail(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                // Pull towards high density
                let pheromone_force = Vec2::new(dx, dy) * 80.0;
                force = force + pheromone_force;
            }

            self.particles[i].acc = force;
        }

        // Integrate
        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x >= self.width {
                p.pos.x = self.width - 0.1;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y >= self.height {
                p.pos.y = self.height - 0.1;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut u = Universe::new(100.0, 100.0);

        // Place particle high up
        u.particles[0].pos = Vec2::new(50.0, 90.0);
        u.particles[0].vel = Vec2::zero();

        let initial_y = u.particles[0].pos.y;

        u.update(0.1);

        let p = &u.particles[0];
        // Gravity should pull it down
        assert!(p.pos.y < initial_y);
    }
}
