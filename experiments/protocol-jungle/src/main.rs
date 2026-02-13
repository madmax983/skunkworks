use ::rand::Rng;
use macroquad::prelude::*; // Disambiguate external rand

mod model;
use model::{Agent, Meaning};

const AGENT_COUNT: usize = 50;
const INTERACTION_RADIUS: f32 = 30.0;
const INTERACTION_COOLDOWN: f32 = 2.0;

fn symbol_to_color(s: Option<u8>) -> Color {
    match s {
        Some(sym) => {
            let hue = sym as f32 / 255.0;
            macroquad::color::hsl_to_rgb(hue, 1.0, 0.5)
        }
        None => GRAY,
    }
}

#[macroquad::main("Protocol Jungle")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    let mut agents: Vec<Agent> = (0..AGENT_COUNT)
        .map(|_| {
            Agent::new(
                rng.gen_range(0.0..screen_width()),
                rng.gen_range(0.0..screen_height()),
            )
        })
        .collect();

    struct InteractionLine {
        start: Vec2,
        end: Vec2,
        color: Color,
        alpha: f32,
    }

    let mut interaction_lines: Vec<InteractionLine> = Vec::new();

    loop {
        let dt = get_frame_time();
        let screen_w = screen_width();
        let screen_h = screen_height();

        clear_background(BLACK);

        // Update agents
        for agent in &mut agents {
            agent.update_pos(dt, screen_w, screen_h);
        }

        // Interactions
        // Naive O(N^2) for now
        for i in 0..agents.len() {
            if agents[i].cooldown > 0.0 {
                continue;
            }

            for j in (i + 1)..agents.len() {
                if agents[j].cooldown > 0.0 {
                    continue;
                }

                let dx = agents[i].x - agents[j].x;
                let dy = agents[i].y - agents[j].y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < INTERACTION_RADIUS * INTERACTION_RADIUS {
                    // Split borrow
                    let (left, right) = agents.split_at_mut(j);
                    let agent_i = &mut left[i];
                    let agent_j = &mut right[0]; // j - j = 0 because split_at_mut(j) puts j in the second slice at index 0

                    let success = agent_i.interact(agent_j);

                    // Set cooldowns
                    agent_i.cooldown = INTERACTION_COOLDOWN;
                    agent_j.cooldown = INTERACTION_COOLDOWN;

                    // Visual feedback
                    let color = if success { GREEN } else { RED };
                    interaction_lines.push(InteractionLine {
                        start: vec2(agent_i.x, agent_i.y),
                        end: vec2(agent_j.x, agent_j.y),
                        color,
                        alpha: 1.0,
                    });

                    // Store last result for visual state
                    agent_i.last_interaction_result = Some(success);
                    agent_j.last_interaction_result = Some(success);
                }
            }
        }

        // Draw interaction lines
        interaction_lines.retain_mut(|line| {
            line.alpha -= dt * 0.5;
            line.alpha > 0.0
        });

        for line in &interaction_lines {
            draw_line(
                line.start.x,
                line.start.y,
                line.end.x,
                line.end.y,
                2.0,
                Color::new(line.color.r, line.color.g, line.color.b, line.alpha),
            );
        }

        // Draw agents
        for agent in &agents {
            // Base circle
            draw_circle(agent.x, agent.y, 8.0, WHITE);

            // Protocol visualization (Rings)
            // Outer: Greeting
            let c_greet = symbol_to_color(agent.vocabulary.get(&Meaning::Greetings).cloned());
            draw_circle(agent.x, agent.y, 7.0, c_greet);

            // Middle: Ack
            let c_ack = symbol_to_color(agent.vocabulary.get(&Meaning::Ack).cloned());
            draw_circle(agent.x, agent.y, 5.0, c_ack);

            // Inner: Trade
            // let c_trade = symbol_to_color(agent.vocabulary.get(&Meaning::Trade).cloned());
            // draw_circle(agent.x, agent.y, 3.0, c_trade);

            // Status indicator (Success/Fail glow)
            if let Some(success) = agent.last_interaction_result {
                let glow_color = if success { GREEN } else { RED };
                if agent.cooldown > INTERACTION_COOLDOWN - 0.5 {
                    draw_circle_lines(agent.x, agent.y, 10.0, 2.0, glow_color);
                }
            }
        }

        // UI
        draw_text("Protocol Jungle", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", agents.len()),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Outer Ring: Greeting | Inner Ring: Ack",
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
