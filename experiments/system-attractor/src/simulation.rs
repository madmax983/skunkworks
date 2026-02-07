use macroquad::prelude::*;
use rayon::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

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
}

pub struct LorenzParams {
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }
}

pub struct SystemMonitor {
    sys: System,
    pub params: LorenzParams,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything()),
        );
        Self {
            sys,
            params: LorenzParams::default(),
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();

        let cpu_usage = self.sys.global_cpu_info().cpu_usage(); // 0-100
        let total_mem = self.sys.total_memory() as f32;
        let used_mem = self.sys.used_memory() as f32;
        let mem_usage = if total_mem > 0.0 {
            used_mem / total_mem
        } else {
            0.0
        };

        // Map to Lorenz Params
        // Sigma (Prandtl): 10.0 base.
        // CPU -> Sigma (Turbulence)
        // 0% -> 10.0
        // 100% -> 50.0
        self.params.sigma = 10.0 + (cpu_usage / 100.0) * 40.0;

        // Rho (Rayleigh): 28.0 base.
        // RAM -> Rho (Driving Force)
        // 0% -> 28.0
        // 100% -> 100.0
        self.params.rho = 28.0 + (mem_usage * 72.0);

        // Beta: 8/3 ~ 2.66
        self.params.beta = 8.0 / 3.0;
    }
}

pub fn update_particles(particles: &mut [Particle], params: &LorenzParams, dt: f32) {
    let sigma = params.sigma;
    let rho = params.rho;
    let beta = params.beta;

    particles.par_iter_mut().for_each(|p| {
        let x = p.pos.x;
        let y = p.pos.y;
        let z = p.pos.z;

        // RK4 Integration
        let (k1_x, k1_y, k1_z) = derivatives(x, y, z, sigma, rho, beta);
        let (k2_x, k2_y, k2_z) = derivatives(
            x + k1_x * dt * 0.5,
            y + k1_y * dt * 0.5,
            z + k1_z * dt * 0.5,
            sigma,
            rho,
            beta,
        );
        let (k3_x, k3_y, k3_z) = derivatives(
            x + k2_x * dt * 0.5,
            y + k2_y * dt * 0.5,
            z + k2_z * dt * 0.5,
            sigma,
            rho,
            beta,
        );
        let (k4_x, k4_y, k4_z) = derivatives(
            x + k3_x * dt,
            y + k3_y * dt,
            z + k3_z * dt,
            sigma,
            rho,
            beta,
        );

        let dx = (k1_x + 2.0 * k2_x + 2.0 * k3_x + k4_x) / 6.0;
        let dy = (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y) / 6.0;
        let dz = (k1_z + 2.0 * k2_z + 2.0 * k3_z + k4_z) / 6.0;

        p.pos.x += dx * dt;
        p.pos.y += dy * dt;
        p.pos.z += dz * dt;

        // Update velocity for coloring (approximate)
        p.vel = vec3(dx, dy, dz);

        // Color mapping
        // Velocity magnitude helps visualize speed
        let speed = p.vel.length();
        // Map speed to color (Blue -> Red)
        // Typical speed in Lorenz is around 10-50
        let t = (speed / 50.0).clamp(0.0, 1.0);

        // Simple heatmap: Blue (slow) -> Red (fast)
        // R = t, G = 0, B = 1-t
        p.color = Color::new(t, 0.2, 1.0 - t, 0.6);
    });
}

fn derivatives(x: f32, y: f32, z: f32, sigma: f32, rho: f32, beta: f32) -> (f32, f32, f32) {
    let dx = sigma * (y - x);
    let dy = x * (rho - z) - y;
    let dz = x * y - beta * z;
    (dx, dy, dz)
}
