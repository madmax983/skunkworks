use macroquad::prelude::*;

mod agent;
mod rhythm;

use agent::{Agent, INTERACTION_RADIUS, INTERACTION_COOLDOWN};
use ::rand::Rng;

const AGENT_COUNT: usize = 50;

fn pulses_to_color(pulses: usize) -> Color {
    // Map 1-16 to Blue -> Red
    // Actually HSL 0 is Red, 240 is Blue.
    // Let's map 1->Blue (240), 16->Red (0).
    let h = (1.0 - (pulses as f32 / 16.0)) * 0.7; // 0.7 (Blueish) to 0.0 (Red)

    // Using macroquad::color::hsl_to_rgb
    macroquad::color::hsl_to_rgb(h, 0.8, 0.5)
}

#[macroquad::main("Rhythmic Jungle")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let screen_w = screen_width();
    let screen_h = screen_height();

    let mut agents: Vec<Agent> = (0..AGENT_COUNT)
        .map(|i| {
            Agent::new(
                i,
                rng.gen_range(0.0..screen_w),
                rng.gen_range(0.0..screen_h),
            )
        })
        .collect();

    let mut interaction_lines: Vec<((f32, f32), (f32, f32), Color, f32)> = Vec::new(); // (start, end, color, alpha)

    loop {
        let dt = get_frame_time();
        let screen_w = screen_width();
        let screen_h = screen_height();

        clear_background(BLACK);

        // Update agents
        for agent in &mut agents {
            agent.update(dt, screen_w, screen_h);
        }

        // Interactions
        // Naive O(N^2) for now
        for i in 0..agents.len() {
            if agents[i].cooldown > 0.0 {
                continue;
            }

            // We need to mutate two different agents.
            // Split borrow trick
            let (left, right) = agents.split_at_mut(i + 1);
            let agent_i = &mut left[i]; // The i-th element is in the left slice at index i

            for agent_j in right {
                if agent_j.cooldown > 0.0 {
                    continue;
                }

                let dx = agent_i.x - agent_j.x;
                let dy = agent_i.y - agent_j.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < INTERACTION_RADIUS * INTERACTION_RADIUS {
                    let success = agent_i.interact(agent_j);

                    // Set cooldowns
                    agent_i.cooldown = INTERACTION_COOLDOWN;
                    agent_j.cooldown = INTERACTION_COOLDOWN;

                    // Visual feedback
                    let color = if success { GREEN } else { RED };
                    interaction_lines.push(((agent_i.x, agent_i.y), (agent_j.x, agent_j.y), color, 1.0));
                }
            }
        }

        // Draw interaction lines
        interaction_lines.retain_mut(|(_, _, _, alpha)| {
            *alpha -= dt * 1.0;
            *alpha > 0.0
        });

        for (start, end, color, alpha) in &interaction_lines {
            draw_line(start.0, start.1, end.0, end.1, 2.0, Color::new(color.r, color.g, color.b, *alpha));
        }

        // Draw agents
        for agent in &agents {
            // Base circle color based on density
            let color = pulses_to_color(agent.rhythm.pulses);

            // Pulse size
            let base_radius = 8.0;
            let radius = if agent.is_pulsing {
                base_radius * 1.5
            } else {
                base_radius
            };

            draw_circle(agent.x, agent.y, radius, color);

            // Draw BPM text nearby (optional, maybe too cluttered)
            // draw_text(&format!("{:.0}", agent.bpm), agent.x - 10.0, agent.y - 10.0, 10.0, WHITE);
        }

        // UI
        draw_text("Rhythmic Jungle", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", agents.len()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Color: Rhythm Density (Blue=Sparse, Red=Dense)", 10.0, 70.0, 20.0, LIGHTGRAY);

        // Calculate average BPM
        let avg_bpm: f32 = agents.iter().map(|a| a.bpm).sum::<f32>() / agents.len() as f32;
        draw_text(&format!("Avg BPM: {:.1}", avg_bpm), 10.0, 90.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
