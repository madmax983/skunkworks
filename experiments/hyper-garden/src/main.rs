mod lsystem;
mod math;
mod monitor;
mod sexagesimal;
mod turtle;

use lsystem::LSystem;
use math::Vec4;
use monitor::SystemMonitor;
use sexagesimal::Sexagesimal;
use turtle::{Line4D, Turtle};

use macroquad::prelude::*;
use std::process::Command;

struct Plant {
    hash: String,
    message: String,
    lines: Vec<Line4D>,
    color: Color,
}

fn harvest_plants() -> Vec<Plant> {
    let output = Command::new("git")
        .args(&["log", "-n", "5", "--pretty=format:%H|%s"])
        .output();

    let mut plants = Vec::new();

    if let Ok(output) = output {
        let stdout = String::from_utf8(output.stdout).unwrap_or_default();
        for (i, line) in stdout.lines().enumerate() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 2 {
                continue;
            }
            let hash = parts[0];
            let message = parts[1];

            let hash_prefix = &hash[0..8];
            let val = u64::from_str_radix(hash_prefix, 16).unwrap_or(0);
            let sexagesimal = Sexagesimal::from_u64(val);

            // Generate L-System rules from Sexagesimal digits
            let mut rule_str = "F".to_string();
            for &digit in &sexagesimal.digits {
                match digit % 6 {
                    0 => rule_str.push_str("[+F]"),
                    1 => rule_str.push_str("[-F]"),
                    2 => rule_str.push_str("[&F]"), // Pitch
                    3 => rule_str.push_str("[^F]"),
                    4 => rule_str.push_str("[>F]"), // Hyper-Yaw
                    5 => rule_str.push_str("[<F]"),
                    _ => {}
                }
                if digit > 30 {
                    rule_str.push_str("F");
                }
            }

            // Axiom and Rules
            let axiom = "X";
            let x_rule = format!("F-[[X]+X]+F[+FX]-X"); // Standard plant-like
                                                        // Mix in the generated rule
            let f_rule = rule_str;

            let rules = vec![('X', x_rule.as_str()), ('F', f_rule.as_str())];
            let lsystem = LSystem::new(axiom, rules);

            // Expand
            let instructions = lsystem.expand(4);

            // Interpret to 4D Lines
            // Angle based on hash too
            let angle = (20.0 + (val % 20) as f32).to_radians();
            let start_pos = Vec4::new((i as f32 - 2.0) * 2.0, -2.0, 0.0, 0.0);
            let start_dir = Vec4::new(0.0, 1.0, 0.0, 0.0); // Up

            let mut turtle = Turtle::new(start_pos, start_dir, 0.1, angle);
            let lines = turtle.interpret(&instructions);

            // Color based on hash
            let r = ((val >> 16) & 0xFF) as f32 / 255.0;
            let g = ((val >> 8) & 0xFF) as f32 / 255.0;
            let b = (val & 0xFF) as f32 / 255.0;

            plants.push(Plant {
                hash: hash.to_string(),
                message: message.to_string(),
                lines,
                color: Color::new(r, g, b, 0.8),
            });
        }
    }

    // Fallback if no git (e.g. fresh container)
    if plants.is_empty() {
        let lsystem = LSystem::new("X", vec![('X', "F-[[X]+X]+F[+FX]-X"), ('F', "FF")]);
        let instructions = lsystem.expand(4);
        let mut turtle = Turtle::new(
            Vec4::new(0.0, -2.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            0.1,
            25.0f32.to_radians(),
        );
        let lines = turtle.interpret(&instructions);
        plants.push(Plant {
            hash: "000000".to_string(),
            message: "No Git Data".to_string(),
            lines,
            color: GREEN,
        });
    }

    plants
}

#[macroquad::main("Hyper Garden")]
async fn main() {
    let plants = harvest_plants();
    let mut monitor = SystemMonitor::new();

    // Camera
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;
    let mut cam_dist = 8.0f32;

    // 4D Rotation
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;

    loop {
        monitor.update();
        let dt = get_frame_time();

        // Rotate based on load
        let base_speed = 0.1;
        let speed_mult = 1.0 + monitor.load_avg * 2.0;
        angle_xw += dt * base_speed * speed_mult;
        angle_yw += dt * base_speed * 0.5 * speed_mult;
        angle_zw += dt * base_speed * 0.2;

        // Input Camera
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

        // Transform Logic
        let sx = 1.0 + monitor.cpu_usage; // CPU stretches X
        let sy = 1.0 + monitor.mem_usage; // Mem stretches Y
        let sz = 1.0 + monitor.swap_usage; // Swap stretches Z
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.5; // Breathing W

        let transform = |v: Vec4| -> Vec3 {
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v.project_to_3d(4.0)
        };

        // Draw Plants
        for plant in &plants {
            for line in &plant.lines {
                let p1 = transform(line.start);
                let p2 = transform(line.end);

                // Color modulation by depth/w?
                // Fade distant points
                draw_line_3d(p1, p2, plant.color);
            }

            // Draw a base for the plant
            if let Some(first) = plant.lines.first() {
                let base = transform(first.start);
                draw_sphere(base, 0.1, None, WHITE);
            }
        }

        // Draw HUD
        set_default_camera();
        draw_text("Hyper Garden", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("CPU: {:.0}%", monitor.cpu_usage * 100.0),
            10.0,
            50.0,
            20.0,
            RED,
        );
        draw_text(
            &format!("MEM: {:.0}%", monitor.mem_usage * 100.0),
            10.0,
            70.0,
            20.0,
            BLUE,
        );

        // Show current plant info
        let y_start = 100.0;
        for (i, plant) in plants.iter().enumerate() {
            draw_text(
                &format!(
                    "{}: {}",
                    plant.hash.chars().take(7).collect::<String>(),
                    plant.message
                ),
                10.0,
                y_start + i as f32 * 20.0,
                15.0,
                plant.color,
            );
        }

        next_frame().await
    }
}
