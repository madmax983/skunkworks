use crossbeam_channel::{bounded, unbounded};
use hyper_system::math::{HyperVector, Vec4};
use hyper_system::monitor::SystemMonitor;
use macroquad::prelude::*;
use std::time::Instant;

mod audio;
mod grid;

use audio::{init_audio, AudioCommand, AudioSnapshot};
use grid::GRID_SIZE;

#[macroquad::main("Hyper Acoustics")]
async fn main() {
    let (cmd_tx, cmd_rx) = unbounded();
    let (snap_tx, snap_rx) = bounded(2);

    // Init Audio
    let _audio_sys = match init_audio(cmd_rx, snap_tx) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Audio init failed: {}", e);
            return;
        }
    };

    let mut monitor = SystemMonitor::new();
    let mut snapshot: Option<AudioSnapshot> = None;

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    // Visualization Grid Positions (Normalized -1 to 1)
    let mut grid_points = Vec::with_capacity(GRID_SIZE * GRID_SIZE * GRID_SIZE * GRID_SIZE);
    for w in 0..GRID_SIZE {
        for z in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    // Map 0..SIZE to -1.0..1.0
                    let fx = (x as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fy = (y as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fz = (z as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    let fw = (w as f32 / (GRID_SIZE - 1) as f32) * 2.0 - 1.0;
                    grid_points.push(Vec4::new(fx, fy, fz, fw));
                }
            }
        }
    }

    let mut _last_pluck = Instant::now();

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Update Physics Parameters based on System Load
        // CPU -> Wave Speed (Higher load = slower waves? Or faster chaos?)
        // Let's say higher load = faster waves (overclocked)
        // RAM -> Damping (More memory = more viscosity/damping)
        let c2 = 0.1 + monitor.cpu_usage * 0.4; // 0.1 to 0.5
        let damping = 0.99 - monitor.mem_usage * 0.1; // 0.99 to 0.89

        let _ = cmd_tx.send(AudioCommand::SetParams { c2, damping });

        // Auto Pluck every few seconds if quiet?
        // Or random plucks based on Swap (Gravity/Random events)
        if rand::gen_range(0.0, 1.0) < monitor.swap_usage * 0.1 {
            let x = rand::gen_range(1, GRID_SIZE - 1);
            let y = rand::gen_range(1, GRID_SIZE - 1);
            let z = rand::gen_range(1, GRID_SIZE - 1);
            let w = rand::gen_range(1, GRID_SIZE - 1);
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x,
                y,
                z,
                w,
                strength: 1.0,
            });
        }

        // Receive Snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            snapshot = Some(snap);
        }

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let x = rand::gen_range(1, GRID_SIZE - 1);
            let y = rand::gen_range(1, GRID_SIZE - 1);
            let z = rand::gen_range(1, GRID_SIZE - 1);
            let w = rand::gen_range(1, GRID_SIZE - 1);
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x,
                y,
                z,
                w,
                strength: 2.0,
            });
        }

        // 4D Rotation (Auto rotate based on load)
        angle_xw += dt * (0.1 + monitor.load_avg);
        angle_yw += dt * (0.15 + monitor.load_avg * 0.5);
        angle_zw += dt * (0.05 + monitor.load_avg * 0.2);

        // Camera Control
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

        // Draw Grid Points
        if let Some(snap) = &snapshot {
            for (i, point) in grid_points.iter().enumerate() {
                if i >= snap.u.len() {
                    break;
                }
                let pressure = snap.u[i];
                let energy = pressure.abs();

                // Only draw significant points
                if energy > 0.01 {
                    // 4D Transform
                    let mut p = *point;
                    p = p.rotate_xw(angle_xw);
                    p = p.rotate_yw(angle_yw);
                    p = p.rotate_zw(angle_zw);
                    let pos3d = p.project_to_3d(3.0);

                    // Color based on pressure
                    let color = if pressure > 0.0 {
                        Color::new(1.0, 1.0 - pressure.min(1.0), 1.0 - pressure.min(1.0), 1.0)
                    // Reddish
                    } else {
                        Color::new(1.0 - energy.min(1.0), 1.0 - energy.min(1.0), 1.0, 1.0)
                        // Bluish
                    };

                    draw_sphere(pos3d.into(), 0.02 + energy * 0.05, None, color);
                }
            }
        }

        set_default_camera();

        // UI
        draw_text("Hyper Acoustics", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("CPU: {:.2} (Speed)", monitor.cpu_usage),
            10.0,
            50.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("RAM: {:.2} (Damping)", monitor.mem_usage),
            10.0,
            70.0,
            20.0,
            BLUE,
        );
        draw_text(
            &format!("Cells: {}^4 = {}", GRID_SIZE, GRID_SIZE.pow(4)),
            10.0,
            90.0,
            20.0,
            GRAY,
        );
        draw_text("Left Click to Pluck 4D Space", 10.0, 110.0, 20.0, YELLOW);

        next_frame().await;
    }
}
