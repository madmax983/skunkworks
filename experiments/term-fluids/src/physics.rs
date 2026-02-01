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
    pub k: f32,  // Gas constant (stiffness)
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
    fn poly6_kernel(&self, r2: f32) -> f32 {
        let h2 = self.h * self.h;
        if r2 < 0.0 || r2 > h2 {
            return 0.0;
        }
        let diff = h2 - r2;
        (315.0 / (64.0 * PI * self.h.powi(9))) * diff.powi(3)
    }

    // Spiky Kernel Gradient for Pressure
    fn spiky_kernel_gradient(&self, r: f32) -> f32 {
        if r <= 0.0 || r > self.h {
            return 0.0;
        }
        let diff = self.h - r;
        -(45.0 / (PI * self.h.powi(6))) * diff.powi(2)
    }

    fn compute_density_pressure(&mut self) {
        let n = self.particles.len();
        // Naive O(N^2)
        for i in 0..n {
            let mut rho = 0.0;
            for j in 0..n {
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;
                rho += self.poly6_kernel(r2);
            }
            self.particles[i].rho = rho.max(0.0001); // Avoid div by zero
            // P = k * (rho - rho0)
            self.particles[i].pressure = self.k * (self.particles[i].rho - self.rest_density);
        }
    }

    fn compute_forces(&mut self) {
        let n = self.particles.len();
        for i in 0..n {
            let mut fx = 0.0;
            let mut fy = 0.0;

            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r = (dx * dx + dy * dy).sqrt();

                if r > 0.0 && r < self.h {
                    // Pressure Force
                    // Fp = - mass * (Pi + Pj) / (2 * rho_j) * grad W
                    // Assuming mass = 1 for simplicity
                    let force_pressure = (self.particles[i].pressure + self.particles[j].pressure)
                        / (2.0 * self.particles[j].rho);
                    let grad = self.spiky_kernel_gradient(r);
                    let f_p = -force_pressure * grad;

                    fx += f_p * (dx / r);
                    fy += f_p * (dy / r);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let mut solver = FluidSolver::new(100.0, 100.0);
        solver.add_particle(50.0, 50.0);

        solver.particles[0].vx = 10.0;
        solver.particles[0].vy = 0.0;
        solver.gravity = 0.0;

        // SPH forces will be zero if only 1 particle (pressure depends on density, density depends on neighbors usually, but here particle sees itself)
        // Self-density: poly6(0) = 315 / 64 ...
        // Pressure = k * (rho - rest). If rho > rest, it expands.
        // But force loop skips i == j. So fx, fy = 0 (plus gravity).

        solver.update(1.0);

        // Expect position to change by velocity * dt
        assert_eq!(solver.particles[0].x, 60.0);
    }

    #[test]
    fn test_boundary_collision() {
        let mut solver = FluidSolver::new(100.0, 100.0);
        solver.add_particle(99.0, 50.0);
        solver.particles[0].vx = 10.0;
        solver.gravity = 0.0;

        solver.update(1.0);

        assert!(solver.particles[0].x <= 100.0);
        assert!(solver.particles[0].vx < 0.0);
    }

    #[test]
    fn test_stacked_particles_nan_explosion() {
        let mut solver = FluidSolver::new(100.0, 100.0);
        // Add two particles at the exact same position
        solver.add_particle(50.0, 50.0);
        solver.add_particle(50.0, 50.0);

        solver.gravity = 0.0; // Isolate from gravity
        solver.update(0.1);

        // Check for NaN
        let p1 = &solver.particles[0];
        let p2 = &solver.particles[1];

        assert!(!p1.x.is_nan(), "Particle 1 x is NaN");
        assert!(!p1.y.is_nan(), "Particle 1 y is NaN");
        assert!(!p1.vx.is_nan(), "Particle 1 vx is NaN");
        assert!(!p1.vy.is_nan(), "Particle 1 vy is NaN");

        assert!(!p2.x.is_nan(), "Particle 2 x is NaN");
        assert!(!p2.y.is_nan(), "Particle 2 y is NaN");
        assert!(!p2.vx.is_nan(), "Particle 2 vx is NaN");
        assert!(!p2.vy.is_nan(), "Particle 2 vy is NaN");
    }

    #[test]
    fn test_particle_interaction() {
        let mut solver = FluidSolver::new(100.0, 100.0);
        // Place two particles within smoothing radius h=4.0
        // P1 at (50, 50)
        // P2 at (52, 50) -> distance r=2.0
        solver.add_particle(50.0, 50.0);
        solver.add_particle(52.0, 50.0);

        solver.gravity = 0.0;

        // Initial state: velocity 0
        assert_eq!(solver.particles[0].vx, 0.0);
        assert_eq!(solver.particles[1].vx, 0.0);

        solver.update(0.1);

        // They should interact.
        // If pressure is positive, they repel.
        // P1 should move left (vx < 0), P2 should move right (vx > 0).
        // Or at least, their velocities should change from 0.

        let p1 = &solver.particles[0];
        let p2 = &solver.particles[1];

        assert!(
            p1.vx.abs() > 0.0,
            "Particle 1 should acquire velocity from interaction"
        );
        assert!(
            p2.vx.abs() > 0.0,
            "Particle 2 should acquire velocity from interaction"
        );

        // Symmetry check: forces should be equal and opposite (if masses equal)
        // Since integration is simple Euler, positions update too.
        // We check ax mainly, but ax is overwritten each step.
        // vx accumulates ax.
        // Since they started at x=50 and x=52, symmetric around 51.
        // P1.vx should be -P2.vx approximately.
        assert!(
            (p1.vx + p2.vx).abs() < 0.0001,
            "Momentum should be conserved (sum of velocities approx 0)"
        );
    }

    #[test]
    fn test_kernels() {
        let solver = FluidSolver::new(100.0, 100.0);
        let h = solver.h;

        // Poly6 Kernel
        // Check r > h (r2 > h2)
        assert_eq!(solver.poly6_kernel(h * h + 0.1), 0.0);

        // Check r = 0
        let expected_poly6_0 = 315.0 / (64.0 * PI * h.powi(9)) * (h * h).powi(3);
        // diff = h^2 - 0 = h^2. diff^3 = h^6.
        // formula: coeff * h^6
        assert!((solver.poly6_kernel(0.0) - expected_poly6_0).abs() < 0.000001);

        // Spiky Gradient
        // Check r > h
        assert_eq!(solver.spiky_kernel_gradient(h + 0.1), 0.0);

        // Check r = 0 (implementation specific)
        assert_eq!(solver.spiky_kernel_gradient(0.0), 0.0);

        // Check intermediate value
        let r = h / 2.0;
        let diff = h - r;
        let expected_grad = -(45.0 / (PI * h.powi(6))) * diff.powi(2);
        assert!((solver.spiky_kernel_gradient(r) - expected_grad).abs() < 0.000001);
    }
}
