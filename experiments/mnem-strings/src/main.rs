use macroquad::prelude::*;

mod graph;
use graph::Graph;
mod glitch;
use glitch::TextGlitcher;
mod audio;
use audio::{init_audio, AudioCommand};

#[macroquad::main("Mnemonic Strings")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    graph.scan_directory("experiments/mnem-strings/src");

    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    // Initialize audio
    let (_audio_handle, cmd_tx) = init_audio().expect("Failed to initialize audio");

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize>;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    // Track visual vibrations for edges
    let mut edge_vibrations = vec![0.0; graph.edges.len()];

    loop {
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        // Input: Camera
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_vec;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_vec - last_mouse;
            offset += delta;
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
        }

        // Logic: Decay
        entropy += 0.0001; // Slow global rot

        // Physics & Decay Loop
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        // Repulsion
        for i in 0..graph.nodes.len() {
            for j in i + 1..graph.nodes.len() {
                let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            // Center attraction (Gravity)
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        // Spring forces
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 100.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        let dt = get_frame_time();
        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        // Ensure edge vibrations array is large enough
        if edge_vibrations.len() < graph.edges.len() {
            edge_vibrations.resize(graph.edges.len(), 0.0);
        }

        let mut plucks = Vec::new();

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            let mut vel = forces[i] * dt;
            vel *= 0.90; // Damping

            let mut health_change = -entropy * dt * 0.05;
            let new_pos = node.pos + vel;

            let mut node_healed = false;
            let prev_health = node.health;

            if new_pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                health_change += dt * 5.0; // Fast heal
                node_healed = true;
            }

            node.vel = vel;
            node.pos = new_pos;
            node.health = (prev_health + health_change).clamp(0.1, 1.0);

            // If health increased significantly or just triggered, queue pluck
            if node_healed && node.health - prev_health > dt * 2.0 && rand::gen_range(0.0, 1.0) < 0.1 {
                plucks.push(i);
            }
        }

        // Apply plucks
        for node_idx in plucks {
            for (e_idx, edge) in graph.edges.iter().enumerate() {
                if edge.from == node_idx || edge.to == node_idx {
                    let other_idx = if edge.from == node_idx { edge.to } else { edge.from };
                    if other_idx < graph.nodes.len() {
                        let other_node = &graph.nodes[other_idx];
                        let avg_health = (graph.nodes[node_idx].health + other_node.health) / 2.0;
                        let dist = graph.nodes[node_idx].pos.distance(other_node.pos).max(10.0);

                        let base_freq = 440.0 * (100.0 / dist).clamp(0.5, 2.0);
                        let health_mod = avg_health * 0.5 + 0.5; // 0.5 to 1.0 multiplier
                        let frequency = base_freq * health_mod;

                        let decay = 0.95 + (avg_health * 0.045); // 0.95 (dissonant rot) to 0.995 (clear ring)
                        let amplitude = 0.3 * edge.strength;

                        // Pluck audio string
                        let _ = cmd_tx.send(AudioCommand::Pluck {
                            frequency,
                            decay,
                            amplitude,
                        });

                        // Pluck visual string
                        edge_vibrations[e_idx] = 1.0;
                    }
                }
            }
        }

        // Render
        clear_background(BLACK);

        // Draw Edges (Strings)
        for (i, edge) in graph.edges.iter().enumerate() {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health = (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                // Visual vibration damping
                edge_vibrations[i] *= 0.90;

                // Jitter (Rot/Entropy) + Vibration (Pluck)
                let jitter_amount = (1.0 - avg_health) * 2.0;
                let vibration_amount = edge_vibrations[i] * 5.0;

                let total_jitter = jitter_amount + vibration_amount;

                let jitter = if total_jitter > 0.01 {
                    vec2(rand::gen_range(-total_jitter, total_jitter), rand::gen_range(-total_jitter, total_jitter))
                } else {
                    vec2(0.0, 0.0)
                };

                let string_color = if avg_health > 0.8 {
                    Color::new(0.5, 0.8, 1.0, avg_health) // tuned (blueish)
                } else if avg_health > 0.4 {
                    Color::new(0.8, 0.8, 0.5, avg_health) // degrading (yellowish)
                } else {
                    Color::new(0.8, 0.3, 0.3, 0.8) // rotted (red)
                };

                // Draw string with vibration/jitter applied to the midpoint to simulate a sine wave fundamental mode
                let mid = (n1 + n2) / 2.0 + jitter * zoom;

                draw_line(
                    n1.x, n1.y,
                    mid.x, mid.y,
                    2.0 * zoom,
                    string_color,
                );
                draw_line(
                    mid.x, mid.y,
                    n2.x, n2.y,
                    2.0 * zoom,
                    string_color,
                );
            }
        }

        // Draw Nodes
        for (i, node) in graph.nodes.iter().enumerate() {
            let pos = node.pos * zoom + offset;
            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            let radius = 5.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 10.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        // UI Overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            draw_rectangle(
                10.0,
                10.0,
                400.0,
                screen_height() - 20.0,
                Color::new(0.05, 0.05, 0.05, 0.95),
            );
            draw_rectangle_lines(10.0, 10.0, 400.0, screen_height() - 20.0, 2.0, WHITE);

            draw_text(&node.name, 20.0, 40.0, 30.0, GREEN);
            draw_text(
                &format!("Health: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                let display_line: String = line.chars().take(50).collect();
                draw_text(&display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text(
            &format!("Entropy: {:.4}", entropy),
            screen_width() - 200.0,
            30.0,
            20.0,
            RED,
        );
        draw_text(
            "Right Click: Pan | Scroll: Zoom | Hover: Heal & Pluck Strings",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await
    }
}
