use hyper_system::math::Vec4;
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;

mod grid;

use grid::{Grid4D, Particle};

#[macroquad::main("Hyper Market")]
async fn main() {
    let grid_size = 6; // 6^4 = 1296 cells
    let mut grid = Grid4D::new(grid_size);
    let mut monitor = SystemMonitor::new();

    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    // Precompute vertex positions
    let mut base_points = Vec::new();
    for i in 0..grid.hypersize {
        for j in 0..grid.depth {
            for k in 0..grid.height {
                for l in 0..grid.width {
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

        // Spawn Bids/Asks randomly at bottom/top edges (in 4D sense)
        // Spawn randomly based on load?
        if rand::gen_range(0.0, 1.0) < 0.2 + monitor.load_avg {
            // Spawn Bid at Y=Height-1 (Wait, logic says Y=0 is High Price, so Bids start at Y=Height-1)
            // But logic update says Bids move Y-1 (Up). So Y=Height-1 is Low Price. Correct.
            let x = rand::gen_range(0, grid.width);
            let z = rand::gen_range(0, grid.depth);
            let w = rand::gen_range(0, grid.hypersize);
            grid.set(x, grid.height - 1, z, w, Particle::Bid(rand::gen_range(0, 10000)));
        }

        if rand::gen_range(0.0, 1.0) < 0.2 + monitor.load_avg {
             // Spawn Ask at Y=0 (High Price)
            let x = rand::gen_range(0, grid.width);
            let z = rand::gen_range(0, grid.depth);
            let w = rand::gen_range(0, grid.hypersize);
            grid.set(x, 0, z, w, Particle::Ask(rand::gen_range(0, 10000)));
        }

        // Run simulation
        let volatility = monitor.cpu_usage;
        grid.update(volatility);

        // Rotate
        let rot_speed = 0.2 + monitor.load_avg;
        angle_xw += dt * rot_speed;
        angle_yw += dt * rot_speed * 0.5;
        angle_zw += dt * rot_speed * 0.2;

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

        // Distortion based on system load
        let sx = 1.0 + monitor.cpu_usage * 0.5;
        let sy = 1.0 + monitor.mem_usage * 0.5; // Price axis stretched by memory?
        let sz = 1.0 + monitor.swap_usage * 0.5;
        let sw = 1.0 + (get_time() as f32 * 2.0).sin() * 0.1;

        // Render
        for (idx, particle) in grid.cells.iter().enumerate() {
            if matches!(particle, Particle::Empty) { continue; }

            let point = base_points[idx]; // idx matches iteration order

            // Transform
            let mut v = point.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            let p3 = v.project_to_3d(3.0);

            let (color, size) = match particle {
                Particle::Bid(_) => (GREEN, 0.15),
                Particle::Ask(_) => (RED, 0.15),
                Particle::Trade { age } => {
                    let alpha = *age as f32 / 10.0;
                    (Color::new(1.0, 1.0, 0.0, alpha), 0.3)
                },
                Particle::Empty => (BLACK, 0.0),
            };

            draw_sphere(p3, size, None, color);
        }

        set_default_camera();

        // UI
        draw_text("Hyper Market", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Volatility (CPU): {:.2}", volatility), 10.0, 50.0, 20.0, RED);
        draw_text(&format!("Liquidity (RAM): {:.2}", monitor.mem_usage), 10.0, 70.0, 20.0, BLUE);
        draw_text(&format!("Trades: {}", grid.trade_count), 10.0, 90.0, 20.0, YELLOW);
        draw_text(&format!("Bids: {} | Asks: {}", grid.active_bids, grid.active_asks), 10.0, 110.0, 20.0, GRAY);

        next_frame().await
    }
}
