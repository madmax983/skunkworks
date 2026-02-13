use std::f32::consts::PI;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32, // Acceleration
    pub ay: f32,
    pub rho: f32, // Density
    pub pressure: f32,
    pub container_id: usize,
}

impl Particle {
    pub fn new(x: f32, y: f32, container_id: usize) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            rho: 0.0,
            pressure: 0.0,
            container_id,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Container {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

pub struct FluidSolver {
    pub particles: Vec<Particle>,
    pub containers: Vec<Container>,
    pub gravity: f32,
    pub h: f32, // Smoothing radius
    pub rest_density: f32,
    pub k: f32,  // Gas constant (stiffness)
    pub mu: f32, // Viscosity
}

impl FluidSolver {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            containers: Vec::new(),
            gravity: 0.5,
            h: 4.0,
            rest_density: 0.1,
            k: 50.0,
            mu: 0.1,
        }
    }

    pub fn add_container(&mut self, id: usize, x: f32, y: f32, w: f32, h: f32) {
        self.containers.push(Container { id, x, y, w, h });
    }

    pub fn add_particle(&mut self, x: f32, y: f32, container_id: usize) {
        self.particles.push(Particle::new(x, y, container_id));
    }

    /// Updates physics and returns a list of particles that have "drained" (hit the bottom)
    /// Returns: Vec<(container_id, x_pos_relative_to_container)>
    pub fn update(&mut self, dt: f32) -> Vec<(usize, f32)> {
        self.compute_density_pressure();
        self.compute_forces();
        self.integrate(dt)
    }

    fn compute_density_pressure(&mut self) {
        let n = self.particles.len();
        let h2 = self.h * self.h;
        let poly6_coeff = 315.0 / (64.0 * PI * self.h.powi(9));

        // Optimized: Only compute interactions between particles in the SAME container
        // Actually, nearby containers might interact if close?
        // For simplicity, let's assume containers are isolated physics worlds for now,
        // OR let's just do O(N^2) but check container_id.
        // Doing O(N^2) is fine for small N (<500).

        for i in 0..n {
            let mut rho = 0.0;
            for j in 0..n {
                // Optimization: Only interact if in same container
                if self.particles[i].container_id != self.particles[j].container_id {
                    continue;
                }

                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                if r2 < h2 {
                    rho += poly6_coeff * (h2 - r2).powi(3);
                }
            }
            self.particles[i].rho = rho.max(0.0001);
            self.particles[i].pressure = self.k * (self.particles[i].rho - self.rest_density);
        }
    }

    fn compute_forces(&mut self) {
        let n = self.particles.len();
        let h2 = self.h * self.h;
        let spiky_grad_coeff = -45.0 / (PI * self.h.powi(6));

        for i in 0..n {
            let mut fx = 0.0;
            let mut fy = 0.0;

            for j in 0..n {
                if i == j {
                    continue;
                }
                if self.particles[i].container_id != self.particles[j].container_id {
                    continue;
                }

                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                if r2 > 0.0 && r2 < h2 {
                    let r = r2.sqrt();
                    let force_pressure = (self.particles[i].pressure + self.particles[j].pressure)
                        / (2.0 * self.particles[j].rho);
                    let grad = spiky_grad_coeff * (self.h - r).powi(2);
                    let f_p = -force_pressure * grad;

                    fx -= f_p * (dx / r);
                    fy -= f_p * (dy / r);
                }
            }
            self.particles[i].ax = fx;
            self.particles[i].ay = fy + self.gravity;
        }
    }

    fn integrate(&mut self, dt: f32) -> Vec<(usize, f32)> {
        let mut drained = Vec::new();
        let mut particles_to_keep = Vec::with_capacity(self.particles.len());

        // Take particles out to avoid borrow checker issues with self.containers
        let particles = std::mem::take(&mut self.particles);

        for mut p in particles {
            p.vx += p.ax * dt;
            p.vy += p.ay * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Find container
            let mut keep = true;
            if let Some(c) = self.containers.iter().find(|c| c.id == p.container_id) {
                let damping = 0.5;

                // Left
                if p.x < c.x {
                    p.x = c.x;
                    p.vx *= -damping;
                }
                // Right
                if p.x > c.x + c.w {
                    p.x = c.x + c.w;
                    p.vx *= -damping;
                }
                // Top (Ceiling for containment)
                if p.y < c.y {
                    p.y = c.y;
                    p.vy *= -damping;
                }
                // Bottom (The Drain)
                if p.y > c.y + c.h {
                    // Drained!
                    drained.push((c.id, p.x));
                    keep = false;
                }
            } else {
                // Orphan particle? Delete it.
                keep = false;
            }

            if keep {
                particles_to_keep.push(p);
            }
        }

        self.particles = particles_to_keep;
        drained
    }
}
