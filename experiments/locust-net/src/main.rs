use macroquad::prelude::*;
use locust_net::World;

#[macroquad::main("Locust Net")]
async fn main() {
    // Wild Mode: Dark background
    let bg_color = Color::new(0.05, 0.05, 0.05, 1.0);

    let mut world = World::new(screen_width(), screen_height());

    // Add some initial servers (Crops)
    for _ in 0..5 {
        world.add_server(
            rand::gen_range(100.0, screen_width() - 100.0),
            rand::gen_range(100.0, screen_height() - 100.0)
        );
    }

    loop {
        // Handle input: Spawn swarm on click
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Spawn a burst of locusts (SCALED UP)
            for _ in 0..500 {
                // Pick a random target server
                let target_idx = if world.servers.is_empty() {
                    None
                } else {
                    Some(rand::gen_range(0, world.servers.len()))
                };

                // Add jitter to spawn position
                let jx = mx + rand::gen_range(-50.0, 50.0);
                let jy = my + rand::gen_range(-50.0, 50.0);

                world.add_locust(jx, jy, target_idx);
            }
        }

        // Update physics
        let dt = get_frame_time();
        world.update(dt);

        // Draw
        clear_background(bg_color);

        // Draw Servers (Crops)
        for server in &world.servers {
            let color = if server.health > 50.0 {
                GREEN
            } else if server.health > 0.0 {
                YELLOW
            } else {
                RED
            };
            draw_circle(server.position.x, server.position.y, 10.0, color);
            // Draw health bar
            draw_line(
                server.position.x - 12.0,
                server.position.y - 15.0,
                server.position.x - 12.0 + (24.0 * (server.health / 100.0)),
                server.position.y - 15.0,
                2.0,
                color
            );
        }

        // Draw Locusts (Packets) - Optimized Rendering
        for locust in &world.locusts {
            // Visualize velocity as color intensity or length?
            // Just simple pixels for massive counts
            draw_rectangle(locust.position.x, locust.position.y, 1.5, 1.5, Color::new(0.8, 0.8, 1.0, 0.5));
        }

        // Draw HUD
        draw_text(
            &format!("Locusts: {}", world.locusts.len()),
            20.0,
            20.0,
            20.0,
            WHITE
        );
        draw_text(
            "Click to SPAWN SWARM",
            20.0,
            40.0,
            20.0,
            LIGHTGRAY
        );

        next_frame().await;
    }
}
