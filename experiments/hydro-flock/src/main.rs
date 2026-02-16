use crate::agents::FlockManager;
use crate::biology::WormManager;
use crate::fluid::FluidSim;
use crate::grid::{CellType, Grid};
use crate::particles::ParticleSystem;
use macroquad::prelude::*;

mod agents;
mod biology;
mod fluid;
mod grid;
mod particles;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 80;
const CELL_SIZE: f32 = 8.0;

#[macroquad::main("Hydro Flock")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut fluid = FluidSim::new(GRID_WIDTH, GRID_HEIGHT);
    let mut flock = FlockManager::new();
    let mut particles = ParticleSystem::new();
    let mut worms = WormManager::new();

    // Add vents
    for x in (10..GRID_WIDTH - 10).step_by(20) {
        let id = (x / 20) as u8;
        grid.set_type(x, GRID_HEIGHT - 1, CellType::Vent(id));
        flock.add_lock(id, x, GRID_HEIGHT - 1);
    }

    // Spawn boids
    for _ in 0..100 {
        flock.spawn_agent(
            rand::gen_range(10.0f32, 700.0f32),
            rand::gen_range(10.0f32, 500.0f32),
        );
    }

    loop {
        let dt = get_frame_time();

        // Update Systems
        flock.update(&mut fluid, dt);
        fluid.update(&grid, dt);

        // Spawn particles from hot spots (Vents/Contention)
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let temp = fluid.get_temp(x, y);
                if temp > 0.5 {
                    if rand::gen_range(0.0, 1.0) < 0.05 * temp {
                        let px = x as f32 * CELL_SIZE + rand::gen_range(0.0, CELL_SIZE);
                        let py = y as f32 * CELL_SIZE + rand::gen_range(0.0, CELL_SIZE);
                        particles.spawn(vec2(px, py), 1);
                    }
                }
            }
        }

        particles.update(&mut grid, &fluid, dt);
        worms.update(&grid, &fluid, dt);

        clear_background(BLACK);

        // Draw Grid (Fluid/Heat)
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(cell) = grid.get(x, y) {
                    let temp = fluid.get_temp(x, y);

                    let color = match cell.cell_type {
                        CellType::Water => {
                            let t = temp.min(1.0);
                            // Heat makes water glow
                            Color::new(t * 0.8, t * 0.4, 0.2 + t * 0.2, 1.0)
                        }
                        CellType::Rock => BROWN,
                        CellType::Vent(_) => RED,
                        CellType::Chimney => DARKGRAY,
                    };

                    if cell.cell_type != CellType::Water || temp > 0.05 {
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
            // Draw as triangle
            let angle = agent.vel.y.atan2(agent.vel.x);
            let size = 6.0;
            let tip = agent.pos + vec2(angle.cos(), angle.sin()) * size;
            let left = agent.pos + vec2((angle + 2.5).cos(), (angle + 2.5).sin()) * size * 0.7;
            let right = agent.pos + vec2((angle - 2.5).cos(), (angle - 2.5).sin()) * size * 0.7;

            draw_triangle(tip, left, right, YELLOW);
        }

        particles.draw();
        worms.draw();

        // UI
        draw_text(
            &format!("Boids: {}", flock.agents.len()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}
