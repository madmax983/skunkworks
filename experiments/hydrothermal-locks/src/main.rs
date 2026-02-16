use crate::agents::{AgentState, LockManager};
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

#[macroquad::main("Hydrothermal Locks")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
    let mut fluid = FluidSim::new(GRID_WIDTH, GRID_HEIGHT);
    let mut locks = LockManager::new();
    let mut particles = ParticleSystem::new();
    let mut worms = WormManager::new();

    // Add some random vents and locks
    for x in (10..GRID_WIDTH - 10).step_by(20) {
        let id = (x / 20) as u8;
        grid.set_type(x, GRID_HEIGHT - 1, CellType::Vent(id));
        locks.add_lock(id, x, GRID_HEIGHT - 1);
    }

    // Spawn agents
    for _ in 0..50 {
        locks.spawn_agent(
            rand::gen_range(10.0f32, 700.0f32),
            rand::gen_range(10.0f32, 500.0f32),
        );
    }

    loop {
        let dt = get_frame_time();

        // Update Systems
        locks.update(&mut fluid, dt);
        fluid.update(&grid, dt);

        // Spawn particles from contentious agents
        for agent in &locks.agents {
            if let AgentState::Waiting = agent.state {
                // Spawn smoke
                if rand::gen_range(0.0f32, 1.0f32) < 0.1 {
                    // 10% chance per frame
                    particles.spawn(agent.pos, 1);
                }
            }
        }

        particles.update(&mut grid, &fluid, dt);
        worms.update(&grid, &fluid, dt);

        clear_background(BLACK);

        // Draw Grid
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                if let Some(cell) = grid.get(x, y) {
                    let temp = fluid.get_temp(x, y);

                    let color = match cell.cell_type {
                        CellType::Water => {
                            let t = temp.min(1.0);
                            Color::new(t * 0.5, t * 0.2, 0.2 + t * 0.1, 1.0)
                        }
                        CellType::Rock => BROWN,
                        CellType::Vent(_) => RED,
                        CellType::Chimney => DARKGRAY,
                    };

                    if cell.cell_type != CellType::Water || temp > 0.1 {
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

        // Draw Agents
        for agent in &locks.agents {
            let color = match agent.state {
                AgentState::Seeking => BLUE,
                AgentState::Waiting => RED,
                AgentState::Holding(_) => GREEN,
                AgentState::Cooldown(_) => WHITE,
            };
            draw_circle(agent.pos.x, agent.pos.y, 3.0, color);
        }

        // Draw Particles
        particles.draw();

        // Draw Worms
        worms.draw();

        draw_text(
            format!("Agents: {}", locks.agents.len()).as_str(),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            format!("Particles: {}", particles.particles.len()).as_str(),
            10.0,
            40.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Worms: {}", worms.worms.len()).as_str(),
            10.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            10.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
