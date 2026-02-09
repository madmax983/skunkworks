use macroquad::prelude::*;
use rayon::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

// Constants for System Monitor scaling
const SIGMA_BASE: f32 = 10.0;
const SIGMA_SCALE: f32 = 40.0;
const RHO_BASE: f32 = 28.0;
const RHO_SCALE: f32 = 72.0;
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
    pub jitter: f32,
    pub color_shift: f32,
}

impl Default for LorenzParams {
    fn default() -> Self {
        Self {
            sigma: SIGMA_BASE,
            rho: RHO_BASE,
            beta: BETA_VAL,
            jitter: 0.0,
            color_shift: 0.0,
        }
    }
}

pub struct SystemMonitor {
    sys: System,
    pub params: LorenzParams,
    last_update: f64,
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
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
            last_update: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = get_time();
        if now - self.last_update < 1.0 {
            return;
        }
        self.last_update = now;

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
        self.params.sigma = SIGMA_BASE + (cpu_usage / 100.0) * SIGMA_SCALE;

        // Rho (Rayleigh): 28.0 base.
        // RAM -> Rho (Driving Force)
        // 0% -> 28.0
        // 100% -> 100.0
        self.params.rho = RHO_BASE + (mem_usage * RHO_SCALE);

        // Beta: 8/3 ~ 2.66
        self.params.beta = BETA_VAL;

        // Swap -> Jitter
        let total_swap = self.sys.total_swap() as f32;
        let used_swap = self.sys.used_swap() as f32;
        let swap_usage = if total_swap > 0.0 {
            used_swap / total_swap
        } else {
            0.0
        };
        self.params.jitter = swap_usage * 0.5; // Max 0.5 jitter

        // Load Avg -> Color Shift
        // Load Avg is usually 0 to N (cores). Normalize somewhat?
        // Let's take 1 min load avg.
        let load = System::load_average();
        let load_val = load.one as f32;
        // Assume load > 4.0 is high
        self.params.color_shift = (load_val / 4.0).clamp(0.0, 1.0);
    }
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub monitor: SystemMonitor,
}

impl Simulation {
    pub fn new(count: usize) -> Self {
        let mut particles = Vec::with_capacity(count);
        for _ in 0..count {
            particles.push(Particle::random());
        }

        Self {
            particles,
            monitor: SystemMonitor::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.monitor.update();
        update_particles(&mut self.particles, &self.monitor.params, dt);
    }

    pub fn reset(&mut self) {
        // Reset single-threaded to avoid rand concurrency issues
        self.particles.iter_mut().for_each(|p| {
            *p = Particle::random();
        });
    }
}

pub fn update_particles(particles: &mut [Particle], params: &LorenzParams, dt: f32) {
    particles.par_iter_mut().for_each(|p| {
        // RK4 Integration
        let mut next_pos = solve_rk4(p.pos, params, dt);

        // Jitter (Swap usage)
        if params.jitter > 0.01 {
            // Deterministic noise based on position to avoid non-thread-safe RNG
            let noise = ((p.pos.x * 12.9898 + p.pos.y * 78.233).sin() * 43758.5453).fract();
            let noise2 = ((p.pos.y * 12.9898 + p.pos.z * 78.233).sin() * 43758.5453).fract();
            let noise3 = ((p.pos.z * 12.9898 + p.pos.x * 78.233).sin() * 43758.5453).fract();

            next_pos.x += (noise - 0.5) * params.jitter;
            next_pos.y += (noise2 - 0.5) * params.jitter;
            next_pos.z += (noise3 - 0.5) * params.jitter;
        }

        let diff = next_pos - p.pos;

        p.pos = next_pos;

        // Update velocity for coloring (approximate)
        // We use (dx, dy, dz) / dt essentially, but here we just store displacement as velocity proxy
        p.vel = diff / dt;

        // Color mapping
        // Velocity magnitude helps visualize speed
        let speed = p.vel.length();
        // Map speed to color (Blue -> Red)
        // Typical speed in Lorenz is around 10-50
        let t = (speed / 50.0).clamp(0.0, 1.0);

        // Load Avg influences color shift (Hue shift simulation)
        // Shift t by params.color_shift
        let shifted_t = (t + params.color_shift).fract(); // Cycle through

        // Simple heatmap: Blue (slow) -> Red (fast)
        // R = t, G = 0, B = 1-t
        // With shift:
        p.color = Color::new(shifted_t, 0.2, 1.0 - shifted_t, 0.6);
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
