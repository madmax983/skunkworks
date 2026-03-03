mod agents;
mod audio;
mod fluid;
mod grid;

use agents::FlockManager;
use audio::AudioEngine;
use fluid::FluidSim;
use grid::{CellType, Grid};
use macroquad::prelude::*;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 80;
const CELL_SIZE: f32 = 8.0;

#[macroquad::main("Hydro Soundscapes")]
async fn main() {
    let grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut fluid = FluidSim::new(GRID_WIDTH, GRID_HEIGHT);
    let mut flock = FlockManager::new();
    let mut audio_engine = AudioEngine::new();

    // Spawn boids
    for _ in 0..200 {
        flock.spawn_agent(
            rand::gen_range(10.0f32, (GRID_WIDTH as f32 * CELL_SIZE) - 10.0),
            rand::gen_range(10.0f32, (GRID_HEIGHT as f32 * CELL_SIZE) - 10.0),
        );
    }

    loop {
        let dt = get_frame_time();

        // Update Systems
        flock.update(&mut fluid, dt);
        fluid.update(&grid, dt);

        // Analyze Fluid for Audio
        let mut total_a = 0.0;
        let mut total_b = 0.0;
        for i in 0..fluid.chem_a.len() {
            total_a += fluid.chem_a[i];
            total_b += fluid.chem_b[i];
        }
        audio_engine.update(total_a, total_b);

        clear_background(BLACK);

        // Draw Grid (Chemicals)
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(cell) = grid.get(x, y) {
                    if cell.cell_type == CellType::Rock {
                        draw_rectangle(
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE,
                            CELL_SIZE,
                            CELL_SIZE,
                            BROWN,
                        );
                        continue;
                    }

                    let a = fluid.get_chem_a(x, y);
                    let b = fluid.get_chem_b(x, y);

                    // Visualization
                    let r = b.min(1.0);
                    let g = (b * 0.5 + a * 0.2).min(1.0);
                    let bl = (a * 0.5).min(1.0);

                    if r > 0.05 || g > 0.05 || bl > 0.05 {
                        let color = Color::new(r, g, bl, 1.0);
                        draw_rectangle(
                            x as f32 * CELL_SIZE,
                            y as f32 * CELL_SIZE,
                            CELL_SIZE,
                            CELL_SIZE,
                            color,
                        );
                    }
                }
            }
        }

        // Draw Boids
        for agent in &flock.agents {
            let angle = agent.vel.y.atan2(agent.vel.x);
            let size = 6.0;
            let tip = agent.pos + vec2(angle.cos(), angle.sin()) * size;
            let left = agent.pos + vec2((angle + 2.5).cos(), (angle + 2.5).sin()) * size * 0.7;
            let right = agent.pos + vec2((angle - 2.5).cos(), (angle - 2.5).sin()) * size * 0.7;

            draw_triangle(tip, left, right, WHITE);
        }

        // UI
        draw_text(
            &format!("Boids: {}", flock.agents.len()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("Chem A: {:.0}", total_a), 10.0, 40.0, 20.0, WHITE);
        draw_text(&format!("Chem B: {:.0}", total_b), 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 80.0, 20.0, WHITE);

        next_frame().await
    }
}
