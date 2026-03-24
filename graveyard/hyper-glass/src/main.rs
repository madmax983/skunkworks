use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;

mod grid;
use grid::SpinGrid4D;

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

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
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
            cam_dist -= 5.0 * dt;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 5.0 * dt;
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
        draw_text(
            &format!("Field (RAM): {:.2}", field),
            10.0,
            70.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Spins: {}", grid.spins.len()),
            10.0,
            90.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
