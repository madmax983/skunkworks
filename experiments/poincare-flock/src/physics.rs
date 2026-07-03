use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use poincare_disk::{mobius_add, Point};
use rand::Rng;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub params: FlockingParams,
    pub positions_buffer: Vec<Vec2>,
    pub velocities_buffer: Vec<Vec2>,
    pub next_velocities_buffer: Vec<Vec2>,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..150 {
            let p = Particle {
                pos: Vec2::new(
                    rng.gen_range(width * 0.4..width * 0.6),
                    rng.gen_range(height * 0.4..height * 0.6),
                ),
                vel: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize()
                    * 1.5,
            };
            particles.push(p);
        }

        Self {
            particles,
            width,
            height,
            positions_buffer: Vec::with_capacity(150),
            velocities_buffer: Vec::with_capacity(150),
            next_velocities_buffer: Vec::with_capacity(150),
            params: FlockingParams {
                view_radius: 20.0,
                separation_radius: 5.0,
                max_speed: 1.5,
                max_force: 0.05,
                separation_weight: 1.5,
                alignment_weight: 1.0,
                cohesion_weight: 1.0,
            },
        }
    }

    pub fn update(&mut self, _dt: f64) {
        let n = self.particles.len();

        self.positions_buffer.clear();
        self.positions_buffer
            .extend(self.particles.iter().map(|p| p.pos));

        self.velocities_buffer.clear();
        self.velocities_buffer
            .extend(self.particles.iter().map(|p| p.vel));

        self.next_velocities_buffer.clear();
        self.next_velocities_buffer
            .extend(self.velocities_buffer.iter().copied());

        let center = Vec2::new(self.width / 2.0, self.height / 2.0);
        let max_r = (self.width.min(self.height) / 2.0) - 2.0;

        for i in 0..n {
            // Apply flocking rules in Euclidean space (local perception)
            let force = compute_force(
                &self.positions_buffer,
                &self.velocities_buffer,
                i,
                &self.params,
            );
            self.next_velocities_buffer[i] += force;
            self.next_velocities_buffer[i] =
                self.next_velocities_buffer[i].limit(self.params.max_speed);

            // Central attractor to keep them from getting lost in infinity
            let delta = center - self.positions_buffer[i];
            let dist = delta.magnitude();
            if dist > max_r * 0.2 {
                self.next_velocities_buffer[i] += delta.normalize() * 0.02 * (dist / max_r);
                self.next_velocities_buffer[i] =
                    self.next_velocities_buffer[i].limit(self.params.max_speed);
            }
        }

        for i in 0..n {
            self.particles[i].vel = self.next_velocities_buffer[i];

            // Map position to unit disk [-1, 1]
            let mut p_nx = (self.particles[i].pos.x - center.x) / max_r;
            let mut p_ny = (self.particles[i].pos.y - center.y) / max_r;

            // Map velocity vector to translation vector in unit disk
            let mut v_nx = self.particles[i].vel.x / max_r;
            let mut v_ny = self.particles[i].vel.y / max_r;

            let p_rad = (p_nx * p_nx + p_ny * p_ny).sqrt();
            let v_rad = (v_nx * v_nx + v_ny * v_ny).sqrt();

            let p_scale = if p_rad >= 0.999 { 0.999 / p_rad } else { 1.0 };
            p_nx *= p_scale;
            p_ny *= p_scale;

            let v_scale = if v_rad >= 0.999 { 0.999 / v_rad } else { 1.0 };
            v_nx *= v_scale;
            v_ny *= v_scale;

            let p_point = Point::new(p_nx, p_ny);
            let v_point = Point::new(v_nx, v_ny);

            // Add translation via Möbius addition to warp space hyperbolically
            let new_p = mobius_add(p_point, v_point);

            // Map back to screen coords
            self.particles[i].pos.x = new_p.re * max_r + center.x;
            self.particles[i].pos.y = new_p.im * max_r + center.y;

            // Extra safety boundary reflection
            let mut delta_c = self.particles[i].pos - center;
            let current_dist = delta_c.magnitude();
            if current_dist > max_r * 0.99 {
                delta_c = delta_c.normalize() * (max_r * 0.99);
                self.particles[i].pos = center + delta_c;
                // Reflect velocity
                let n = -delta_c.normalize();
                self.particles[i].vel =
                    self.particles[i].vel - n * (2.0 * self.particles[i].vel.dot(n));
            }
        }
    }
}
