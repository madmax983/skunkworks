use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use physics_pbd::{Constraint, PbdSystem};
use poincare_disk::{hyperbolic_dist, Point};

// Constants
const GRID_COLS: usize = 12;
const GRID_ROWS: usize = 12;

#[macroquad::main("Poincaré Origami")]
async fn main() {
    let mut system = PbdSystem::new();

    // Miura-ori setup
    let params = MiuraParams {
        a: 0.1,
        b: 0.1,
        gamma: 60.0_f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    // Original 3D structure -> flat array
    let points = generate_miura_grid(params, (GRID_COLS, GRID_ROWS), 0.8);

    // Map initial points into the Poincaré disk roughly
    let mut disk_points = Vec::new();
    for p in points.iter() {
        // Project 3D (x,y,z) into 2D disk (u,v). We squash Z and scale to fit unit disk.
        let u = (p.x * 2.0).clamp(-0.9, 0.9);
        let v = (p.y * 2.0).clamp(-0.9, 0.9);

        // Ensure within unit disk
        let r = (u * u + v * v).sqrt();
        let (du, dv) = if r > 0.95 {
            (u * 0.95 / r, v * 0.95 / r)
        } else {
            (u, v)
        };

        let p_idx = system.add_particle(vec3(du, dv, 0.0), 1.0);
        disk_points.push(p_idx);
    }

    // Setup structural constraints inside the disk using hyperbolic distance
    for j in 0..GRID_ROWS {
        for i in 0..GRID_COLS {
            let idx = j * (GRID_COLS + 1) + i;
            let right = idx + 1;
            let down = (j + 1) * (GRID_COLS + 1) + i;

            // Euclidean distance in 3D is original constraint.
            // In the Poincaré disk, the distance must be scaled by hyperbolic geometry.
            let orig_len_x = params.a;
            let orig_len_y = params.b;

            system.add_distance_constraint(idx, right, orig_len_x * 0.5);
            system.add_distance_constraint(idx, down, orig_len_y * 0.5);

            // Shear constraint
            let diag = right + (GRID_COLS + 1);
            let diag_len = (orig_len_x.powi(2) + orig_len_y.powi(2)).sqrt() * 0.5;
            if j < GRID_ROWS && i < GRID_COLS {
                system.add_distance_constraint(idx, diag, diag_len);
            }
        }
    }

    let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let scale = 300.0;

    let mut time = 0.0_f32;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        time += get_frame_time();

        // PBD step
        system.step(get_frame_time().min(0.05), 10);

        // Draw disk boundary
        draw_circle_lines(center.x, center.y, scale, 2.0, GRAY);

        // Hyperbolic translation center (moving point)
        let cx = (time * 0.5).cos() * 0.6;
        let cy = (time * 0.3).sin() * 0.6;
        let center_pt = Point::new(cx as f64, cy as f64);
        draw_circle(center.x + cx * scale, center.y + cy * scale, 5.0, RED);

        // For each constraint, actuate it based on hyperbolic geometry
        // As elements move further from the center of the disk, the hyperbolic distance increases rapidly,
        // so we warp the structural distance constraint to compress physically near the boundary.
        for c in &mut system.constraints {
            if let Constraint::Distance {
                p1,
                p2,
                rest_length,
                ..
            } = c
            {
                let p1_pos = system.particles[*p1].pos;
                let p2_pos = system.particles[*p2].pos;

                let p1_pd = Point::new(p1_pos.x as f64, p1_pos.y as f64);
                let p2_pd = Point::new(p2_pos.x as f64, p2_pos.y as f64);

                // Hyperbolic distance from the center attractor
                let d1 = hyperbolic_dist(p1_pd, center_pt);
                let d2 = hyperbolic_dist(p2_pd, center_pt);

                let avg_dist = (d1 + d2) * 0.5;

                // Warp the rest length based on the hyperbolic space curvature.
                // Things far away (high hyperbolic distance) shrink relative to Euclidean space
                *rest_length = (0.05 * (1.0 / (1.0 + avg_dist as f32))).clamp(0.01, 0.1);

                // Prevent particles from escaping disk
                let r1 = (p1_pos.x * p1_pos.x + p1_pos.y * p1_pos.y).sqrt();
                if r1 > 0.99 {
                    system.particles[*p1].pos = p1_pos.normalize() * 0.99;
                }
            }
        }

        // Draw mesh
        for j in 0..GRID_ROWS {
            for i in 0..GRID_COLS {
                let idx = j * (GRID_COLS + 1) + i;
                let right = idx + 1;
                let down = (j + 1) * (GRID_COLS + 1) + i;

                let pos1 = system.particles[idx].pos;
                let v1 = vec2(center.x + pos1.x * scale, center.y + pos1.y * scale);

                if i < GRID_COLS {
                    let pos2 = system.particles[right].pos;
                    let v2 = vec2(center.x + pos2.x * scale, center.y + pos2.y * scale);
                    draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, WHITE);
                }
                if j < GRID_ROWS {
                    let pos3 = system.particles[down].pos;
                    let v3 = vec2(center.x + pos3.x * scale, center.y + pos3.y * scale);
                    draw_line(v1.x, v1.y, v3.x, v3.y, 1.0, WHITE);
                }
            }
        }

        next_frame().await;
    }
}
