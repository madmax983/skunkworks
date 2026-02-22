use macroquad::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

mod math;
mod grid;
mod agent;

use math::Vec4;
use grid::Grid4D;
use agent::Agent;

// --- Constants ---
const NUM_PARTICLES: usize = 2000;
const NUM_AGENTS: usize = 32;
const GRID_SIZE: usize = 20;

// --- Structs ---
#[derive(Clone, Copy)]
struct Particle {
    pos: Vec4,
    vel: Vec4,
    acc: Vec4,
}

impl Particle {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            pos: Vec4::new(x, y, z, w),
            vel: Vec4::zero(),
            acc: Vec4::zero(),
        }
    }
}

struct SystemMonitor {
    sys: System,
    last_update: f64,
    // Metrics (0.0 - 1.0)
    cpu_usage: f32,
    mem_usage: f32,
    swap_usage: f32,
    load_avg: f32,
    // Target metrics for interpolation
    target_cpu: f32,
    target_mem: f32,
    target_swap: f32,
    target_load: f32,
}

impl SystemMonitor {
    fn new() -> Self {
        Self {
            sys: System::new_with_specifics(
                RefreshKind::new()
                    .with_cpu(CpuRefreshKind::everything())
                    .with_memory(MemoryRefreshKind::everything()),
            ),
            last_update: 0.0,
            cpu_usage: 0.0,
            mem_usage: 0.0,
            swap_usage: 0.0,
            load_avg: 0.0,
            target_cpu: 0.0,
            target_mem: 0.0,
            target_swap: 0.0,
            target_load: 0.0,
        }
    }

    fn update(&mut self) {
        let now = get_time();
        if now - self.last_update > 1.0 {
            self.sys.refresh_cpu();
            self.sys.refresh_memory();
            self.last_update = now;

            self.target_cpu = self.sys.global_cpu_info().cpu_usage() / 100.0;

            let total_mem = self.sys.total_memory() as f32;
            let used_mem = self.sys.used_memory() as f32;
            self.target_mem = if total_mem > 0.0 {
                used_mem / total_mem
            } else {
                0.0
            };

            let total_swap = self.sys.total_swap() as f32;
            let used_swap = self.sys.used_swap() as f32;
            self.target_swap = if total_swap > 0.0 {
                used_swap / total_swap
            } else {
                0.0
            };

            let load = System::load_average();
            self.target_load = (load.one as f32 / 4.0).clamp(0.0, 1.0);
        }

        // Interpolate
        let dt = get_frame_time();
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let speed = 2.0 * dt;

        self.cpu_usage = lerp(self.cpu_usage, self.target_cpu, speed);
        self.mem_usage = lerp(self.mem_usage, self.target_mem, speed);
        self.swap_usage = lerp(self.swap_usage, self.target_swap, speed);
        self.load_avg = lerp(self.load_avg, self.target_load, speed);
    }
}

fn generate_tesseract_edges() -> (Vec<Vec4>, Vec<(usize, usize)>) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { 1.0 } else { -1.0 };
        let y = if i & 2 != 0 { 1.0 } else { -1.0 };
        let z = if i & 4 != 0 { 1.0 } else { -1.0 };
        let w = if i & 8 != 0 { 1.0 } else { -1.0 };
        verts.push(Vec4::new(x, y, z, w));
    }

    let mut edges = Vec::new();
    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                edges.push((i, j));
            }
        }
    }
    (verts, edges)
}

