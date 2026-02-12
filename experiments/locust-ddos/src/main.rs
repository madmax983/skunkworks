mod simulation;
use macroquad::prelude::*;
use simulation::{World, WORLD_SIZE};

#[macroquad::main("Locust DDoS")]
async fn main() {
    let mut world = World::new();

    // Texture setup (Fixed internal resolution)
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Input
        let mouse_pos = mouse_position();

        // Map screen mouse to world coords
        // We stretch the 1000x1000 world to fit the screen
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE
        );

        if is_mouse_button_down(MouseButton::Left) {
            // Add firewall every frame is too much, maybe throttle?
            // Or just allow painting barriers.
            // Let's add with a small chance or if moved enough, or just every frame (dense wall)
             world.add_firewall(world_mouse, 20.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Update
        world.update();

        // Render to buffer
        world.render_to_buffer(&mut image.bytes, width, height);
        texture.update(&image);

        // Draw
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

        // Draw Firewalls overlay (visualize them clearly)
        for (pos, radius) in &world.firewalls {
            // Map world pos to screen pos
            let sx = pos.x / WORLD_SIZE * screen_width();
            let sy = pos.y / WORLD_SIZE * screen_height();
            // Aspect ratio might distort circle if screen is not square
            // Use width for radius scale
            let sr = radius / WORLD_SIZE * screen_width();
            draw_circle(sx, sy, sr, Color::new(1.0, 0.0, 0.0, 0.1));
        }

        // Draw Target
        let tx = world.target.x / WORLD_SIZE * screen_width();
        let ty = world.target.y / WORLD_SIZE * screen_height();
        draw_circle(tx, ty, 15.0, BLUE);
        draw_text("SERVER", tx - 30.0, ty - 20.0, 20.0, WHITE);

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Packets: {}", world.agents.len()), 10.0, 60.0, 30.0, WHITE);
        draw_text("Left Click: Deploy Firewall | C: Clear Rules", 10.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
