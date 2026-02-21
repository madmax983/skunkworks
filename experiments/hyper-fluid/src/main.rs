mod physics;

use macroquad::prelude::*;
use physics::{PhysicsParams, Universe, Vec4};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

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
                RefreshKind::nothing()
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
            self.sys.refresh_cpu_all();
            self.sys.refresh_memory();
            self.last_update = now;

            self.target_cpu = self.sys.global_cpu_usage() / 100.0;

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

// Project 4D point to 3D space
fn project_to_3d(v: Vec4, camera_w: f32) -> Vec3 {
    let w_dist = camera_w - v.w;
    let scale = 2.0 / w_dist.max(0.1);
    vec3(v.x * scale, v.y * scale, v.z * scale)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Hyper-Fluid".to_string(),
        window_width: 1024,
        window_height: 768,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut monitor = SystemMonitor::new();
    let universe_size = 60.0;
    let mut universe = Universe::new(universe_size, 1500);

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 80.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // 1. Update Physics
        let params = PhysicsParams {
            gravity_strength: 5.0 + monitor.swap_usage * 50.0,
            viscosity: 0.95 - monitor.mem_usage * 0.1, // High mem = more viscous (lower damping factor?) wait, viscosity factor < 1 is damping. Lower factor = more damping.
            // Let's say: 0.99 is low damping (water), 0.90 is high damping (honey).
            // So High Mem -> 0.90. Low Mem -> 0.99.
            // 0.99 - mem * 0.1 => 0.89 at full mem. Correct.
            temperature: 0.5 + monitor.cpu_usage * 10.0,
        };
        universe.update(dt.min(0.05), params);

        // 2. Rotate
        let rot_speed = 0.2 + monitor.load_avg;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * rot_speed * 0.25;

        // 3. Input
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
            cam_dist -= 20.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 20.0 * dt;
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

        // 4. Render
        // Center shift
        let offset = universe_size / 2.0;

        // Metric Distortion
        let sx = 1.0 + monitor.cpu_usage * 0.5;
        let sy = 1.0 + monitor.mem_usage * 0.5;
        let sz = 1.0 + monitor.swap_usage * 0.5;
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.2; // Breathing

        for p in &universe.particles {
            let mut v = Vec4::new(
                p.pos.x - offset,
                p.pos.y - offset,
                p.pos.z - offset,
                p.pos.w - offset,
            );

            // Deform
            v = v.scale_dim(sx, sy, sz, 1.0 + sw);

            // Rotate
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);

            // Project
            let pos3 = project_to_3d(v, 4.0);

            // Color based on W
            // W is originally [0, size]. Centered [-size/2, size/2].
            // After rotation, W is mixed.
            // Let's use the projected W depth for color to give depth cue.
            // Or use the original W?
            // Using current v.w (depth in 4D view) is good.
            let depth = (v.w + offset) / universe_size; // approx 0..1

            let col = Color::new(
                depth.clamp(0.0, 1.0),
                0.2 + monitor.cpu_usage, // Green gets brighter with CPU
                1.0 - depth.clamp(0.0, 1.0),
                0.8,
            );

            draw_sphere(pos3, 0.4, None, col);
        }

        // Draw Tesseract Bounds
        let s = offset;
        for i in 0..16 {
            for j in (i + 1)..16 {
                let diff: usize = i ^ j;
                if diff.count_ones() == 1 {
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
                        project_to_3d(v, 4.0)
                    };
                    draw_line_3d(get_pos(i), get_pos(j), Color::new(0.3, 0.3, 0.3, 0.3));
                }
            }
        }

        set_default_camera();

        draw_text("HYPER-FLUID", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Particles: {}", universe.particles.len()),
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );

        draw_text(
            &format!("CPU (Temp): {:.0}%", monitor.cpu_usage * 100.0),
            10.0,
            90.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM (Visc): {:.0}%", monitor.mem_usage * 100.0),
            10.0,
            110.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("SWP (Grav): {:.0}%", monitor.swap_usage * 100.0),
            10.0,
            130.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!("LOD (Scale): {:.2}", monitor.load_avg),
            10.0,
            150.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}
