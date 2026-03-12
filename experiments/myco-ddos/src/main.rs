mod simulation;
use macroquad::prelude::*;
use simulation::{World, WORLD_SIZE};

#[macroquad::main("Myco-DDoS: Mycelial Cyberwarfare")]
async fn main() {
    let mut world = World::new();

    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.add_firewall(world_mouse, 30.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        world.update();

        world.render_to_buffer(&mut image.bytes, width, height);
        texture.update(&image);

        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Draw Firewalls overlay
        for (pos, radius) in &world.firewalls {
            let sx = pos.x / WORLD_SIZE * screen_width();
            let sy = pos.y / WORLD_SIZE * screen_height();
            let sr = radius / WORLD_SIZE * screen_width();
            draw_circle(sx, sy, sr, Color::new(1.0, 0.0, 0.0, 0.1));
            // Outer ring
            draw_circle_lines(sx, sy, sr, 2.0, Color::new(1.0, 0.2, 0.2, 0.3));
        }

        // Draw Target (Server)
        let tx = world.target.x / WORLD_SIZE * screen_width();
        let ty = world.target.y / WORLD_SIZE * screen_height();

        let health_pct = (world.server_health / world.max_health).clamp(0.0, 1.0);
        let server_color = Color::new(1.0 - health_pct, 0.0, health_pct, 1.0); // Blue -> Red

        draw_circle(tx, ty, 20.0, server_color);
        draw_text("SERVER", tx - 35.0, ty - 30.0, 24.0, WHITE);

        // Health Bar
        draw_rectangle(tx - 40.0, ty + 25.0, 80.0, 10.0, RED);
        draw_rectangle(tx - 40.0, ty + 25.0, 80.0 * health_pct, 10.0, GREEN);

        // UI Text
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Botnet Nodes: {}", world.agents.len()),
            10.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Server Health: {:.1}%", health_pct * 100.0),
            10.0,
            90.0,
            30.0,
            if health_pct < 0.2 { RED } else { WHITE },
        );
        draw_text(
            "Left Click: Deploy Firewall | C: Clear Firewalls",
            10.0,
            screen_height() - 20.0,
            24.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
