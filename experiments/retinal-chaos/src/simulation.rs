use macroquad::prelude::*;
use rayon::prelude::*;

// Constants for Lorenz Params
const SIGMA_BASE: f32 = 10.0;
const RHO_BASE: f32 = 28.0;
const BETA_VAL: f32 = 8.0 / 3.0;

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Vec3,
    pub vel: Vec3,
    pub color: Color,
}

impl Particle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            pos: vec3(x, y, z),
            vel: vec3(0.0, 0.0, 0.0),
            color: WHITE,
        }
    }

    pub fn random() -> Self {
        Self::new(
            rand::gen_range(-10.0, 10.0),
            rand::gen_range(-10.0, 10.0),
            rand::gen_range(10.0, 40.0),
        )
    }
}


pub struct LorenzParams {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: SIGMA_BASE,
            rho: RHO_BASE,
            beta: BETA_VAL,
        }
    }
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub params: LorenzParams,
}

impl Simulation {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for _ in 0..count {
            particles.push(Particle::random());
        }

        Self {
            particles,
            params: LorenzParams::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        update_particles(&mut self.particles, &self.params, dt);
    }

    pub fn reset(&mut self) {
        self.particles.par_iter_mut().for_each(|p| {
            *p = Particle::random();
        });
        self.params = LorenzParams::default();
    }
}

pub fn update_particles(particles: &mut [Particle], params: &LorenzParams, dt: f32) {
    particles.par_iter_mut().for_each(|p| {
        // RK4 Integration
        let next_pos = solve_rk4(p.pos, params, dt);
        let diff = next_pos - p.pos;

        p.pos = next_pos;

        // Update velocity for coloring (approximate)
        p.vel = diff / dt;

        // Color mapping
        // Velocity magnitude helps visualize speed
        let speed = p.vel.length();
        // Map speed to color (Blue -> Red)
        // Typical speed in Lorenz is around 10-50
        let t = (speed / 50.0).clamp(0.0, 1.0);

        // Simple heatmap: Blue (slow) -> Red (fast)
        p.color = Color::new(t, 0.2, 1.0 - t, 0.6);
    });
}

pub fn derivatives(pos: Vec3, params: &LorenzParams) -> Vec3 {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    let dx = params.sigma * (y - x);
    let dy = x * (params.rho - z) - y;
    let dz = x * y - params.beta * z;

    vec3(dx, dy, dz)
}

pub fn solve_rk4(pos: Vec3, params: &LorenzParams, dt: f32) -> Vec3 {
    let k1 = derivatives(pos, params);
    let k2 = derivatives(pos + k1 * dt * 0.5, params);
    let k3 = derivatives(pos + k2 * dt * 0.5, params);
    let k4 = derivatives(pos + k3 * dt, params);

    pos + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0)
}