#[macroquad::main("Chimera Fluid")]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let mut grid = Grid4D::new(GRID_SIZE);

    // Spawn particles
    let mut particles = Vec::with_capacity(NUM_PARTICLES);
    for _ in 0..NUM_PARTICLES {
        particles.push(Particle::new(
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
            rand::gen_range(-1.0, 1.0),
        ));
    }

    // Spawn Agents
    let mut agents = Vec::with_capacity(NUM_AGENTS);
    for i in 0..NUM_AGENTS {
        agents.push(Agent::new_random(i as u64));
    }

    let (base_verts, edges) = generate_tesseract_edges();

    // Camera vars
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 6.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time().min(0.1); // Cap dt

        // --- Physics ---

        // 1. Clear Grid
        // Grid4D clears on creation, but we need to clear it every frame or decay it?
        // Let's clear it for "density pressure" calculation this frame.
        // Or reuse diffuse_and_decay logic from hyper-mold?
        // hyper-mold accumulates pheromones.
        // Here we want instantaneous density for pressure.
        // So we clear.
        grid.cells.fill(0.0);

        // 2. Populate Grid (Density)
        // Map [-2.0, 2.0] space to [0, GRID_SIZE]
        let map_coord = |val: f32| -> usize {
            let norm = (val + 2.0) / 4.0;
            (norm * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32) as usize
        };

        for p in &particles {
            let x = map_coord(p.pos.x);
            let y = map_coord(p.pos.y);
            let z = map_coord(p.pos.z);
            let w = map_coord(p.pos.w);
            grid.add_val(x, y, z, w, 1.0);
        }

        // Agents also contribute to density (Body mass)
        for a in &agents {
            let x = map_coord(a.pos.x);
            let y = map_coord(a.pos.y);
            let z = map_coord(a.pos.z);
            let w = map_coord(a.pos.w);
            grid.add_val(x, y, z, w, 2.0); // Agents are denser
        }

        // 3. Diffuse Density slightly?
        // grid.diffuse_and_decay(0.0, 0.5); // Optional smoothing

        // 4. Update Particles
        // Parameters driven by System Metrics
        let gravity_strength = monitor.swap_usage * 5.0; // Swap -> Gravity (pull down in Y)
        let viscosity_damping = 0.98 - (monitor.mem_usage * 0.1); // RAM -> Viscosity (lower damping = faster stop)
        let agitation_strength = monitor.cpu_usage * 2.0; // CPU -> Agitation
        let pressure_strength = 0.5; // Constant repulsion

        // Gravity Direction: Down in Y (to look like fluid in a tank)
        let gravity = Vec4::new(0.0, -gravity_strength, 0.0, 0.0);

        // Update Passive Particles
        for p in &mut particles {
            let mut force = gravity;

            // Agitation (Random kicks)
            force = force.add(Vec4::new(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
            ).scale(agitation_strength));

            // Pressure (Repulsion from high density)
            let gx = map_coord(p.pos.x);
            let gy = map_coord(p.pos.y);
            let gz = map_coord(p.pos.z);
            let gw = map_coord(p.pos.w);

            // Simple gradient check (central difference)
            // X Gradient
            let dx = grid.get(gx + 1, gy, gz, gw) - if gx > 0 { grid.get(gx - 1, gy, gz, gw) } else { 0.0 };
            // Y Gradient
            let dy = grid.get(gx, gy + 1, gz, gw) - if gy > 0 { grid.get(gx, gy - 1, gz, gw) } else { 0.0 };
            // Z Gradient
            let dz = grid.get(gx, gy, gz + 1, gw) - if gz > 0 { grid.get(gx, gy, gz - 1, gw) } else { 0.0 };
            // W Gradient
            let dw = grid.get(gx, gy, gz, gw + 1) - if gw > 0 { grid.get(gx, gy, gz, gw - 1) } else { 0.0 };

            let gradient = Vec4::new(dx, dy, dz, dw);
            // Push away from high density
            force = force.add(gradient.scale(-pressure_strength));

            // Apply Force
            p.acc = force;
            p.vel = p.vel.add(p.acc.scale(dt));

            // Damping (Viscosity)
            p.vel = p.vel.scale(viscosity_damping);

            // Move
            p.pos = p.pos.add(p.vel.scale(dt));

            // Boundary Constraints (Hypercube [-1.5, 1.5])
            let bounds = 1.5;
            if p.pos.x < -bounds || p.pos.x > bounds { p.vel.x *= -0.8; p.pos.x = p.pos.x.clamp(-bounds, bounds); }
            if p.pos.y < -bounds || p.pos.y > bounds { p.vel.y *= -0.8; p.pos.y = p.pos.y.clamp(-bounds, bounds); }
            if p.pos.z < -bounds || p.pos.z > bounds { p.vel.z *= -0.8; p.pos.z = p.pos.z.clamp(-bounds, bounds); }
            if p.pos.w < -bounds || p.pos.w > bounds { p.vel.w *= -0.8; p.pos.w = p.pos.w.clamp(-bounds, bounds); }
        }

        // Update Active Agents
        for agent in &mut agents {
             // Calculate Local Gradient
             let gx = map_coord(agent.pos.x);
             let gy = map_coord(agent.pos.y);
             let gz = map_coord(agent.pos.z);
             let gw = map_coord(agent.pos.w);

             let density = grid.get(gx, gy, gz, gw);

             let dx = grid.get(gx + 1, gy, gz, gw) - if gx > 0 { grid.get(gx - 1, gy, gz, gw) } else { 0.0 };
             let dy = grid.get(gx, gy + 1, gz, gw) - if gy > 0 { grid.get(gx, gy - 1, gz, gw) } else { 0.0 };
             let dz = grid.get(gx, gy, gz + 1, gw) - if gz > 0 { grid.get(gx, gy, gz - 1, gw) } else { 0.0 };
             let dw = grid.get(gx, gy, gz, gw + 1) - if gw > 0 { grid.get(gx, gy, gz, gw - 1) } else { 0.0 };
             let gradient = Vec4::new(dx, dy, dz, dw);

             // Passive Physics (Drag & Pressure)
             let mut passive_force = gravity;
             // Push away from high density (Pressure)
             passive_force = passive_force.add(gradient.scale(-pressure_strength));
             // Agitation
             passive_force = passive_force.add(Vec4::new(
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
                rand::gen_range(-1.0, 1.0),
             ).scale(agitation_strength));

             agent.acc = passive_force;

             // Active Logic (Brain)
             agent.update(density, gradient, dt);

             // Integration
             agent.vel = agent.vel.add(agent.acc.scale(dt));
             agent.vel = agent.vel.scale(viscosity_damping); // Damping
             agent.pos = agent.pos.add(agent.vel.scale(dt));

             // Boundary Constraints
             let bounds = 1.5;
             if agent.pos.x < -bounds || agent.pos.x > bounds { agent.vel.x *= -0.8; agent.pos.x = agent.pos.x.clamp(-bounds, bounds); }
             if agent.pos.y < -bounds || agent.pos.y > bounds { agent.vel.y *= -0.8; agent.pos.y = agent.pos.y.clamp(-bounds, bounds); }
             if agent.pos.z < -bounds || agent.pos.z > bounds { agent.vel.z *= -0.8; agent.pos.z = agent.pos.z.clamp(-bounds, bounds); }
             if agent.pos.w < -bounds || agent.pos.w > bounds { agent.vel.w *= -0.8; agent.pos.w = agent.pos.w.clamp(-bounds, bounds); }
        }

        // --- Rendering ---

        // Input Camera
        if is_key_down(KeyCode::Left) { cam_angle_y += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 2.0 * dt; }
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

        // Auto-Rotate 4D
        let rot_speed = 0.2 * (1.0 + monitor.load_avg);
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.7;

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Transform Helper
        let transform = |v: Vec4| -> Vec3 {
            // No scale distortion for fluid, just rotation
            let mut v = v;
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v.project_to_3d(3.0)
        };

        // Draw Hypercube Edges
        for &(i, j) in &edges {
            let p1 = transform(base_verts[i].scale(1.5)); // Scale up bounds
            let p2 = transform(base_verts[j].scale(1.5));
            draw_line_3d(p1, p2, Color::new(0.3, 0.3, 0.3, 0.5));
        }

        // Draw Particles
        for p in &particles {
            let pos_3d = transform(p.pos);
            let w_norm = (p.pos.w + 1.5) / 3.0;
            let color = Color::new(
                0.2 + w_norm * 0.3, // R
                0.5 + monitor.cpu_usage * 0.5, // G (Agitation)
                1.0 - monitor.mem_usage * 0.5, // B (Viscosity)
                0.5 // More transparent
            );
            draw_sphere(pos_3d, 0.03, None, color);
        }

        // Draw Agents
        for a in &agents {
            let pos_3d = transform(a.pos);
            draw_sphere(pos_3d, 0.08, None, a.color);
        }

        set_default_camera();

        // UI Overlay
        draw_text("Chimera Fluid", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", NUM_AGENTS), 10.0, 40.0, 20.0, YELLOW);
        draw_text(&format!("CPU (Agitation): {:.0}%", monitor.cpu_usage * 100.0), 10.0, 60.0, 20.0, GREEN);
        draw_text(&format!("MEM (Viscosity): {:.0}%", monitor.mem_usage * 100.0), 10.0, 80.0, 20.0, BLUE);
        draw_text(&format!("SWP (Gravity):   {:.0}%", monitor.swap_usage * 100.0), 10.0, 100.0, 20.0, RED);
        draw_text(&format!("Particles: {}", NUM_PARTICLES), 10.0, 120.0, 20.0, GRAY);

        next_frame().await
    }
}
