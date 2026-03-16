mod audio;
mod simulation;
mod string;

use macroquad::prelude::*;
use simulation::{World, WORLD_SIZE};
use audio::{init_audio, AudioCommand};
use string::FerrousString;

/// 🧬 Lineage: experiments/locust-strings
///
/// This experiment crosses the visual macroquad botnet swarm from `locust-ddos`
/// with the vibrating magnetic string physics of `ferrous-strings`.
///
/// Novel trait: Acoustic Botnet. The DDoS packets act as kinetic agents that, as they
/// navigate through the environment towards the server, collide with the strings.
/// This collision deposits kinetic energy (plucks), translating the digital attack
/// into high-fidelity acoustic oscillations mapped onto a magnetic field.

#[macroquad::main("Locust Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    // Keep handle alive
    let _audio_handle = audio_handle;

    let mut world = World::new(cmd_tx);

    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // Input
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.add_firewall(world_mouse, 20.0);
        }

        if is_key_pressed(KeyCode::C) {
            world.clear_firewalls();
        }

        // Update
        world.update();

        // Render buffer (platter, agents, pheromones)
        world.render_to_buffer(&mut image.bytes, width, height);
        texture.update(&image);

        // Draw texture
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
            draw_circle(sx, sy, sr, Color::new(1.0, 0.0, 0.0, 0.2));
        }

        // Draw Server Target
        let tx = world.target.x / WORLD_SIZE * screen_width();
        let ty = world.target.y / WORLD_SIZE * screen_height();

        let health_pct = (world.server_health / world.max_health).clamp(0.0, 1.0);
        let server_color = Color::new(1.0 - health_pct, 0.0, health_pct, 1.0);

        draw_circle(tx, ty, 15.0, server_color);
        draw_text("SERVER", tx - 30.0, ty - 25.0, 20.0, WHITE);
        draw_rectangle(tx - 40.0, ty + 20.0, 80.0, 8.0, RED);
        draw_rectangle(tx - 40.0, ty + 20.0, 80.0 * health_pct, 8.0, GREEN);

        // Draw Strings
        // Need to scale string positions appropriately
        for string in &world.strings {
            let start = string.pos;
            let end = string.pos + vec2(0.0, string.length);

            // Draw segment based
            let segments = 20;
            let step = string.length / segments as f32;

            let mut prev_x = (start.x / WORLD_SIZE) * screen_width();
            let mut prev_y = (start.y / WORLD_SIZE) * screen_height();

            let thickness = 3.0;

            let charge = (string.vibration / 20.0).clamp(-1.0, 1.0);
            let mut color = string.color;
            if charge > 0.0 {
                color = Color::new(1.0, 1.0 - charge, 1.0 - charge, 1.0);
            } else {
                color = Color::new(1.0 + charge, 1.0 + charge, 1.0, 1.0);
            }

            for i in 1..=segments {
                let y_offset = i as f32 * step;
                let ratio = y_offset / string.length;
                let shape = (std::f32::consts::PI * ratio).sin();
                let x = string.pos.x + string.vibration * shape;
                let y = string.pos.y + y_offset;

                let current_x = (x / WORLD_SIZE) * screen_width();
                let current_y = (y / WORLD_SIZE) * screen_height();

                draw_line(prev_x, prev_y, current_x, current_y, thickness, color);
                prev_x = current_x;
                prev_y = current_y;
            }

            // Draw frequency text
            draw_text(
                &format!("{:.1}Hz", string.frequency),
                (start.x / WORLD_SIZE) * screen_width() - 20.0,
                (end.y / WORLD_SIZE) * screen_height() + 20.0,
                16.0,
                LIGHTGRAY,
            );
        }

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Packets: {}", world.agents.len()),
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
            "Left Click: Deploy Firewall | C: Clear Rules",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
