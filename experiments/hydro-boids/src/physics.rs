use std::f32::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Species {
    Fluid, // Passive fluid particle
    Boid,  // Active flocking agent
}

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ax: f32,
    pub ay: f32,
    pub rho: f32,
    pub pressure: f32,
    pub species: Species,
}

impl Particle {
    pub fn new(x: f32, y: f32, species: Species) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            rho: 0.0,
            pressure: 0.0,
            species,
        }
    }
}

pub struct FluidSolver {
    pub particles: Vec<Particle>,
    pub width: f32,
    pub height: f32,

    // SPH Constants
    pub h: f32,             // Smoothing radius
    pub rest_density: f32,  // Target density
    pub k: f32,             // Gas constant (pressure multiplier)
    pub mu: f32,            // Viscosity coefficient

    // Boid Constants
    pub view_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub max_speed: f32,
    pub max_force: f32,
}

impl FluidSolver {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            particles: Vec::new(),
            width,
            height,
            // SPH Params
            h: 4.0,
            rest_density: 0.1,
            k: 50.0,
            mu: 0.1,
            // Boid Params
            view_radius: 8.0,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
            max_speed: 1.5,
            max_force: 0.05,
        }
    }

    pub fn add_particle(&mut self, x: f32, y: f32, species: Species) {
        let mut p = Particle::new(x, y, species);
        // Give random initial velocity to stir things up
        use rand::Rng;
        let mut rng = rand::thread_rng();
        p.vx = rng.gen_range(-0.5..0.5);
        p.vy = rng.gen_range(-0.5..0.5);
        self.particles.push(p);
    }

    pub fn update(&mut self, dt: f32) {
        self.compute_density_pressure();
        self.compute_forces();
        self.compute_boid_steering(); // Add biological forces
        self.integrate(dt);
    }

    fn compute_density_pressure(&mut self) {
        let n = self.particles.len();
        let h2 = self.h * self.h;
        let poly6_coeff = 315.0 / (64.0 * PI * self.h.powi(9));

        for i in 0..n {
            let mut rho = 0.0;
            for j in 0..n {
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
        let viscosity_coeff = 45.0 * self.mu / (PI * self.h.powi(6));

        for i in 0..n {
            let mut fx = 0.0;
            let mut fy = 0.0;

            for j in 0..n {
                if i == j { continue; }
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let r2 = dx * dx + dy * dy;

                if r2 > 0.0 && r2 < h2 {
                    let r = r2.sqrt();

                    // Pressure Force
                    let force_pressure = (self.particles[i].pressure + self.particles[j].pressure)
                                       / (2.0 * self.particles[j].rho);
                    let grad = spiky_grad_coeff * (self.h - r).powi(2);
                    let f_p = -force_pressure * grad;

                    fx -= f_p * (dx / r);
                    fy -= f_p * (dy / r);

                    // Viscosity Force
                    let laplacian = self.h - r;
                    let f_v = viscosity_coeff * laplacian / self.particles[j].rho;

                    fx += f_v * (self.particles[j].vx - self.particles[i].vx);
                    fy += f_v * (self.particles[j].vy - self.particles[i].vy);
                }
            }

            // Gravity/Buoyancy
            // Boids are neutrally buoyant? Or maybe slightly heavy?
            // Fluid is heavy?
            // Let's add slight gravity to everything to keep them in the tank (Bottom is Y=0 in Ratatui Canvas)
            let gravity = -0.05;
            fy += gravity;

            self.particles[i].ax = fx;
            self.particles[i].ay = fy;
        }
    }

    fn compute_boid_steering(&mut self) {
        // Collect forces first to avoid borrowing issues
        let n = self.particles.len();
        let mut steering_forces = vec![(0.0, 0.0); n];
        let view_radius_sq = self.view_radius * self.view_radius;

        for i in 0..n {
            if self.particles[i].species != Species::Boid {
                continue;
            }

            let mut sep = (0.0, 0.0);
            let mut ali = (0.0, 0.0);
            let mut coh = (0.0, 0.0);
            let mut count = 0;

            for j in 0..n {
                if i == j { continue; }
                // Boids interact with other Boids for flocking
                if self.particles[j].species != Species::Boid { continue; }

                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let d2 = dx*dx + dy*dy;

                if d2 < view_radius_sq && d2 > 0.0 {
                    let d = d2.sqrt();

                    // Separation: Steer away
                    sep.0 -= dx / d;
                    sep.1 -= dy / d;

                    // Alignment: Match velocity
                    ali.0 += self.particles[j].vx;
                    ali.1 += self.particles[j].vy;

                    // Cohesion: Steer towards position
                    coh.0 += self.particles[j].x;
                    coh.1 += self.particles[j].y;

                    count += 1;
                }
            }

            if count > 0 {
                let count_f = count as f32;

                // Average and Normalize
                if sep.0 != 0.0 || sep.1 != 0.0 {
                    let len = (sep.0*sep.0 + sep.1*sep.1).sqrt();
                    sep.0 = (sep.0 / len) * self.max_speed;
                    sep.1 = (sep.1 / len) * self.max_speed;
                    // Steering = Desired - Velocity
                    sep.0 -= self.particles[i].vx;
                    sep.1 -= self.particles[i].vy;
                }

                ali.0 /= count_f;
                ali.1 /= count_f;
                let ali_len = (ali.0*ali.0 + ali.1*ali.1).sqrt();
                if ali_len > 0.0 {
                    ali.0 = (ali.0 / ali_len) * self.max_speed;
                    ali.1 = (ali.1 / ali_len) * self.max_speed;
                    ali.0 -= self.particles[i].vx;
                    ali.1 -= self.particles[i].vy;
                }

                coh.0 /= count_f;
                coh.1 /= count_f;
                // Vector to target
                coh.0 -= self.particles[i].x;
                coh.1 -= self.particles[i].y;
                let coh_len = (coh.0*coh.0 + coh.1*coh.1).sqrt();
                if coh_len > 0.0 {
                    coh.0 = (coh.0 / coh_len) * self.max_speed;
                    coh.1 = (coh.1 / coh_len) * self.max_speed;
                    coh.0 -= self.particles[i].vx;
                    coh.1 -= self.particles[i].vy;
                }
            }

            // Limit forces
            let limit = |vx: f32, vy: f32, max: f32| {
                let len = (vx*vx + vy*vy).sqrt();
                if len > max {
                    (vx / len * max, vy / len * max)
                } else {
                    (vx, vy)
                }
            };

            let (sep_x, sep_y) = limit(sep.0, sep.1, self.max_force);
            let (ali_x, ali_y) = limit(ali.0, ali.1, self.max_force);
            let (coh_x, coh_y) = limit(coh.0, coh.1, self.max_force);

            steering_forces[i] = (
                sep_x * self.separation_weight + ali_x * self.alignment_weight + coh_x * self.cohesion_weight,
                sep_y * self.separation_weight + ali_y * self.alignment_weight + coh_y * self.cohesion_weight
            );
        }

        // Apply
        for i in 0..n {
            self.particles[i].ax += steering_forces[i].0;
            self.particles[i].ay += steering_forces[i].1;
        }
    }

    fn integrate(&mut self, dt: f32) {
        for p in &mut self.particles {
            p.vx += p.ax * dt;
            p.vy += p.ay * dt;

            // Damping/Friction
            p.vx *= 0.98;
            p.vy *= 0.98;

            // Limit speed
            let speed = (p.vx*p.vx + p.vy*p.vy).sqrt();
            if speed > self.max_speed * 2.0 { // Allow bursts
                p.vx = (p.vx / speed) * self.max_speed * 2.0;
                p.vy = (p.vy / speed) * self.max_speed * 2.0;
            }

            p.x += p.vx * dt;
            p.y += p.vy * dt;

            // Boundaries (Wrap around for X, Bounce for Y)
            if p.x < 0.0 { p.x = self.width; }
            if p.x > self.width { p.x = 0.0; }

            if p.y < 0.0 {
                p.y = 0.0;
                p.vy *= -0.5;
            }
            if p.y > self.height {
                p.y = self.height;
                p.vy *= -0.5;
            }
        }
    }
}
