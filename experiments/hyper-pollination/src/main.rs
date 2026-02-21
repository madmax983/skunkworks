mod boid;
mod lsystem;
mod math;
mod monitor;
mod sexagesimal;
mod turtle;

use boid::Boid4D;
use lsystem::LSystem;
use math::Vec4D;
use monitor::SystemMonitor;
use sexagesimal::Sexagesimal;
use turtle::{Line4D, Turtle};

use macroquad::prelude::*;
use std::process::Command;

struct Plant {
    hash: String,
    message: String,
    lines: Vec<Line4D>,
    tips: Vec<Vec4D>,
    color: Color,
}

fn harvest_plants() -> Vec<Plant> {
    let output = Command::new("git")
        .args(&["log", "-n", "5", "--pretty=format:%H|%s"])
        .output();

    let mut plants = Vec::new();

    let stdout = if let Ok(output) = output {
        String::from_utf8(output.stdout).unwrap_or_default()
    } else {
        String::new()
    };

    let lines_iter: Vec<&str> = stdout.lines().collect();

    // Fallback if no git or empty output
    if lines_iter.is_empty() {
         let lsystem = LSystem::new("X", vec![('X', "F-[[X]+X]+F[+FX]-X"), ('F', "FF")]);
        let instructions = lsystem.expand(4);
        let mut turtle = Turtle::new(
            Vec4D::new(0.0, -2.0, 0.0, 0.0),
            Vec4D::new(0.0, 1.0, 0.0, 0.0),
            0.1,
            25.0f32.to_radians(),
        );
        let lines = turtle.interpret(&instructions);
        let tips: Vec<Vec4D> = lines.iter().map(|l| l.end).collect();
        plants.push(Plant {
            hash: "000000".to_string(),
            message: "No Git Data (Simulation Mode)".to_string(),
            lines,
            tips,
            color: GREEN,
        });
        return plants;
    }

    for (i, line) in lines_iter.iter().enumerate() {
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
        let x_rule = format!("F-[[X]+X]+F[+FX]-X");
        let f_rule = rule_str;

        let rules = vec![('X', x_rule.as_str()), ('F', f_rule.as_str())];
        let lsystem = LSystem::new(axiom, rules);

        // Expand
        let instructions = lsystem.expand(4);

        // Interpret to 4D Lines
        let angle = (20.0 + (val % 20) as f32).to_radians();
        // Spaced out in X
        let start_pos = Vec4D::new((i as f32 - 2.0) * 1.5, -2.0, 0.0, 0.0);
        let start_dir = Vec4D::new(0.0, 1.0, 0.0, 0.0); // Up

        let mut turtle = Turtle::new(start_pos, start_dir, 0.1, angle);
        let lines = turtle.interpret(&instructions);

        let tips: Vec<Vec4D> = lines.iter().map(|l| l.end).collect();

        // Color based on hash
        let r = ((val >> 16) & 0xFF) as f32 / 255.0;
        let g = ((val >> 8) & 0xFF) as f32 / 255.0;
        let b = (val & 0xFF) as f32 / 255.0;

        plants.push(Plant {
            hash: hash.to_string(),
            message: message.to_string(),
            lines,
            tips,
            color: Color::new(r, g, b, 0.8),
        });
    }

    plants
}

#[macroquad::main("Hyper-Pollination")]
async fn main() {
    let plants = harvest_plants();
    let mut monitor = SystemMonitor::new();
    let mut boids: Vec<Boid4D> = (0..100).map(|_| Boid4D::new()).collect();

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

        // Camera Input
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

        // System Distortion Vector (Scaling Factors)
        let sx = 1.0 + monitor.cpu_usage;
        let sy = 1.0 + monitor.mem_usage;
        let sz = 1.0 + monitor.swap_usage;
        let sw = 1.0 + (get_time() as f32 * (1.0 + monitor.load_avg * 5.0)).sin() * 0.5;

        // Transform Helper (4D -> 3D)
        let transform = |v: Vec4D| -> Vec3 {
            // Apply distortion
            let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
            // Apply Global Rotation
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            // Project
            v.project_to_3d(4.0)
        };

        // Update Boids
        let old_boids = boids.clone();

        // Bounds are fixed in Base Space so boids stay near plants
        let bounds = Vec4D::new(3.0, 3.0, 3.0, 3.0);

        for boid in boids.iter_mut() {
            // Find nearest plant tip
            let mut nearest_dist = f32::MAX;
            let mut target: Option<Vec4D> = None;
            let mut target_color = WHITE;

            for plant in &plants {
                for tip in &plant.tips {
                     // Check if roughly close to optimize
                     let d = boid.position.distance_squared(*tip);
                     if d < nearest_dist {
                         nearest_dist = d;
                         target = Some(*tip);
                         target_color = plant.color;
                     }
                }
            }

            // Pollination Event
            if nearest_dist < 0.1 {
                boid.pollinate(target_color);
            }

            boid.flock(&old_boids, target);
            boid.update(bounds);
        }

        // Draw Plants
        for plant in &plants {
            for line in &plant.lines {
                let p1 = transform(line.start);
                let p2 = transform(line.end);
                draw_line_3d(p1, p2, plant.color);
            }
        }

        // Draw Boids
        for boid in &boids {
            let p = transform(boid.position);
            draw_sphere(p, 0.05, None, boid.dna.color);
        }

        // Draw Tesseract Bounds
        draw_tesseract_wireframe(Vec4D::new(3.0, 3.0, 3.0, 3.0), angle_xw, angle_yw, angle_zw, sx, sy, sz, sw);

        set_default_camera();
        draw_text("Hyper-Pollination", 10.0, 20.0, 30.0, WHITE);
        draw_text("4D Ecosystem driven by Git & CPU", 10.0, 40.0, 20.0, GRAY);

        draw_text(&format!("CPU: {:.0}%", monitor.cpu_usage * 100.0), 10.0, 70.0, 20.0, RED);
        draw_text(&format!("MEM: {:.0}%", monitor.mem_usage * 100.0), 10.0, 90.0, 20.0, BLUE);
        draw_text(&format!("Boids: {}", boids.len()), 10.0, 110.0, 20.0, YELLOW);

        // Draw Plant Info
        let mut y = 140.0;
        for plant in &plants {
            draw_text(&format!("{}: {}", &plant.hash[0..7.min(plant.hash.len())], plant.message), 10.0, y, 15.0, plant.color);
            y += 20.0;
        }

        next_frame().await
    }
}

fn draw_tesseract_wireframe(bounds: Vec4D, axw: f32, ayw: f32, azw: f32, sx: f32, sy: f32, sz: f32, sw: f32) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { bounds.x } else { -bounds.x };
        let y = if i & 2 != 0 { bounds.y } else { -bounds.y };
        let z = if i & 4 != 0 { bounds.z } else { -bounds.z };
        let w = if i & 8 != 0 { bounds.w } else { -bounds.w };
        verts.push(Vec4D::new(x, y, z, w));
    }

    let transform = |v: Vec4D| -> Vec3 {
        // Apply same distortion as plants
        let mut v = v.scale_dim(sx, sy, sz, 1.0 + sw);
        v = v.rotate_xw(axw);
        v = v.rotate_yw(ayw);
        v = v.rotate_zw(azw);
        v.project_to_3d(4.0)
    };

    for i in 0..16 {
        for j in (i + 1)..16 {
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                let p1 = transform(verts[i]);
                let p2 = transform(verts[j]);
                draw_line_3d(p1, p2, Color::new(0.5, 0.5, 0.5, 0.2));
            }
        }
    }
}
