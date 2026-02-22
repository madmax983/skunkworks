use macroquad::prelude::*;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

mod grid;
use grid::SpinGrid4D;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        Self {
            x: self.x * sx,
            y: self.y * sy,
            z: self.z * sz,
            w: self.w * sw,
        }
    }

    pub fn rotate_xw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x * c - self.w * s,
            y: self.y,
            z: self.z,
            w: self.x * s + self.w * c,
        }
    }

    pub fn rotate_yw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y * c - self.w * s,
            z: self.z,
            w: self.y * s + self.w * c,
        }
    }

    pub fn rotate_zw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y,
            z: self.z * c - self.w * s,
            w: self.z * s + self.w * c,
        }
    }

    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        let scale = 2.0 / w_dist.max(0.1);
        vec3(self.x * scale, self.y * scale, self.z * scale)
    }
}

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
            self.target_mem = if total_mem > 0.0 { used_mem / total_mem } else { 0.0 };

            let total_swap = self.sys.total_swap() as f32;
            let used_swap = self.sys.used_swap() as f32;
            self.target_swap = if total_swap > 0.0 { used_swap / total_swap } else { 0.0 };

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

// Convert angle (0-2PI) to Color (Hue)
fn angle_to_color(theta: f32) -> Color {
    let normalized = theta.rem_euclid(2.0 * std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
    // HSL: Hue = theta, Saturation = 1.0, Lightness = 0.5
    hsl_to_rgb(normalized, 1.0, 0.5)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    // Simple HSL to RGB conversion
    // Macroquad doesn't seem to expose one publicly? Or maybe it does.
    // Let's implement one to be safe.
    // Or use h,s,l from 0.0 to 1.0

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0/6.0 {
        (c, x, 0.0)
    } else if h < 2.0/6.0 {
        (x, c, 0.0)
    } else if h < 3.0/6.0 {
        (0.0, c, x)
    } else if h < 4.0/6.0 {
        (0.0, x, c)
    } else if h < 5.0/6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}

#[macroquad::main("Hyper Glass")]
async fn main() {
    let grid_size = 6; // 6^4 = 1296 cells. Manageable. 8^4 = 4096.
    let mut grid = SpinGrid4D::new(grid_size);
    let mut monitor = SystemMonitor::new();

    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let angle_zw = 0.0;

    // Precompute vertex positions (Grid coords centered at 0)
    // Normalized to [-1, 1]
    let mut base_points = Vec::new();
    for i in 0..grid.hypersize {
        for j in 0..grid.depth {
            for k in 0..grid.height {
                for l in 0..grid.width {
                    // Map 0..size to -1..1
                    let x = (l as f32 / (grid.width - 1) as f32) * 2.0 - 1.0;
                    let y = (k as f32 / (grid.height - 1) as f32) * 2.0 - 1.0;
                    let z = (j as f32 / (grid.depth - 1) as f32) * 2.0 - 1.0;
                    let w = (i as f32 / (grid.hypersize - 1) as f32) * 2.0 - 1.0;
                    base_points.push(Vec4::new(x, y, z, w));
                }
            }
        }
    }

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Physics Step
        // Temp = CPU Usage (0.0 - 1.0) * Scale
        // Field = RAM Usage (0.0 - 1.0) * Scale

        // Critical Temp for XY model is ~0.89 (2D). In 4D? Probably higher.
        // Let's scale T from 0.0 to 5.0
        let temp = monitor.cpu_usage * 5.0;

        // Field forces alignment. RAM usage.
        let field = monitor.mem_usage * 2.0;

        // Step multiple times per frame for speed?
        // 1296 spins, one pass is fast.
        for _ in 0..5 {
            grid.step_metropolis(temp, field);
        }

        // Rotate
        let rot_speed = 0.2 + monitor.load_avg;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;

        // Camera
         if is_key_down(KeyCode::Left) { cam_angle_y += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_angle_y -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_angle_x += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 2.0 * dt; }
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

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

        // Render
        let sx = 1.0 + monitor.cpu_usage * 0.5;
        let sy = 1.0 + monitor.mem_usage * 0.5;
        let sz = 1.0 + monitor.swap_usage * 0.5;
        let sw = 1.0 + (get_time() as f32 * 2.0).sin() * 0.1; // Breathing

        // Iterate over grid and points
        // base_points and grid.spins are aligned by index construction order
        // Order in grid.rs: w * (whd) + z * (wh) + y * w + x
        // Order in main.rs loop: i(w), j(z), k(y), l(x) -> matches.

        for (idx, point) in base_points.iter().enumerate() {
            let theta = grid.spins[idx];

            // Transform
            let mut v = point.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            let p3 = v.project_to_3d(3.0);

            let color = angle_to_color(theta);

            // Draw
            // Size?
            let size = 0.08;
            draw_sphere(p3, size, None, color);
        }

        // Draw Tesseract Edges (Subset)?
        // Just drawing corner connections might be cool but complex to map indices.
        // Let's stick to point cloud.

        set_default_camera();

        // UI
        draw_text("Hyper Glass", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Temp (CPU): {:.2}", temp), 10.0, 50.0, 20.0, RED);
        draw_text(&format!("Field (RAM): {:.2}", field), 10.0, 70.0, 20.0, BLUE);
        draw_text(&format!("Spins: {}", grid.spins.len()), 10.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}
