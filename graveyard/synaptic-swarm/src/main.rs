mod agent;
mod brain;

use agent::{Agent, NeighborInfo};
use brain::{N_INTER, N_MOTOR, N_SENSORY, N_TOTAL};
use macroquad::prelude::*;

const AGENT_COUNT: usize = 50;
const SIDEBAR_WIDTH: f32 = 300.0;

#[macroquad::main("Synaptic Swarm")]
async fn main() {
    // Seed macroquad random
    rand::srand(macroquad::miniquad::date::now() as u64);

    let mut agents = Vec::with_capacity(AGENT_COUNT);
    let sw = screen_width() - SIDEBAR_WIDTH;
    let sh = screen_height();

    for _ in 0..AGENT_COUNT {
        let pos = vec2(rand::gen_range(0.0, sw), rand::gen_range(0.0, sh));
        agents.push(Agent::new(pos));
    }

    let mut selected_agent_idx: Option<usize> = None;

    loop {
        let dt = get_frame_time().min(0.05);
        let sw = screen_width() - SIDEBAR_WIDTH;
        let sh = screen_height();

        // 1. Calculate Neighbors
        // For simplicity, O(N^2)
        // Store neighbors for each agent
        let mut all_neighbors: Vec<Vec<NeighborInfo>> = vec![Vec::new(); agents.len()];

        for i in 0..agents.len() {
            for j in 0..agents.len() {
                if i == j {
                    continue;
                }

                let mut diff = agents[j].pos - agents[i].pos;
                // Toroidal wrapping distance
                if diff.x > sw / 2.0 {
                    diff.x -= sw;
                }
                if diff.x < -sw / 2.0 {
                    diff.x += sw;
                }
                if diff.y > sh / 2.0 {
                    diff.y -= sh;
                }
                if diff.y < -sh / 2.0 {
                    diff.y += sh;
                }

                let dist = diff.length();

                // Angle relative to velocity
                let forward = if agents[i].vel.length() > 0.001 {
                    agents[i].vel.normalize()
                } else {
                    vec2(1.0, 0.0)
                };

                // Angle between forward and diff
                // atan2(y, x) gives angle.
                // Angle of forward: atan2(fy, fx)
                // Angle of diff: atan2(dy, dx)
                // Relative angle: diff - forward
                let angle_forward = forward.y.atan2(forward.x);
                let angle_target = diff.y.atan2(diff.x);
                let mut rel_angle = angle_target - angle_forward;

                // Normalize to -PI..PI
                while rel_angle > std::f32::consts::PI {
                    rel_angle -= 2.0 * std::f32::consts::PI;
                }
                while rel_angle < -std::f32::consts::PI {
                    rel_angle += 2.0 * std::f32::consts::PI;
                }

                if dist < 150.0 {
                    // interaction radius
                    all_neighbors[i].push(NeighborInfo {
                        dist,
                        angle: rel_angle,
                    });
                }
            }
        }

        // 2. Update Agents
        for (i, agent) in agents.iter_mut().enumerate() {
            agent.update(dt, &all_neighbors[i], sw, sh);
        }

        // 3. Handle Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position().into();
            let mut best_dist = 50.0;
            let mut best_idx = None;

            for (i, agent) in agents.iter().enumerate() {
                let d = agent.pos.distance(mpos);
                if d < best_dist {
                    best_dist = d;
                    best_idx = Some(i);
                }
            }
            selected_agent_idx = best_idx;
        }

        // 4. Draw
        clear_background(Color::new(0.1, 0.1, 0.12, 1.0));

        // Draw Boundary
        draw_line(sw, 0.0, sw, sh, 2.0, GRAY);

        // Draw Agents
        for (i, agent) in agents.iter().enumerate() {
            agent.draw();

            // Highlight selected
            if Some(i) == selected_agent_idx {
                draw_circle_lines(agent.pos.x, agent.pos.y, 15.0, 2.0, YELLOW);
            }
        }

        // Draw Sidebar (Brain)
        if let Some(idx) = selected_agent_idx {
            if idx < agents.len() {
                draw_brain(&agents[idx], sw, 0.0, SIDEBAR_WIDTH, sh);
            } else {
                selected_agent_idx = None; // Reset if invalid
            }
        } else {
            draw_text("Select an agent", sw + 10.0, 30.0, 20.0, WHITE);
            draw_text("to see its brain.", sw + 10.0, 50.0, 20.0, WHITE);
        }

        draw_text("Synaptic Swarm", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}

fn draw_brain(agent: &Agent, x: f32, y: f32, w: f32, h: f32) {
    // Draw background
    draw_rectangle(x, y, w, h, Color::new(0.05, 0.05, 0.05, 1.0));

    let brain = &agent.brain;

    // Layout neurons
    // Sensory: Top
    // Inter: Middle
    // Motor: Bottom

    let radius = 8.0;
    let padding = 40.0;

    let sensory_y = y + padding;
    let inter_y = y + h / 2.0;
    let motor_y = y + h - padding;

    // Helper to get position
    let get_pos = |idx: usize| -> Vec2 {
        if idx < N_SENSORY {
            let step = w / (N_SENSORY as f32 + 1.0);
            vec2(x + step * (idx as f32 + 1.0), sensory_y)
        } else if idx < N_SENSORY + N_INTER {
            let i = idx - N_SENSORY;
            let step = w / (N_INTER as f32 + 1.0);
            vec2(x + step * (i as f32 + 1.0), inter_y)
        } else {
            let i = idx - (N_SENSORY + N_INTER);
            let step = w / (N_MOTOR as f32 + 1.0);
            vec2(x + step * (i as f32 + 1.0), motor_y)
        }
    };

    // Draw Synapses first
    for i in 0..N_TOTAL {
        for j in 0..N_TOTAL {
            let weight = brain.weights[i][j];
            if weight.abs() > 0.1 {
                let start = get_pos(i);
                let end = get_pos(j);

                let thickness = (weight.abs() / 5.0).clamp(0.5, 3.0);
                let color = if weight > 0.0 {
                    Color::new(0.0, 0.5, 1.0, 0.3) // Blue excitatory
                } else {
                    Color::new(1.0, 0.0, 0.0, 0.3) // Red inhibitory
                };

                draw_line(start.x, start.y, end.x, end.y, thickness, color);

                // Flash if spiked recently (trace is high)
                if brain.traces[i] > 0.5 {
                    let flash_color = if weight > 0.0 { BLUE } else { RED };
                    draw_line(start.x, start.y, end.x, end.y, thickness * 2.0, flash_color);
                }
            }
        }
    }

    // Draw Neurons
    for i in 0..N_TOTAL {
        let pos = get_pos(i);
        let n = &brain.neurons[i];

        // Color by voltage
        // -65 (rest) -> Dark
        // 30 (spike) -> Bright
        let v_norm = ((n.v + 70.0) / 100.0).clamp(0.0, 1.0);
        let color = Color::new(v_norm, v_norm, v_norm, 1.0);

        draw_circle(pos.x, pos.y, radius, color);
        draw_circle_lines(pos.x, pos.y, radius, 1.0, GRAY);

        // Label
        let label = if i < N_SENSORY {
            match i {
                0 => "Dist",
                1 => "Ang",
                2 => "Vel",
                3 => "Rnd",
                _ => "S",
            }
        } else if i >= N_SENSORY + N_INTER {
            match i - (N_SENSORY + N_INTER) {
                0 => "Thrust",
                1 => "Turn",
                _ => "M",
            }
        } else {
            ""
        };
        draw_text(label, pos.x - 10.0, pos.y - radius - 5.0, 10.0, LIGHTGRAY);
    }
}
