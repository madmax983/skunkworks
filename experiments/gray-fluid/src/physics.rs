use ferrous_core::Platter;
use gray_scott::GrayScott;
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
    pub polarity: bool,
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub magnets: Vec<Magnet>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
    pub gray_scott: GrayScott,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..600 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.2..width * 0.8),
                rng.gen_range(height * 0.1..height * 0.5),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        let mut gray_scott = GrayScott::new(grid_w, grid_h);

        // Seed the center
        gray_scott.add_chemical(grid_w / 2, grid_h / 2, 1.0);

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
            gray_scott,
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
        let gravity = Vec2::new(0.0, -10.0);
        let damping = 0.96;

        self.platter.clear();

        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            self.platter.accumulate(gx, gy, 1.0);

            // Particles act as catalysts: deposit V where they are
            self.gray_scott.add_chemical(gx, gy, 0.2);
        }

        // Update Gray-Scott (f=0.055, k=0.062 spots)
        self.gray_scott.update(0.055, 0.062, 1.0);

        let gs_u = self.gray_scott.u();

        for i in 0..self.particles.len() {
            let mut force = gravity;

            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            let mut u_concentration = 1.0;
            if gx < self.gray_scott.width() && gy < self.gray_scott.height() {
                let idx = self.gray_scott.get_index(gx, gy);
                if idx < gs_u.len() {
                    // Lower U concentration = more "permeability" / stronger magnetic pull
                    // U ranges from 0.0 to 1.0
                    u_concentration = gs_u[idx] as f64;
                }
            }

            // The less U there is, the stronger the magnetism acts.
            let mag_multiplier = 1.0 + (1.0 - u_concentration) * 5.0;

            for mag in &self.magnets {
                let delta = mag.pos - p_pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mut mag_force = (mag.strength / dist_sq).min(200.0);

                    mag_force *= mag_multiplier;

                    if mag.polarity {
                        force += dir * mag_force;
                    } else {
                        force += dir * -mag_force;
                    }
                }
            }

            if gx > 0 && gx < (self.platter.width() - 1) && gy > 0 && gy < (self.platter.height() - 1) {
                let left = self.platter.get_magnetism(gx - 1, gy);
                let right = self.platter.get_magnetism(gx + 1, gy);
                let down = self.platter.get_magnetism(gx, gy - 1);
                let up = self.platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 50.0;
                force += pressure_force;
            }

            self.particles[i].acc = force;
        }

        for p in &mut self.particles {
            p.vel += p.acc * dt;
            p.vel *= damping;
            p.pos += p.vel * dt;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut u = Universe::new(100.0, 100.0);
        u.particles[0].pos = Vec2::new(50.0, 90.0);
        u.particles[0].vel = Vec2::zero();
        let initial_y = u.particles[0].pos.y;
        u.update(0.1);
        let p = &u.particles[0];
        assert!(p.pos.y < initial_y);
    }
}
