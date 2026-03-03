use crate::spectral::SpectralField;
use ferrous_core::Platter;
use locus::Vec2;
use rand::Rng;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass: 1.0,
        }
    }
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
    pub spectral_field: SpectralField,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn particles in a cluster
        for _ in 0..800 {
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let dist = rng.gen_range(0.0..height * 0.3);
            let x = width * 0.5 + angle.cos() * dist;
            let y = height * 0.5 + angle.sin() * dist;

            particles.push(Particle::new(x, y));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
            spectral_field: SpectralField::new(grid_w, grid_h),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let damping = 0.98;

        // 1. Rasterize Particles to Platter (Spatial Domain)
        self.platter.clear();
        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            self.platter.accumulate(gx, gy, 1.0);
        }

        // 2. Compute Spectral Field (Frequency Domain -> Potential)
        // We do this every frame? It's expensive but for TUI grid sizes (100x50) it's fine.
        // Convert Platter to Vec<f64> via iter/clone?
        // Platter.magnetism is Vec<f64>.
        self.spectral_field
            .compute_spectrum(&self.platter.magnetism());
        self.spectral_field.compute_potential();

        // 3. Apply Forces
        for p in &mut self.particles {
            let mut force = Vec2::zero();

            // A. Spectral Gradient Force (The "Ghost" Potential)
            // Push towards peaks? Or valleys?
            // Let's say potential is "Resonance Energy". We want to settle in low energy?
            // So push away from high potential. Force = -Gradient.
            let (dx, dy) = self.spectral_field.get_gradient(p.pos.x, p.pos.y);
            let spectral_force = Vec2::new(-dx, -dy) * 500.0; // Tune strength

            force = force + spectral_force;

            // B. Center Gravity (keep them on screen)
            let center = Vec2::new(self.width * 0.5, self.height * 0.5);
            let to_center = center - p.pos;
            force = force + to_center * 0.5;

            // Apply
            p.acc = force * (1.0 / p.mass);
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            // Boundaries
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x *= -0.8;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x *= -0.8;
            }
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y *= -0.8;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y *= -0.8;
            }
        }
    }
}
