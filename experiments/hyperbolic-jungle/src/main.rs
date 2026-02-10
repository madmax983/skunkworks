use macroquad::prelude::*;
use num_complex::Complex;
use rayon::prelude::*;

mod agent;
mod rhythm;
use agent::{Agent, INTERACTION_COOLDOWN, INTERACTION_RADIUS};
use poincare_disk::{hyperbolic_dist, Point};

fn pulses_to_color(pulses: usize) -> Color {
    // Map 1-16 to Blue -> Red
    // HSL: 0.7 (Blue) -> 0.0 (Red)
    let h = (1.0 - (pulses as f32 / 16.0)) * 0.7;
    macroquad::color::hsl_to_rgb(h, 0.8, 0.5)
}

#[macroquad::main("Hyperbolic Jungle")]
async fn main() {
    let mut rng = ::rand::thread_rng();

    // N=200 for performance with O(N^2)
    let num_agents = 200;

    let mut agents: Vec<Agent> = (0..num_agents).map(|i| Agent::new(i)).collect();

    let mut interaction_lines: Vec<((f32, f32), (f32, f32), Color, f32)> = Vec::new();

    loop {
        let dt = get_frame_time();
        let width = screen_width();
        let height = screen_height();

        let disk_radius = width.min(height) * 0.45;
        let disk_center = vec2(width / 2.0, height / 2.0);

        // Update Agents
        // Parallel update for movement/rhythm
        agents.par_iter_mut().for_each(|agent| {
            agent.update(dt);
        });

        // Interactions (Sequential for now to allow mutation of pairs)
        for i in 0..agents.len() {
            if agents[i].cooldown > 0.0 {
                continue;
            }

            let (left, right) = agents.split_at_mut(i + 1);
            let agent_i = &mut left[i];

            for agent_j in right {
                if agent_j.cooldown > 0.0 {
                    continue;
                }

                // Check hyperbolic distance
                let dist = hyperbolic_dist(agent_i.pos, agent_j.pos);

                if dist < INTERACTION_RADIUS {
                    // Interact logic (converge BPM/Pulses)
                    agent_i.interact(agent_j);

                    // Set Cooldowns
                    agent_i.cooldown = INTERACTION_COOLDOWN;
                    agent_j.cooldown = INTERACTION_COOLDOWN;

                    // Visual line
                    // Map positions to screen
                    let p1_screen = complex_to_screen(agent_i.pos, disk_center, disk_radius);
                    let p2_screen = complex_to_screen(agent_j.pos, disk_center, disk_radius);

                    interaction_lines.push((p1_screen, p2_screen, GREEN, 1.0));
                }
            }
        }

        // Draw
        clear_background(BLACK);

        // Draw Disk Boundary
        draw_circle_lines(disk_center.x, disk_center.y, disk_radius, 2.0, DARKGRAY);

        // Draw Interaction Lines
        interaction_lines.retain_mut(|(_, _, _, alpha)| {
            *alpha -= dt * 2.0;
            *alpha > 0.0
        });

        for (start, end, _, alpha) in &interaction_lines {
            draw_line(
                start.0,
                start.1,
                end.0,
                end.1,
                1.0,
                Color::new(0.0, 1.0, 0.0, *alpha),
            );
        }

        // Draw Agents
        for agent in &agents {
            let screen_pos = complex_to_screen(agent.pos, disk_center, disk_radius);

            let mut color = pulses_to_color(agent.rhythm.pulses);

            let mut radius = 3.0;
            if agent.is_pulsing {
                radius = 6.0;
                color = WHITE;
            }

            draw_circle(screen_pos.0, screen_pos.1, radius, color);
        }

        // UI
        draw_text("Hyperbolic Jungle", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Agents: {}", num_agents),
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        let avg_bpm: f32 = agents.iter().map(|a| a.bpm).sum::<f32>() / agents.len() as f32;
        draw_text(
            &format!("Avg BPM: {:.1}", avg_bpm),
            20.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}

fn complex_to_screen(z: Point, center: Vec2, radius: f32) -> (f32, f32) {
    let x = center.x + z.re as f32 * radius;
    let y = center.y + z.im as f32 * radius; // Invert Y? Complex plane usually Y is up, screen Y is down.
                                             // Let's keep Y up for now to match standard math orientation if needed, but for visual chaos it doesn't matter much.
                                             // Actually screen Y is down. So +Im -> +Y means Up -> Down.
                                             // Usually we want +Im -> Up (Screen -Y).
                                             // Let's invert imaginary part mapping.
    let y = center.y - z.im as f32 * radius;
    (x, y)
}
