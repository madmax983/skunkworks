use macroquad::prelude::*;
use miller_lattice::Crystal;
use poincare_disk::{mobius_add, Point};
use std::collections::HashMap;
use std::path::Path;

fn window_conf() -> Conf {
    Conf {
        window_title: "Poincaré Lattice".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root_path = Path::new(manifest_dir).join("../../crates");
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    let atom_count = crystal.atoms.len();
    println!(
        "Built Crystal Lattice from {:?} with {} atoms.",
        root_path, atom_count
    );

    let mut h_points: HashMap<usize, Point> = HashMap::new();
    let mut depths: HashMap<usize, usize> = HashMap::new();

    if !crystal.atoms.is_empty() {
        h_points.insert(0, Point::new(0.0, 0.0));
        depths.insert(0, 0);
    }

    for &(parent_idx, child_idx) in &crystal.bonds {
        let p_atom = &crystal.atoms[parent_idx];
        let c_atom = &crystal.atoms[child_idx];

        let p_pos = *h_points.get(&parent_idx).unwrap_or(&Point::new(0.0, 0.0));
        let depth = *depths.get(&parent_idx).unwrap_or(&0) + 1;
        depths.insert(child_idx, depth);

        let dx = (c_atom.position.x - p_atom.position.x) as f64;
        let dy = (c_atom.position.y - p_atom.position.y) as f64;
        let dz = (c_atom.position.z - p_atom.position.z) as f64;

        let vx = dx + dz * 0.5;
        let vy = dy + dz * 0.5;

        let len = (vx * vx + vy * vy).sqrt();
        let (nx, ny) = if len > 0.0 {
            (vx / len, vy / len)
        } else {
            (1.0, 0.0)
        };

        let step_size = 0.2 / (1.0 + (depth as f64) * 0.05);

        let step = Point::new(nx * step_size, ny * step_size);

        let new_pos = mobius_add(p_pos, step);
        h_points.insert(child_idx, new_pos);
    }

    let mut time = 0.0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
        time += get_frame_time();

        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let radius = cx.min(cy) * 0.9;

        draw_circle_lines(cx, cy, radius, 2.0, Color::new(0.3, 0.3, 0.4, 0.5));

        for &(parent_idx, child_idx) in &crystal.bonds {
            if let (Some(p1), Some(p2)) = (h_points.get(&parent_idx), h_points.get(&child_idx)) {
                let x1 = cx + (p1.re as f32) * radius;
                let y1 = cy + (p1.im as f32) * radius;

                let x2 = cx + (p2.re as f32) * radius;
                let y2 = cy + (p2.im as f32) * radius;

                draw_line(x1, y1, x2, y2, 1.0, Color::new(0.0, 1.0, 0.8, 0.4));
            }
        }

        for (idx, point) in &h_points {
            let atom = &crystal.atoms[*idx];
            let px = cx + (point.re as f32) * radius;
            let py = cy + (point.im as f32) * radius;

            let r_sq = point.norm_sqr() as f32;
            let size = 3.0 * (1.0 - r_sq).max(0.1);

            let color = if atom.is_dir {
                Color::new(0.2, 0.8, 0.2, 0.8)
            } else {
                Color::new(0.8, 0.4, 0.1, 0.8)
            };

            draw_circle(px, py, size, color);

            let pulse = ((time * 2.0 + *idx as f32 * 0.1).sin() * 0.5 + 0.5) * 1.5;
            draw_circle_lines(
                px,
                py,
                size + pulse,
                0.5,
                Color::new(color.r, color.g, color.b, 0.3),
            );
        }

        draw_text("Poincaré Lattice", 20.0, 40.0, 30.0, WHITE);
        draw_text(&format!("Nodes: {}", atom_count), 20.0, 70.0, 20.0, GRAY);
        draw_text("Hyperbolic Codebase Morphogenesis", 20.0, 95.0, 20.0, GRAY);

        next_frame().await
    }
}
