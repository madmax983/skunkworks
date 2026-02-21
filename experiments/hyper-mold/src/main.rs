use macroquad::prelude::*;
use rayon::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

mod agent;
mod grid;
mod math;

use agent::Agent;
use grid::Grid4D;
use math::Vec4;

struct SystemMonitor {
    sys: System,
    last_update: f64,
    cpu_usage: f32,
    mem_usage: f32,
    swap_usage: f32,
    load_avg: f32,
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

        let dt = get_frame_time();
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let speed = 2.0 * dt;

        self.cpu_usage = lerp(self.cpu_usage, self.target_cpu, speed);
        self.mem_usage = lerp(self.mem_usage, self.target_mem, speed);
        self.swap_usage = lerp(self.swap_usage, self.target_swap, speed);
        self.load_avg = lerp(self.load_avg, self.target_load, speed);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyper-Mold".to_string(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Parameters
    let grid_size = 24; // 24^4 = 331,776 cells
    let num_agents = 5000;

    let mut grid = Grid4D::new(grid_size);
    let mut agents: Vec<Agent> = (0..num_agents)
        .map(|_| {
            let mut rng = ::rand::thread_rng();
            use ::rand::Rng;
            Agent::new(Vec4::new(
                rng.gen_range(0.0..grid_size as f32),
                rng.gen_range(0.0..grid_size as f32),
                rng.gen_range(0.0..grid_size as f32),
                rng.gen_range(0.0..grid_size as f32),
            ))
        })
        .collect();

    let mut monitor = SystemMonitor::new();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 40.0f32;

    // 4D Rotation State
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // 1. Update Agents
        // Parallel update logic is a bit tricky with macroquad main loop + rayon
        // We can just iterate normally or use par_iter if we are careful.
        // Since Agent update only reads grid, we can par_iter agents.

        // However, grid is not Sync if we modify it?
        // Agent::update reads grid. So grid must be immutable.
        // We update agent state (pos/vel) in place.
        agents.par_iter_mut().for_each(|agent| {
            agent.update(&grid, 1.5, 0.1, 0.5);
        });

        // 2. Deposit
        // Agents deposit pheromones. This is serial to avoid race conditions on grid cells.
        // Or we use AtomicFloat? Too complex. Serial is fast enough for 5000 agents.
        for agent in &agents {
            grid.add_val(
                agent.pos.x as usize,
                agent.pos.y as usize,
                agent.pos.z as usize,
                agent.pos.w as usize,
                0.5, // Deposit amount
            );
        }

        // 3. Diffuse & Decay
        // Run every frame? Or every N frames?
        // 331k cells is fast.
        grid.diffuse_and_decay(0.02, 0.1); // decay, diffuse

        // 4. Update Rotation
        let rot_speed = 0.2 * (1.0 + monitor.load_avg);
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * rot_speed * 0.25;

        // Input
        if is_key_down(KeyCode::Left) {
            cam_angle_y += 2.0 * dt;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x += 2.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x -= 2.0 * dt;
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 10.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 10.0 * dt;
        }

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

        // Render Agents
        // We need to transform their 4D positions to 3D points
        // The transformation depends on system metrics (distortion)

        let sx = 1.0 + monitor.cpu_usage; // Stretch X
        let sy = 1.0 + monitor.mem_usage; // Stretch Y
        let sz = 1.0 + monitor.swap_usage; // Stretch Z
        let sw = 1.0 + monitor.load_avg; // Stretch W

        // Center the grid: Agents are in [0, 24]. Shift to [-12, 12].
        let offset = grid_size as f32 / 2.0;

        for agent in &agents {
            let mut v = Vec4::new(
                agent.pos.x - offset,
                agent.pos.y - offset,
                agent.pos.z - offset,
                agent.pos.w - offset,
            );

            // Apply Metric Distortion
            v = v.scale_dim(sx, sy, sz, 1.0 + sw);

            // Apply Global Rotation
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);

            // Project to 3D
            let p = v.project_to_3d(4.0); // Camera W distance

            // Color based on W (before rotation/projection? or after?)
            // Use original W for color to see "depth" in 4th dimension
            let w_norm = agent.pos.w / grid_size as f32;
            let col = Color::new(
                w_norm,                 // Red
                1.0 - w_norm,           // Green
                (w_norm * 2.0).fract(), // Blue
                0.8,
            );

            draw_sphere(p, 0.1, None, col);
        }

        // Draw Tesseract Bounds Wireframe?
        // Just draw the 16 vertices of the bounding box
        // Bounding box is [-offset, offset]
        let s = offset;
        for i in 0..16 {
            let x = if i & 1 != 0 { s } else { -s };
            let y = if i & 2 != 0 { s } else { -s };
            let z = if i & 4 != 0 { s } else { -s };
            let w = if i & 8 != 0 { s } else { -s };

            let mut v = Vec4::new(x, y, z, w);
            v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            let p = v.project_to_3d(4.0);

            draw_sphere(p, 0.2, None, WHITE);
        }

        // Draw Edges of bounding box
        // Iterate all pairs, if hamming distance is 1, draw line
        for i in 0..16 {
            for j in (i + 1)..16 {
                let diff: usize = i ^ j;
                if diff.count_ones() == 1 {
                    // Calculate positions again (inefficient but simple)
                    let get_pos = |idx: usize| -> Vec3 {
                        let x = if idx & 1 != 0 { s } else { -s };
                        let y = if idx & 2 != 0 { s } else { -s };
                        let z = if idx & 4 != 0 { s } else { -s };
                        let w = if idx & 8 != 0 { s } else { -s };
                        let mut v = Vec4::new(x, y, z, w);
                        v = v.scale_dim(sx, sy, sz, 1.0 + sw);
                        v = v.rotate_xw(angle_xw);
                        v = v.rotate_yw(angle_yw);
                        v = v.rotate_zw(angle_zw);
                        v.project_to_3d(4.0)
                    };
                    draw_line_3d(get_pos(i), get_pos(j), Color::new(0.3, 0.3, 0.3, 0.5));
                }
            }
        }

        set_default_camera();
        draw_text("HYPER-MOLD", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Agents: {}", num_agents),
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );

        draw_text(
            &format!("CPU (X): {:.0}%", monitor.cpu_usage * 100.0),
            10.0,
            90.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM (Y): {:.0}%", monitor.mem_usage * 100.0),
            10.0,
            110.0,
            20.0,
            GREEN,
        );
        draw_text(
            &format!("SWP (Z): {:.0}%", monitor.swap_usage * 100.0),
            10.0,
            130.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("LOD (W): {:.2}", monitor.load_avg),
            10.0,
            150.0,
            20.0,
            YELLOW,
        );

        next_frame().await
    }
}
