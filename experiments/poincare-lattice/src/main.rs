use macroquad::prelude::*;
use miller_lattice::Crystal;
use poincare_disk::{mobius_add, Point};
use std::collections::HashMap;

#[macroquad::main("Poincaré Lattice")]
async fn main() {
    let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    let mut time = 0.0_f32;

    // Map crystal nodes to Poincare points
    // We do a BFS to project the 3D lattice onto the 2D hyperbolic disk
    let mut disk_points: HashMap<usize, Point> = HashMap::new();
    let root_idx = 0; // Assuming 0 is root
    disk_points.insert(root_idx, Point::new(0.0, 0.0));

    // BFS to assign points
    let mut queue = vec![root_idx];
    let mut visited = std::collections::HashSet::new();
    visited.insert(root_idx);

    while !queue.is_empty() {
        let mut next_queue = Vec::new();

        for parent_idx in queue {
            let parent_pt = disk_points[&parent_idx];

            // Find children
            let mut children = Vec::new();
            for (p_idx, c_idx) in &crystal.bonds {
                if *p_idx == parent_idx {
                    children.push(*c_idx);
                }
            }

            // Distribute children evenly around the parent in hyperbolic space
            let n = children.len();
            if n > 0 {
                let r = 0.3; // Hyperbolic step size
                let angle_step = std::f64::consts::PI * 2.0 / (n as f64);

                for (i, child_idx) in children.iter().enumerate() {
                    if !visited.contains(child_idx) {
                        visited.insert(*child_idx);

                        let angle = i as f64 * angle_step;
                        let displacement = Point::new(r * angle.cos(), r * angle.sin());

                        // mobius_add applies the translation
                        let child_pt = mobius_add(parent_pt, displacement);
                        disk_points.insert(*child_idx, child_pt);

                        next_queue.push(*child_idx);
                    }
                }
            }
        }

        queue = next_queue;
    }

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));
        time += get_frame_time();

        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        let scale = (screen_height() * 0.45).min(screen_width() * 0.45);

        // Draw disk boundary
        draw_circle_lines(center.x, center.y, scale, 2.0, GRAY);

        // Pan the view slightly in hyperbolic space
        let cx = (time * 0.2).cos() * 0.5;
        let cy = (time * 0.15).sin() * 0.5;
        let view_shift = Point::new(cx as f64, cy as f64);

        // Draw bonds
        for (parent_idx, child_idx) in &crystal.bonds {
            if let (Some(p1), Some(p2)) = (disk_points.get(parent_idx), disk_points.get(child_idx)) {
                // Apply view shift
                let p1_shifted = mobius_add(*p1, view_shift);
                let p2_shifted = mobius_add(*p2, view_shift);

                let v1 = vec2(center.x + p1_shifted.re as f32 * scale, center.y + p1_shifted.im as f32 * scale);
                let v2 = vec2(center.x + p2_shifted.re as f32 * scale, center.y + p2_shifted.im as f32 * scale);

                // Opacity based on distance to boundary
                let r1 = p1_shifted.norm();
                let alpha = (1.0 - r1 as f32).max(0.1);

                draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, Color::new(1.0, 1.0, 1.0, alpha * 0.5));
            }
        }

        // Draw atoms
        for (idx, atom) in crystal.atoms.iter().enumerate() {
            if let Some(pt) = disk_points.get(&idx) {
                let shifted_pt = mobius_add(*pt, view_shift);

                let r = shifted_pt.norm();
                // If it's too close to the boundary, it's effectively invisible/infinitely small
                if r < 0.99 {
                    let v = vec2(center.x + shifted_pt.re as f32 * scale, center.y + shifted_pt.im as f32 * scale);

                    let size = if atom.is_dir { 3.0 } else { 1.5 };
                    // Apparent size shrinks near boundary due to perspective
                    let apparent_size = size * (1.0 - r as f32);

                    let color = if atom.is_dir {
                        Color::new(0.0, 1.0, 0.0, 0.8)
                    } else {
                        Color::new(0.0, 0.5, 1.0, 0.8)
                    };

                    draw_circle(v.x, v.y, apparent_size.max(0.5), color);
                }
            }
        }

        draw_text("Poincaré Lattice", 10.0, 20.0, 30.0, WHITE);
        draw_text("Hyperbolic Codebase Visualization", 10.0, 50.0, 20.0, GRAY);

        next_frame().await;
    }
}
