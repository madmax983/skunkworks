use macroquad::prelude::*;

mod glitch;
mod graph;
mod simulation;

use glitch::TextGlitcher;
use graph::Graph;
use simulation::Simulation;

#[macroquad::main("Mnem-DDoS")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt to scan the current crate first
    graph.scan_directory("experiments/mnem-ddos/src");

    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut sim = Simulation::new(graph);

    // Camera
    let mut offset = vec2(screen_width() / 2.0, screen_height() / 2.0);
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
            let zoom_factor = 1.1f32;
            if wheel.1 > 0.0 {
                zoom *= zoom_factor;
            } else {
                zoom /= zoom_factor;
            }
        }

        // Physics & Graph layout
        let mut forces = vec![vec2(0.0, 0.0); sim.graph.nodes.len()];
        for i in 0..sim.graph.nodes.len() {
            for j in i + 1..sim.graph.nodes.len() {
                let diff = sim.graph.nodes[i].pos - sim.graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - sim.graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        for edge in &sim.graph.edges {
            if edge.from < sim.graph.nodes.len() && edge.to < sim.graph.nodes.len() {
                let n1 = sim.graph.nodes[edge.from].pos;
                let n2 = sim.graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 100.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        let dt = get_frame_time();
        let world_mouse = (mouse_vec - offset) / zoom;
        let mut hovered_node = None;

        for (i, node) in sim.graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping
            node.pos += node.vel;

            // Interaction / Healing
            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                node.health += dt * 5.0; // Fast heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // Sim updates
        sim.update();

        // Rendering
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Draw Edges
        for edge in &sim.graph.edges {
            let from = sim.graph.nodes[edge.from].pos * zoom + offset;
            let to = sim.graph.nodes[edge.to].pos * zoom + offset;
            let health_factor = sim.graph.nodes[edge.from]
                .health
                .min(sim.graph.nodes[edge.to].health);
            let color = Color::new(0.3, 0.4, 0.5, 0.5 * health_factor * edge.strength);
            draw_line(
                from.x,
                from.y,
                to.x,
                to.y,
                2.0 * zoom * edge.strength,
                color,
            );
        }

        // Draw Swarm
        for agent in &sim.agents {
            let screen_pos = agent.pos * zoom + offset;
            draw_circle(
                screen_pos.x,
                screen_pos.y,
                1.5 * zoom,
                Color::new(1.0, 0.0, 0.0, 0.6),
            );
        }

        // Draw Nodes
        for (i, node) in sim.graph.nodes.iter().enumerate() {
            let screen_pos = node.pos * zoom + offset;
            let is_hovered = hovered_node == Some(i);

            let color = if is_hovered {
                WHITE
            } else {
                Color::new(1.0 - node.health, node.health, 0.2, 1.0)
            };

            let radius = if is_hovered { 15.0 * zoom } else { 8.0 * zoom };
            draw_circle(screen_pos.x, screen_pos.y, radius, color);

            // Label
            if zoom > 0.5 || is_hovered {
                let glitched_name = TextGlitcher::corrupt(&node.name, 1.0 - node.health);
                draw_text(
                    &glitched_name,
                    screen_pos.x + 10.0 * zoom,
                    screen_pos.y,
                    20.0 * zoom,
                    color,
                );
            }
        }

        // Draw Hover Overlay
        if let Some(idx) = hovered_node {
            let node = &sim.graph.nodes[idx];
            let content = TextGlitcher::corrupt(&node.content, 1.0 - node.health);

            let padding = 20.0;
            draw_rectangle(
                padding,
                padding,
                screen_width() - padding * 2.0,
                screen_height() - padding * 2.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );

            draw_text(&node.name, padding + 10.0, padding + 30.0, 30.0, WHITE);

            let mut y = padding + 60.0;
            for line in content.lines().take(30) {
                draw_text(
                    line,
                    padding + 10.0,
                    y,
                    16.0,
                    Color::new(0.7, 0.8, 0.9, 1.0),
                );
                y += 20.0;
            }
        }

        next_frame().await;
    }
}
