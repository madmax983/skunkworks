use macroquad::prelude::*;
use std::env;

mod audio;
mod scanner;
mod visuals;

use audio::{init_audio, AudioCommand};
use scanner::scan_directory;
use visuals::StringVisual;

const STRING_SPACING: f32 = 15.0;
const STRING_LENGTH: f32 = 400.0;
const TOP_MARGIN: f32 = 100.0;

#[macroquad::main("String Theory")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    println!("Scanning {}...", path);
    let files = scan_directory(path);
    println!("Found {} files.", files.len());

    let (audio_stream, cmd_tx) = match init_audio() {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Failed to init audio: {}", e);
            return;
        }
    };
    // Keep audio stream alive for the duration of the program
    // We don't need to manage its lifetime explicitly as we run until exit
    // But dropping it stops audio, so we move it or forget it.
    // However, if we just let it fall out of scope it drops.
    // Ideally we store it in a struct or just box leak it.
    let _audio_stream = audio_stream;

    let mut visuals: Vec<StringVisual> = files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let x = i as f32 * STRING_SPACING + 50.0;
            let color = match f.extension.as_deref() {
                Some("rs") => ORANGE,
                Some("toml") => BROWN,
                Some("md") => LIGHTGRAY,
                Some("json") => YELLOW,
                Some("lock") => DARKGRAY,
                _ => BLUE,
            };
            StringVisual::new(
                vec2(x, TOP_MARGIN),
                STRING_LENGTH,
                color,
                f.path
                    .file_name()
                    .unwrap_or(std::ffi::OsStr::new(""))
                    .to_string_lossy()
                    .to_string(),
            )
        })
        .collect();

    let mut camera_x = 0.0;
    let (mx, my) = mouse_position();
    let mut prev_mouse = vec2(mx, my);

    loop {
        clear_background(BLACK);

        let (mx, my) = mouse_position();
        let mouse_pos = vec2(mx, my);
        let mouse_delta = mouse_pos - prev_mouse;
        let mouse_speed = mouse_delta.length();

        // Scroll handling
        if is_key_down(KeyCode::Right) {
            camera_x += 10.0;
        }
        if is_key_down(KeyCode::Left) {
            camera_x -= 10.0;
        }
        let (_, wheel_y) = mouse_wheel();
        camera_x -= wheel_y * 20.0;

        let max_scroll = (visuals.len() as f32 * STRING_SPACING - screen_width() + 100.0).max(0.0);
        camera_x = camera_x.clamp(0.0, max_scroll);

        // Update and Draw Strings
        for (i, v) in visuals.iter_mut().enumerate() {
            // Apply camera offset
            let original_x = i as f32 * STRING_SPACING + 50.0;
            v.pos.x = original_x - camera_x;

            // Optimization: Skip off-screen strings
            if v.pos.x < -20.0 || v.pos.x > screen_width() + 20.0 {
                continue;
            }

            // Physics Update
            v.update(get_frame_time());

            // Interaction
            // Check if mouse crossed the string
            // Check relative to the string X + vibration
            let string_x = v.pos.x + v.vibration;
            let crossed = (prev_mouse.x < string_x && mouse_pos.x >= string_x)
                || (prev_mouse.x > string_x && mouse_pos.x <= string_x);

            // Check vertical bounds
            let in_vertical_range = mouse_pos.y >= v.pos.y && mouse_pos.y <= v.pos.y + v.length;

            if crossed && in_vertical_range {
                // Pluck strength based on mouse speed
                // Normalize speed somewhat, 10.0 is slow, 100.0 is fast
                let strength = mouse_speed.clamp(5.0, 50.0);

                // Direction of pluck
                let direction = if mouse_delta.x > 0.0 { 1.0 } else { -1.0 };
                v.pluck(strength * direction * 2.0); // Visual pluck

                // Trigger Audio
                let file_data = &files[i];
                let freq = file_data.get_frequency();
                let decay = file_data.get_decay();
                // Amplitude based on pluck strength
                let amp = (strength / 50.0).clamp(0.2, 0.9);

                let _ = cmd_tx.send(AudioCommand::Pluck {
                    frequency: freq,
                    decay,
                    amplitude: amp,
                });
            }

            // Hover for UI
            v.check_hover(mouse_pos);

            v.draw();
        }

        // UI Overlay
        if let Some(hovered) = visuals.iter().find(|v| v.is_hovered) {
            // Find corresponding file data for more info?
            // We'd need to lookup by label or store index in visual.
            // For now just show label.
            draw_text(&format!("File: {}", hovered.label), 10.0, 30.0, 30.0, WHITE);
        } else {
            draw_text("String Theory: Pluck the Codebase", 10.0, 30.0, 30.0, WHITE);
        }

        draw_text(
            "Arrows/Scroll: Navigate | Mouse: Pluck",
            10.0,
            screen_height() - 10.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Files: {}", visuals.len()),
            screen_width() - 150.0,
            30.0,
            20.0,
            DARKGRAY,
        );

        prev_mouse = mouse_pos;
        next_frame().await;
    }
}
