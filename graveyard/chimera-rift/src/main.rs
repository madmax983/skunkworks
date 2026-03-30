mod agent;
mod world;

use agent::Agent;
use macroquad::prelude::*;
use world::{World, GRID_HEIGHT, GRID_WIDTH, TILE_SIZE};

#[macroquad::main("Chimera-Rift")]
async fn main() {
    let mut world = World::new();
    let mut agents: Vec<Agent> = (0..50)
        .map(|i| Agent::new(i, GRID_WIDTH / 2, GRID_HEIGHT / 2))
        .collect();

    loop {
        // Logic Update
        world.update();

        for agent in agents.iter_mut() {
            agent.update(&mut world);
        }

        // Remove Dead Agents (energy <= 0)
        agents.retain(|a| a.energy > 0);

        // Repopulate (if low pop)
        if agents.len() < 20 {
            // New agents spawn at center or random food location?
            // Let's spawn at center
            let new_agent = Agent::new(::rand::random(), GRID_WIDTH / 2, GRID_HEIGHT / 2);
            agents.push(new_agent);
        }

        // Render
        clear_background(BLACK);

        // Draw World (Grid, Walls, Food, Portals)
        world.render();

        // Draw Agents
        for agent in &agents {
            let x = agent.pos.0 as f32 * TILE_SIZE;
            let y = agent.pos.1 as f32 * TILE_SIZE;

            draw_circle(
                x + TILE_SIZE / 2.0,
                y + TILE_SIZE / 2.0,
                TILE_SIZE / 3.0,
                agent.color,
            );

            // Draw Anchor Line if active
            if let Some(anchor) = agent.anchor {
                let ax = anchor.0 as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let ay = anchor.1 as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                draw_line(x + TILE_SIZE / 2.0, y + TILE_SIZE / 2.0, ax, ay, 1.0, BLUE);
            }
        }

        // UI
        draw_text("Chimera-Rift: Evo-Portals", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Portals: {}", world.portals.len()),
            10.0,
            70.0,
            20.0,
            ORANGE,
        );
        draw_text(
            &format!("Food: {}", world.food_count),
            10.0,
            90.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}
