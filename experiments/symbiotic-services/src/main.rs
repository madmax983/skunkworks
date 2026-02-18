mod simulation;

use macroquad::prelude::*;
use simulation::World;

fn window_conf() -> Conf {
    Conf {
        window_title: "Symbiotic Services".to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());

    // Spawn some initial nodes
    for _ in 0..5 {
        world.spawn_node();
    }

    loop {
        let dt = get_frame_time();

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            let m_pos = mouse_position();
            world.add_load_at(vec2(m_pos.0, m_pos.1));
        }

        if is_key_pressed(KeyCode::Space) {
            world.spawn_node();
        }

        // Update
        world.update(dt);

        // Draw
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0)); // Deep blue-black

        // Draw Hyphae (Connections)
        for hypha in &world.hyphae {
            // Pulse effect based on flow?
            // For now just white lines
            let alpha = (hypha.width / 5.0).clamp(0.2, 0.8);
            draw_line(
                hypha.start.x, hypha.start.y,
                hypha.end.x, hypha.end.y,
                hypha.width,
                Color::new(0.8, 0.8, 0.9, alpha),
            );
        }

        // Draw Particles (Traffic)
        for p in &world.particles {
            if p.active {
                draw_circle(p.pos.x, p.pos.y, 2.0, YELLOW);
            }
        }

        // Draw Nodes (Services)
        for node in &world.nodes {
            // Color based on load
            // Green (Healthy) -> Red (Overload)
            let load_ratio = (node.load / node.capacity).clamp(0.0, 1.0);
            let color = if node.is_gateway {
                Color::new(0.2, 0.8, 1.0, 1.0) // Cyan for Gateway
            } else {
                Color::new(
                    load_ratio,           // R increases with load
                    1.0 - load_ratio,     // G decreases with load
                    0.2,                  // B constant
                    1.0
                )
            };

            // Pulse radius
            let pulse = (get_time() * 5.0).sin() as f32 * 0.5;
            draw_circle(node.pos.x, node.pos.y, node.radius + pulse, color);

            // Draw Health ring
            draw_circle_lines(node.pos.x, node.pos.y, node.radius + 2.0, 1.0, WHITE);
        }

        // UI Overlay
        draw_text("Symbiotic Services", 20.0, 30.0, 30.0, WHITE);
        draw_text("Space: Spawn Node | Click: Add Traffic", 20.0, 60.0, 20.0, GRAY);
        draw_text(&format!("Nodes: {}", world.nodes.len()), 20.0, 90.0, 20.0, GRAY);
        draw_text(&format!("Connections: {}", world.hyphae.len()), 20.0, 120.0, 20.0, GRAY);

        next_frame().await
    }
}
