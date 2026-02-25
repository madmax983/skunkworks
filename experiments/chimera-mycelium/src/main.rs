use macroquad::prelude::*;
use ::rand::thread_rng;
use ::rand::Rng;

mod chaos;
mod agent;

use chaos::ChaosSubstrate;
use agent::{Agent, AgentAction};

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 200;
const MAX_AGENTS: usize = 1000;

#[macroquad::main("Chimera Mycelium")]
async fn main() {
    let mut rng = thread_rng();

    // Initialize Chaos Substrate
    let mut substrate = ChaosSubstrate::new(GRID_WIDTH, GRID_HEIGHT);
    let mut sequence = "BAB".to_string();
    let mut a_range = (2.8, 4.0);
    let mut b_range = (2.8, 4.0);

    substrate.generate(&sequence, a_range, b_range);
    substrate.texture.set_filter(FilterMode::Nearest);

    // Initialize Agents
    let start_pos = IVec2::new(GRID_WIDTH as i32 / 2, GRID_HEIGHT as i32 / 2);
    let mut agents = vec![Agent::new(Agent::random_dna(), start_pos)];

    // Mycelial Network (Segments for drawing)
    // List of (start, end) points in grid coords
    let mut network: Vec<(IVec2, IVec2)> = Vec::new();

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        let cell_w = screen_w / GRID_WIDTH as f32;
        let cell_h = screen_h / GRID_HEIGHT as f32;

        // --- Input ---
        if is_key_pressed(KeyCode::Space) {
            // Reset
            agents = vec![Agent::new(Agent::random_dna(), start_pos)];
            network.clear();

            // Mutate Chaos
            let seqs = ["A", "B", "AB", "BA", "AAB", "ABB", "BBA", "BBAB", "AAAAAB"];
            sequence = seqs[rng.gen_range(0..seqs.len())].to_string();
            a_range = (rng.gen_range(2.0..3.0), rng.gen_range(3.5..4.0));
            b_range = (rng.gen_range(2.0..3.0), rng.gen_range(3.5..4.0));
            substrate.generate(&sequence, a_range, b_range);
        }

        // --- Update ---
        let mut new_agents = Vec::new();
        let current_agent_count = agents.len();

        for agent in agents.iter_mut() {
            let cost = substrate.get_cost(agent.pos.x, agent.pos.y);
            let action = agent.step(cost);

            match action {
                AgentAction::Move(new_pos) => {
                    // Check bounds
                    if new_pos.x >= 0 && new_pos.x < GRID_WIDTH as i32 &&
                       new_pos.y >= 0 && new_pos.y < GRID_HEIGHT as i32 {
                        network.push((agent.pos, new_pos));
                        agent.pos = new_pos;
                    } else {
                        // Hit wall -> Die
                        agent.energy = 0.0; // Mark for death
                    }
                }
                AgentAction::Branch(new_pos) => {
                    if current_agent_count + new_agents.len() < MAX_AGENTS {
                        if new_pos.x >= 0 && new_pos.x < GRID_WIDTH as i32 &&
                           new_pos.y >= 0 && new_pos.y < GRID_HEIGHT as i32 {
                            let mut child = agent.clone();
                            child.pos = new_pos;
                            child.mutate(); // Evolve
                            child.energy = 50.0; // Give child energy
                            new_agents.push(child);

                            network.push((agent.pos, new_pos));
                        }
                    }
                }
                AgentAction::Die => {
                    // Already handled by filtering via energy check below
                }
                AgentAction::None => {}
            }
        }

        // Remove dead agents
        agents.retain(|a| a.energy > 0.0);
        agents.append(&mut new_agents);

        // --- Draw ---
        clear_background(BLACK);

        // Draw Chaos
        draw_texture_ex(
            &substrate.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                ..Default::default()
            },
        );

        // Draw Network
        // We draw lines in screen space
        for (start, end) in &network {
            let x1 = start.x as f32 * cell_w + cell_w/2.0;
            let y1 = start.y as f32 * cell_h + cell_h/2.0;
            let x2 = end.x as f32 * cell_w + cell_w/2.0;
            let y2 = end.y as f32 * cell_h + cell_h/2.0;
            draw_line(x1, y1, x2, y2, 1.0, Color::new(1.0, 1.0, 1.0, 0.5));
        }

        // Draw Agents (Tips)
        for agent in &agents {
            let x = agent.pos.x as f32 * cell_w + cell_w/2.0;
            let y = agent.pos.y as f32 * cell_h + cell_h/2.0;
            draw_circle(x, y, 2.0, RED);
        }

        // --- UI ---
        draw_rectangle(0., 0., screen_w, 40., Color::new(0., 0., 0., 0.7));
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 25.0, 20.0, WHITE);
        draw_text(&format!("Sequence: {}", sequence), 150.0, 25.0, 20.0, YELLOW);

        next_frame().await
    }
}
