use macroquad::prelude::*;
use petgraph::visit::EdgeRef;

mod graph;
mod pheromone;
mod ant;
mod simulation;

use simulation::Simulation;

#[macroquad::main("Cargo Ants")]
async fn main() {
    let mut sim = Simulation::new(screen_width(), screen_height());

    loop {
        if is_key_pressed(KeyCode::R) {
            sim = Simulation::new(screen_width(), screen_height());
        }

        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Darker background

        sim.step();

        // Draw Edges
        for edge in sim.graph.graph.edge_references() {
            let start = sim.graph.graph[edge.source()].position;
            let end = sim.graph.graph[edge.target()].position;

            let p = sim.pheromones.get(edge.id());
            let intensity = (p / 20.0).clamp(0.0, 1.0);

            // Pheromone trail: Cyan/Green
            let color = Color::new(0.0, 0.5 + intensity * 0.5, 0.5 + intensity * 0.5, 0.1 + intensity * 0.9);

            draw_line(start.x, start.y, end.x, end.y, 1.0 + intensity * 3.0, color);
        }

        // Draw Nodes
        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        for node in sim.graph.graph.node_weights() {
            let color = if node.is_root {
                BLUE
            } else if node.is_conflict {
                RED
            } else {
                GRAY
            };
            draw_circle(node.position.x, node.position.y, 5.0, color);

            if node.position.distance(mouse_pos) < 10.0 {
                draw_text(&format!("{} v{}", node.name, node.version),
                          node.position.x + 10.0, node.position.y - 10.0,
                          20.0, WHITE);
            }
        }

        // Draw Ants
        for ant in &sim.ants {
            let color = match ant.state {
                ant::AntState::Searching => Color::new(1.0, 1.0, 1.0, 0.8),
                ant::AntState::Returning => Color::new(0.0, 1.0, 0.0, 0.8), // Green carrying package
                ant::AntState::Dead => Color::new(1.0, 0.0, 0.0, 0.5),
            };
            draw_circle(ant.position.x, ant.position.y, 2.0, color);
        }

        // HUD
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Ants: {}", sim.ants.len()), 10.0, 40.0, 20.0, WHITE);
        draw_text("Press 'R' to reset graph", 10.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
