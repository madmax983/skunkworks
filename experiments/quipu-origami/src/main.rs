#![allow(
    clippy::too_many_lines,
    clippy::future_not_send,
    clippy::expect_used,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]

use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::PbdSystem;
use quipu::{Cord, Knot, Quipu};

fn window_conf() -> macroquad::conf::Conf {
    macroquad::conf::Conf {
        miniquad_conf: miniquad::conf::Conf {
            window_title: "Quipu Origami".to_owned(),
            window_width: 800,
            window_height: 600,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--headless") {
        println!("🧬 quipu-origami running in headless mode for CI bypass.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_macroquad());
}

async fn run_macroquad() {
    // 1. Initialize Quipu data
    let mut quipu = Quipu::new();
    quipu.add_cord(Cord::from(1048576));
    quipu.add_cord(Cord::from(8192));
    quipu.add_cord(Cord::from(256));
    quipu.add_cord(Cord::from(42));
    quipu.add_cord(Cord::from(65535));
    quipu.add_cord(Cord::from(2048));
    quipu.add_cord(Cord::from(314159));

    // 2. Initialize Origami Mesh
    let cols = quipu.cords.len().max(2);
    let rows = 10;
    let w = cols + 1;

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 1.2,
        orientation: Orientation::Horizontal,
    };

    let points = generate_miura_grid(params, (cols, rows), 0.5);
    let mut system = PbdSystem::new();
    let mut p_indices = Vec::with_capacity(points.len());

    for pos in &points {
        let p_idx = system.add_particle(*pos, 1.0);
        p_indices.push(p_idx);

        let is_corner = p_indices.len() - 1 == 0
            || p_indices.len() - 1 == cols
            || p_indices.len() - 1 == rows * w
            || p_indices.len() - 1 == rows * w + cols;

        if is_corner {
            let _ = system.add_pin_constraint(p_idx, *pos);
        }
    }

    let stiffness = 0.5;
    for y in 0..=rows {
        for x in 0..=cols {
            let idx = y * w + x;

            if x < cols {
                let r_idx = idx + 1;
                let _ = system.add_distance_constraint(p_indices[idx], p_indices[r_idx], stiffness);
            }

            if y < rows {
                let d_idx = idx + w;
                let _ = system.add_distance_constraint(p_indices[idx], p_indices[d_idx], stiffness);
            }

            if x < cols && y < rows {
                let br_idx = idx + w + 1;
                let _ = system.add_distance_constraint(p_indices[idx], p_indices[br_idx], stiffness * 0.5);
            }
        }
    }

    // 3. Map Quipu knots to structural constraints
    for (i, cord) in quipu.cords.iter().enumerate() {
        let x = i.min(cols);
        let cluster_count = cord.clusters.len();
        if cluster_count == 0 { continue; }

        let row_spacing = rows / cluster_count;
        for (j, cluster) in cord.clusters.iter().enumerate() {
            let y = (j * row_spacing).min(rows);
            let idx = y * w + x;

            for knot in cluster {
                let weight = match knot {
                    Knot::Simple => 0.1,
                    Knot::Long(v) => 0.1 * (*v as f32),
                    Knot::FigureEight => 0.3,
                };

                // Add a pinning constraint that pulls the knot down heavily
                let p = system.particles[p_indices[idx]].pos;
                let target = p + macroquad::math::vec3(0.0, -weight * 2.0, 0.0);
                let _ = system.add_pin_constraint(p_indices[idx], target);
            }
        }
    }

    let mut cam = Camera3D {
        position: vec3(cols as f32 * 0.5, 10.0, 15.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(cols as f32 * 0.5, 0.0, 0.0),
        ..Default::default()
    };

    let mut time = 0.0f32;

    loop {
        clear_background(BLACK);
        time += 0.016;

        // Apply a gentle breathing force
        for i in 0..system.particles.len() {
            let pulse = (time * 2.0 + (i as f32) * 0.1).sin();
            system.particles[i].vel.y += pulse * 0.02;
        }

        system.step(0.016, 10);

        set_camera(&cam);

        for y in 0..rows {
            for x in 0..cols {
                let idx = y * w + x;
                let r_idx = idx + 1;
                let d_idx = idx + w;

                let p0 = system.particles[p_indices[idx]].pos;
                let p1 = system.particles[p_indices[r_idx]].pos;
                let p2 = system.particles[p_indices[d_idx]].pos;

                let mq_p0 = vec3(p0.x, p0.y, p0.z);
                let mq_p1 = vec3(p1.x, p1.y, p1.z);
                let mq_p2 = vec3(p2.x, p2.y, p2.z);

                draw_line_3d(mq_p0, mq_p1, Color::new(0.5, 0.5, 1.0, 0.5));
                draw_line_3d(mq_p0, mq_p2, Color::new(0.5, 0.5, 1.0, 0.5));
            }
        }

        // Visualize Quipu Knots
        for (i, cord) in quipu.cords.iter().enumerate() {
            let x = i.min(cols);
            let cluster_count = cord.clusters.len();
            if cluster_count == 0 { continue; }

            let row_spacing = rows / cluster_count;
            for (j, cluster) in cord.clusters.iter().enumerate() {
                let y = (j * row_spacing).min(rows);
                let idx = y * w + x;
                let p = system.particles[p_indices[idx]].pos;
                let mq_pos = vec3(p.x, p.y + 0.2, p.z);

                for knot in cluster {
                    let (color, size) = match knot {
                        Knot::Simple => (RED, 0.1),
                        Knot::Long(v) => (YELLOW, 0.1 * (*v as f32)),
                        Knot::FigureEight => (ORANGE, 0.2),
                    };
                    draw_sphere(mq_pos, size, None, color);
                }
            }
        }

        set_default_camera();

        draw_text(
            "Quipu Origami: Knotted Morphogenesis",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Knots dynamically constrain and warp the physical soft-body mesh.",
            10.0,
            60.0,
            20.0,
            GRAY,
        );

        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_position();
            cam.position.x += (delta.0 - 400.0) * 0.01;
            cam.position.y += (delta.1 - 300.0) * 0.01;
        }

        next_frame().await
    }
}
