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
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            rho: 0.0,
            pressure: 0.0,
        }
    }
}

pub struct FluidSolver {
    pub particles: Vec<Particle>,
    pub width: f32,
    pub height: f32,
    pub gravity: f32,
    pub h: f32, // Smoothing radius
    pub rest_density: f32,
    pub k: f32, // Gas constant (stiffness)
    #[allow(dead_code)]
    pub mu: f32, // Viscosity
}

impl FluidSolver {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            gravity: 0.5,
            h: 4.0,
            rest_density: 0.1,
            k: 50.0,
            mu: 0.1,
        }
    }

    pub fn add_particle(&mut self, x: f32, y: f32) {
        self.particles.push(Particle::new(x, y));
    }

    pub fn update(&mut self, dt: f32) {
        self.compute_density_pressure();
        self.compute_forces();
        self.integrate(dt);
    }

    // Poly6 Kernel for Density
    // Kept for tests and reference; logic is inlined in compute_density_pressure for performance
    #[allow(dead_code)]
    fn poly6_kernel(&self, r2: f32) -> f32 {
        let h2 = self.h * self.h;
        if r2 < 0.0 || r2 > h2 {
            return 0.0;
        }
        let diff = h2 - r2;
        (315.0 / (64.0 * PI * self.h.powi(9))) * diff.powi(3)
    }

    // Spiky Kernel Gradient for Pressure
    // Kept for tests and reference; logic is inlined in compute_forces for performance
    #[allow(dead_code)]
    fn spiky_kernel_gradient(&self, r: f32) -> f32 {
        if r <= 0.0 || r > self.h {
            return 0.0;
        }
        let diff = self.h - r;
        -(45.0 / (PI * self.h.powi(6))) * diff.powi(2)
    }

    fn compute_density_pressure(&mut self) {
        let n = self.particles.len();
        let h2 = self.h * self.h;
        // Precompute poly6 coefficient: 315 / (64 * PI * h^9)
        let poly6_coeff = 315.0 / (64.0 * PI * self.h.powi(9));

        // Naive O(N^2)
        for i in 0..n {
            let mut rho = 0.0;
            for j in 0..n {
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                // Optimization: Inline kernel and check r2 < h2
                if r2 < h2 {
                    rho += poly6_coeff * (h2 - r2).powi(3);
                }
            }
            self.particles[i].rho = rho.max(0.0001); // Avoid div by zero
            // P = k * (rho - rho0)
            self.particles[i].pressure = self.k * (self.particles[i].rho - self.rest_density);
        }
    }

    fn compute_forces(&mut self) {
        let n = self.particles.len();
        let h2 = self.h * self.h;
        // Precompute spiky gradient coefficient: -45 / (PI * h^6)
        let spiky_grad_coeff = -45.0 / (PI * self.h.powi(6));

        for i in 0..n {
            let mut fx = 0.0;
            let mut fy = 0.0;

            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;

                // Optimization: Check distance squared before sqrt to avoid expensive sqrt for far particles
                let r2 = dx * dx + dy * dy;

                if r2 > 0.0 && r2 < h2 {
                    let r = r2.sqrt();

                    // Pressure Force
                    // Fp = - mass * (Pi + Pj) / (2 * rho_j) * grad W
                    // Assuming mass = 1 for simplicity
                    let force_pressure = (self.particles[i].pressure + self.particles[j].pressure)
                        / (2.0 * self.particles[j].rho);

                    // Optimization: Inline spiky gradient
                    // let grad = self.spiky_kernel_gradient(r);
                    let grad = spiky_grad_coeff * (self.h - r).powi(2);

                    let f_p = -force_pressure * grad;

                    fx -= f_p * (dx / r);
                    fy -= f_p * (dy / r);

                    // Viscosity Force (simplified)
                    // Fv = mu * (vj - vi) / rho_j * laplacian W (using simple approximation)
                    // Actually let's use a simpler damping for now or correct viscosity kernel
                }
            }

            self.particles[i].ax = fx;
            self.particles[i].ay = fy + self.gravity;
        }
    }

    fn integrate(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.vx += p.ax * dt;
            p.vy += p.ay * dt;

            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Boundaries
            let damping = 0.5;

            // Left
            if p.x < 0.0 {
                p.x = 0.0;
                p.vx *= -damping;
            }
            // Right
            if p.x > self.width {
                p.x = self.width;
                p.vx *= -damping;
            }
            // Top
            if p.y < 0.0 {
                p.y = 0.0;
                p.vy *= -damping;
            }
            // Bottom
            if p.y > self.height {
                p.y = self.height;
                p.vy *= -damping;
            }
        }
    }
}
