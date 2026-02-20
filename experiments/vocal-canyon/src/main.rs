use macroquad::prelude::*;
use std::sync::Arc;

mod audio;
mod state;

use crate::state::SharedState;
use crate::audio::start_audio;

#[macroquad::main("Vocal Canyon")]
async fn main() {
    // Initialize State
    let num_segments = 44; // Standard approximation for 17.5cm tract at 44.1kHz

    let state = SharedState::new(num_segments);

    // Start Audio
    let _stream = match start_audio(state.clone()) {
        Ok(s) => {
            println!("Audio started successfully.");
            Some(s)
        },
        Err(e) => {
            eprintln!("Failed to start audio: {}", e);
            None
        }
    };

    loop {
        clear_background(BLACK);

        // Input Handling
        let mouse_pos = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let center_y = screen_h / 2.0;

        // Read Params
        let mut params = state.params.write();
        let num_areas = params.areas.len();
        let segment_width = screen_w / num_areas as f32;

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let idx = (mouse_pos.0 / segment_width) as usize;
            if idx < num_areas {
                // Calculate new area based on distance from center
                let dy = (mouse_pos.1 - center_y).abs();
                // Scale: 100 pixels = Area 1.0?
                // Let's say Area 1.0 is "neutral".
                // 50 pixels = 1.0
                let new_area = dy / 50.0;
                params.areas[idx] = new_area.max(0.01);
            }
        }

        // Smoothing (Right Click)
        if is_mouse_button_down(MouseButton::Right) {
             let idx = (mouse_pos.0 / segment_width) as usize;
             // Apply smoothing kernel around mouse
             let radius = 2;
             let start = idx.saturating_sub(radius);
             let end = (idx + radius + 1).min(num_areas);

             // Simple averaging
             for i in start..end {
                 let prev = if i > 0 { params.areas[i-1] } else { params.areas[i] };
                 let next = if i < num_areas - 1 { params.areas[i+1] } else { params.areas[i] };
                 params.areas[i] = (prev + params.areas[i] + next) / 3.0;
             }
        }

        // Controls
        if is_key_pressed(KeyCode::Space) {
            params.is_speaking = !params.is_speaking;
        }

        if is_key_down(KeyCode::Up) {
            params.frequency *= 1.01;
        }
        if is_key_down(KeyCode::Down) {
            params.frequency *= 0.99;
        }

        let is_speaking = params.is_speaking;
        let frequency = params.frequency;

        // Clone areas for drawing to release lock early?
        // No, we are single threaded in main loop, and audio thread only reads briefly.
        // But holding write lock blocks audio.
        // We should release lock before drawing if possible.
        let areas_to_draw = params.areas.clone();
        drop(params);

        // Drawing
        // Draw Canyon Walls
        for (i, area) in areas_to_draw.iter().enumerate() {
            let x = i as f32 * segment_width;
            let h = area * 50.0; // Scale for visual

            // Top Wall
            draw_rectangle(x, 0.0, segment_width, center_y - h, GRAY);
            // Bottom Wall
            draw_rectangle(x, center_y + h, segment_width, center_y, GRAY);

            // Draw "Fluid" / Air inside
            let color = if is_speaking {
                // Modulate color by level?
                let level = state.metrics.get_level();
                let intensity = (level * 5.0).min(1.0);
                Color::new(0.2, 0.2 + intensity * 0.8, 0.2 + intensity * 0.4, 1.0)
            } else {
                DARKGRAY
            };
            draw_rectangle(x, center_y - h, segment_width, h * 2.0, color);
        }

        // Draw UI
        draw_text(format!("Frequency: {:.1} Hz", frequency).as_str(), 10.0, 30.0, 20.0, WHITE);
        draw_text(if is_speaking { "Speaking (SPACE to stop)" } else { "Silent (SPACE to start)" }, 10.0, 50.0, 20.0, WHITE);
        draw_text("Left Click: Carve | Right Click: Smooth | Up/Down: Pitch", 10.0, screen_h - 10.0, 20.0, WHITE);

        next_frame().await
    }
}
