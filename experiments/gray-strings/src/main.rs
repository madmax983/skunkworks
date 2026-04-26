use gray_scott::GrayScott;
use macroquad::prelude::*;

mod audio;
mod string;

use audio::{init_audio, AudioCommand};
use string::GrayString;

const STRING_COUNT: usize = 8;
const STRING_SPACING: f32 = 100.0;
const BASE_FREQ: f32 = 110.0; // A2

#[macroquad::main("Gray Strings")]
async fn main() {
    let (audio_handle, cmd_tx) = init_audio().expect("Failed to init audio");
    let _audio_handle = audio_handle;

    let grid_w = 200;
    let grid_h = 150;
    let mut gs = GrayScott::new(grid_w, grid_h);
    let grid_scale = 4.0;

    // Seed the simulation
    gs.add_chemical(grid_w / 2, grid_h / 2, 1.0);

    let mut strings: Vec<GrayString> = Vec::new();

    // Initialize strings
    for i in 0..STRING_COUNT {
        let x = 100.0 + i as f32 * STRING_SPACING;
        let pos = vec2(x, 200.0);
        let target_freq = BASE_FREQ * (2.0f32).powf(i as f32 / 12.0);
        strings.push(GrayString::new(pos, 300.0, target_freq));
    }

    let texture =
        Texture2D::from_image(&Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest);

    let mut prev_mouse = vec2(0.0, 0.0);

    // Coral / Labyrinths params
    let mut feed = 0.0545;
    let mut kill = 0.0620;

    loop {
        let dt = get_frame_time();

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_delta = mouse_pos - prev_mouse;
        let mouse_speed = mouse_delta.length();

        // 1. Update Gray-Scott
        gs.update(feed, kill, 1.0);

        // 2. Update Strings
        for s in &mut strings {
            // Read from GrayScott
            s.update_physics(dt, &gs, grid_scale);

            // Write to GrayScott
            s.perturb_grid(&mut gs, grid_scale);

            // Mouse Plucking
            let string_x = s.pos.x + s.vibration;
            let crossed = (prev_mouse.x < string_x && mouse_pos.x >= string_x)
                || (prev_mouse.x > string_x && mouse_pos.x <= string_x);
            let in_range = mouse_pos.y >= s.pos.y && mouse_pos.y <= s.pos.y + s.length;

            if crossed && in_range {
                let strength = mouse_speed.clamp(5.0, 50.0);
                let direction = if mouse_delta.x > 0.0 { 1.0 } else { -1.0 };
                s.pluck(strength * direction);

                let _ = cmd_tx.send(AudioCommand {
                    frequency: s.frequency,
                    decay: s.decay,
                    amplitude: (strength / 50.0).clamp(0.1, 0.8),
                });
            }
        }

        // --- Render ---
        clear_background(BLACK);

        // Render Heatmap
        let mut image = texture.get_texture_data();
        let v_data = gs.v();
        for y in 0..grid_h {
            for x in 0..grid_w {
                let idx = y * grid_w + x;
                let val = v_data[idx];

                // Color mapping: Purple/Blue for V concentration
                let color = if val > 0.01 {
                    Color::new(val.min(1.0), 0.0, val.min(1.0), 1.0)
                } else {
                    BLACK
                };

                image.set_pixel(x as u32, y as u32, color);
            }
        }
        texture.update(&image);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(grid_w as f32 * grid_scale, grid_h as f32 * grid_scale)),
                ..Default::default()
            },
        );

        // Draw Strings
        for s in &strings {
            s.draw();
        }

        // Draw UI
        draw_text("Gray Strings", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Strings vibrate -> Perturb Reaction-Diffusion",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Reaction-Diffusion -> Alters String Tension",
            10.0,
            70.0,
            20.0,
            GRAY,
        );
        draw_text("Pluck with Mouse!", 10.0, 90.0, 20.0, YELLOW);

        // Parameter controls
        draw_text(
            format!("Feed: {:.4} (UP/DOWN)", feed).as_str(),
            10.0,
            120.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            format!("Kill: {:.4} (LEFT/RIGHT)", kill).as_str(),
            10.0,
            140.0,
            20.0,
            LIGHTGRAY,
        );

        if is_key_down(KeyCode::Up) {
            feed += 0.0001;
        }
        if is_key_down(KeyCode::Down) {
            feed -= 0.0001;
        }
        if is_key_down(KeyCode::Right) {
            kill += 0.0001;
        }
        if is_key_down(KeyCode::Left) {
            kill -= 0.0001;
        }

        prev_mouse = mouse_pos;
        next_frame().await;
    }
}
