use macroquad::prelude::*;

mod graph;
use graph::Graph;
mod glitch;
use glitch::TextGlitcher;

#[macroquad::main("Mnemonic Rot")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt to scan the current crate first
    graph.scan_directory("experiments/mnem-rot/src");

    // If that fails (e.g., path diff), try current dir but limit depth in mind (scan_directory doesn't limit depth yet but it's fine for small repos)
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize> = None;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

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
                    // Optimization radius
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

        // 2. Apply Forces & Decay
        let dt = get_frame_time();
        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping
            node.pos += node.vel;

            // Decay
            node.health -= entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            // Interaction
            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                // Heal
                node.health += dt * 5.0; // Fast heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // Render
        clear_background(BLACK);

        // Draw Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                // Jitter if low health
                let jitter = if avg_health < 0.5 {
                    vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    2.0 * zoom,
                    Color::new(0.5, 0.5, 0.5, avg_health),
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

            // Draw name if healthy enough or hovered
            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 10.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        // UI Overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            // Panel
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

            // Content
            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            // Render text lines
            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                // Wrap text manually or just truncate
                let display_line = if line.len() > 50 { &line[0..50] } else { line };
                draw_text(display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
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
            "Right Click: Pan | Scroll: Zoom | Hover: Heal",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await
    }
}
