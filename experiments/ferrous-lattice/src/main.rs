use ferrous_core::Platter;
use macroquad::prelude::*;
use miller_lattice::Crystal;
use std::path::PathBuf;

#[macroquad::main("Ferrous Lattice")]
async fn main() {
    let root_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for atom in &crystal.atoms {
        min_x = min_x.min(atom.position.x as f32);
        max_x = max_x.max(atom.position.x as f32);
        min_y = min_y.min(atom.position.y as f32);
        max_y = max_y.max(atom.position.y as f32);
    }

    let crys_w = (max_x - min_x).max(1.0);
    let crys_h = (max_y - min_y).max(1.0);

    let grid_w = 400;
    let grid_h = 400;
    let mut platter = Platter::new(grid_w, grid_h);

    let mut time = 0.0f64;

    loop {
        clear_background(BLACK);
        time += 0.016;

        platter.decay(0.98);

        for atom in &crystal.atoms {
            let normalized_x = (atom.position.x as f32 - min_x) / crys_w;
            let normalized_y = (atom.position.y as f32 - min_y) / crys_h;

            // Map to grid coordinates with some padding
            let x = (normalized_x * (grid_w as f32 * 0.8) + (grid_w as f32 * 0.1)) as usize;
            let y = (normalized_y * (grid_h as f32 * 0.8) + (grid_h as f32 * 0.1)) as usize;

            let amount = if atom.is_dir { 1.0 } else { -0.5 };

            // Pulse over time
            let pulse = (time * 2.0 + atom.position.z as f64).sin();

            platter.accumulate(x, y, amount * pulse * 0.1);
        }

        // Render fluid
        for y in 0..grid_h {
            for x in 0..grid_w {
                let v = platter.get(x, y);
                if v.abs() > 0.01 {
                    // Positive values are blue (directories), negative are red (files)
                    let c = if v > 0.0 {
                        Color::new(0.0, 0.5, 1.0, v.min(1.0) as f32)
                    } else {
                        Color::new(1.0, 0.0, 0.0, (-v).min(1.0) as f32)
                    };

                    let scr_x = x as f32 * (screen_width() / grid_w as f32);
                    let scr_y = y as f32 * (screen_height() / grid_h as f32);
                    let w = screen_width() / grid_w as f32;
                    let h = screen_height() / grid_h as f32;

                    draw_rectangle(scr_x, scr_y, w, h, c);
                }
            }
        }

        draw_text(
            "Ferrous Lattice: Magnetic Codebase",
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text("Blue: Directories | Red: Files", 10.0, 60.0, 20.0, GRAY);

        next_frame().await;
    }
}
