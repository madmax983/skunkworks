//! # Myco Lattice 🍄💠
//!
//! **A hybrid of `crates/miller-lattice` and `experiments/myco-transit`.**
//!
//! "The slime mold maps the crystalline directory structure."
//!
//! ## Concept
//!
//! This experiment combines the hierarchical 3D crystal lattice generation of file systems (`miller-lattice`) with the biological slime mold (Physarum polycephalum) pathfinding simulation (`myco-transit`).
//!
//! - **Structure**: The `crates` directory is parsed into a `Crystal`, generating a 3D topological map of the codebase, which is projected into a 2D grid.
//! - **Foraging**: These directory/file "atoms" act as food sources ("cities").
//! - **Mycelial Agents**: Thousands of slime mold agents navigate the grid, leaving pheromone trails.
//! - **Emergent Behavior**: The slime mold naturally forms organic, glowing "highways" between the deeply nested hierarchical nodes of the codebase, mapping out an efficient biological transit network over a rigid crystalline architecture.
//!
//! ## Lineage
//!
//! - **Parent A**: `crates/miller-lattice` (The Splice Surgeon) - Provided the Crystal struct that maps a codebase into a spatial, hierarchical lattice.
//! - **Parent B**: `experiments/myco-transit` (The Splice Surgeon) - Provided the slime mold pheromone agents and `rayon` driven parallel simulation logic.
//!
//! ## Controls
//!
//! -   **Q / Esc**: Quit.
//!
//! ## Build
//!
//! ```bash
//! cargo run -p myco-lattice --release
//! ```
//!
mod simulation;

use macroquad::prelude::*;
use miller_lattice::Crystal;
use simulation::{Agent, World};
use std::path::Path;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Myco Lattice".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    if std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux") {
        return;
    }

    // Use CARGO_MANIFEST_DIR to dynamically resolve the path relative to this crate
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let root_path = Path::new(manifest_dir).join("../../crates");
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

    let grid_width = 800;
    let grid_height = 800;
    let mut world = World::new(grid_width, grid_height);

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for atom in &crystal.atoms {
        let x = atom.position.x as f32;
        let y = atom.position.y as f32;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let range_x = (max_x - min_x).max(1.0);
    let range_y = (max_y - min_y).max(1.0);

    for atom in &crystal.atoms {
        let nx = (atom.position.x as f32 - min_x) / range_x;
        let ny = (atom.position.y as f32 - min_y) / range_y;

        let cx = 100.0 + nx as f64 * (grid_width as f64 - 200.0);
        let cy = 100.0 + ny as f64 * (grid_height as f64 - 200.0);
        world.cities.push((cx, cy));
    }

    if world.cities.is_empty() {
        world
            .cities
            .push((grid_width as f64 / 2.0, grid_height as f64 / 2.0));
    }

    use ::rand::Rng;
    let mut rng = ::rand::thread_rng();

    let num_agents = 5000;
    let mut agents = Vec::with_capacity(num_agents);

    for _ in 0..num_agents {
        let home_city = rng.gen_range(0..world.cities.len());
        let mut target_city = rng.gen_range(0..world.cities.len());
        while target_city == home_city && world.cities.len() > 1 {
            target_city = rng.gen_range(0..world.cities.len());
        }

        let (cx, cy) = world.cities[home_city];
        agents.push(Agent::new(
            cx,
            cy,
            rng.gen::<f64>() * std::f64::consts::PI * 2.0,
            home_city,
            target_city,
        ));
    }

    let mut image = Image::gen_image_color(grid_width as u16, grid_height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        // --- Sim Update ---
        for _ in 0..2 {
            world.update_agents_parallel(&mut agents);
            world.diffuse_and_decay();
        }

        // --- Render ---
        let data = image.get_image_data_mut();
        for y in 0..grid_height {
            for x in 0..grid_width {
                let idx = y * grid_width + x;
                let val = world.trails[idx];
                let c = val.min(255.0) as u8;
                let c2 = c / 2;
                data[idx] = [0, c, c2, 255];
            }
        }

        for &(cx, cy) in &world.cities {
            let px = cx as usize;
            let py = cy as usize;
            if px < grid_width && py < grid_height {
                let idx = py * grid_width + px;
                data[idx] = [255, 255, 255, 255];
            }
        }

        texture.update(&image);

        clear_background(BLACK);

        let sw = screen_width();
        let sh = screen_height();
        let scale = (sw / grid_width as f32).min(sh / grid_height as f32);

        let draw_w = grid_width as f32 * scale;
        let draw_h = grid_height as f32 * scale;
        let offset_x = (sw - draw_w) / 2.0;
        let offset_y = (sh - draw_h) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
