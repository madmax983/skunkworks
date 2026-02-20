use macroquad::prelude::*;
use chimera_lang::prelude::Value;

mod audio;
mod state;
mod vm_adapter;

use crate::state::SharedState;
use crate::audio::start_audio;
use crate::vm_adapter::{init_vm, update_state};

#[macroquad::main("Chimera Voice")]
async fn main() {
    let num_segments = 44;
    let state = SharedState::new(num_segments);

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

    let mut vm = init_vm();

    loop {
        clear_background(BLACK);

        // VM Step
        // Run multiple steps per frame for speed
        for _ in 0..10 {
            vm.step();
        }

        update_state(&mut vm, &state);

        // Read Params for Drawing
        let params = state.params.read();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let center_y = screen_h / 3.0; // Tract in top half

        let areas = params.areas.clone();
        let freq = params.frequency;
        drop(params);

        // Draw Vocal Tract
        let segment_width = screen_w / areas.len() as f32;
        for (i, area) in areas.iter().enumerate() {
            let x = i as f32 * segment_width;
            let h = area * 30.0;
            // Top
            draw_rectangle(x, 0.0, segment_width, center_y - h, GRAY);
            // Bottom
            draw_rectangle(x, center_y + h, segment_width, center_y, GRAY);
            // Air
            draw_rectangle(x, center_y - h, segment_width, h * 2.0, DARKBLUE);
        }

        // Draw VM Grid (Bottom Half)
        let grid_size = 16;
        let cell_size = (screen_h / 2.0) / grid_size as f32;
        let grid_start_y = screen_h / 2.0;
        let grid_start_x = (screen_w - (cell_size * grid_size as f32)) / 2.0;

        for y in 0..grid_size {
            for x in 0..grid_size {
                let val = &vm.grid[y][x];
                let color = match val {
                    Value::Int(0) => BLACK,
                    Value::Int(_) => GREEN,
                    Value::Str(_) => PURPLE,
                    _ => WHITE,
                };
                draw_rectangle(
                    grid_start_x + x as f32 * cell_size,
                    grid_start_y + y as f32 * cell_size,
                    cell_size - 1.0,
                    cell_size - 1.0,
                    color,
                );
            }
        }

        // UI
        draw_text(format!("Freq: {:.1} Hz", freq).as_str(), 10.0, 20.0, 20.0, WHITE);
        draw_text(format!("Energy: {}", vm.energy).as_str(), 10.0, 40.0, 20.0, WHITE);
        draw_text("Space: Reset VM", 10.0, screen_h - 20.0, 20.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            vm = init_vm();
        }

        next_frame().await
    }
}
