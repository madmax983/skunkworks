use macroquad::prelude::*;
use std::env;

mod etymology;
mod phonology;
mod visuals;

use etymology::trace_history;
use visuals::draw_river;

#[macroquad::main("Git Etymologist")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let repo_path = ".";
    // Default to Cargo.toml if no file specified
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "Cargo.toml"
    };

    let river = match trace_history(repo_path, file_path) {
        Ok(r) => Some(r),
        Err(e) => {
            eprintln!("Error tracing history: {}", e);
            None
        }
    };

    let mut offset = vec2(50.0, 50.0);
    let mut zoom: f32 = 1.0;
    let mut dragging = false;
    let mut last_mouse_pos = vec2(0.0, 0.0);

    loop {
        clear_background(BLACK);

        if let Some(ref r) = river {
            // Input
            if is_mouse_button_pressed(MouseButton::Left) {
                dragging = true;
                last_mouse_pos = mouse_position().into();
            }
            if is_mouse_button_released(MouseButton::Left) {
                dragging = false;
            }
            if dragging {
                let current_mouse_pos: Vec2 = mouse_position().into();
                let delta = current_mouse_pos - last_mouse_pos;
                offset += delta;
                last_mouse_pos = current_mouse_pos;
            }

            let (_, wheel) = mouse_wheel();
            if wheel != 0.0 {
                zoom *= if wheel > 0.0 { 1.1 } else { 0.9 };
                zoom = zoom.clamp(0.1, 5.0);
            }

            draw_river(r, offset, zoom);

            // Legend
            draw_text("Code River", 10.0, 30.0, 30.0, WHITE);
            draw_text(&format!("File: {}", file_path), 10.0, 50.0, 20.0, LIGHTGRAY);
            draw_text(
                "Green: Stable | Red: Mutation",
                10.0,
                screen_height() - 20.0,
                20.0,
                GRAY,
            );
        } else {
            draw_text("Failed to load history.", 20.0, 50.0, 30.0, RED);
            draw_text(&format!("File: {}", file_path), 20.0, 80.0, 20.0, WHITE);
            draw_text(
                "Usage: git-etymologist <file_path>",
                20.0,
                110.0,
                20.0,
                GRAY,
            );
            draw_text(
                "Ensure the file exists in the git index.",
                20.0,
                140.0,
                20.0,
                GRAY,
            );
        }

        next_frame().await
    }
}
