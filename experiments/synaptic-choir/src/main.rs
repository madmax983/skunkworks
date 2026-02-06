mod neuron;
mod population;
mod audio;

use macroquad::prelude::*;
use audio::{AudioEngine, ControlParams, Snapshot, POP_SIZE};
use std::collections::VecDeque;

const HISTORY_LEN: usize = 600;

#[macroquad::main("Synaptic Choir")]
async fn main() -> anyhow::Result<()> {
    // Start Audio
    // We unwrap here for simplicity in this demo, but in prod we handle error
    let mut audio_sys = AudioEngine::new()?;

    // UI State
    let mut mean_field_history: VecDeque<f32> = VecDeque::with_capacity(HISTORY_LEN);
    for _ in 0..HISTORY_LEN {
        mean_field_history.push_back(-65.0);
    }

    let mut last_snapshot = Snapshot {
        voltages: [-65.0; POP_SIZE],
        mean_field: -65.0,
    };

    let mut coupling = 0.0;
    let mut current = 5.0;

    loop {
        // Input
        let (mx, my) = mouse_position();
        let sw = screen_width();
        let sh = screen_height();

        if is_mouse_button_down(MouseButton::Left) {
            // Map X to Current (Frequency)
            // Map Y to Coupling (Synchrony)
            current = (mx / sw) * 40.0; // 0..40
            coupling = 1.0 - (my / sh); // 1..0 (Top is high coupling)
            coupling = coupling.clamp(0.0, 1.0) * 0.2; // Max coupling 0.2 (it gets loud/unstable if higher)
        }

        // Send Controls
        // We ignore error if buffer is full, it's fine, we'll send next frame
        let _ = audio_sys.control_tx.push(ControlParams { current, coupling });

        // Receive Data
        while let Some(snap) = audio_sys.snapshot_rx.pop() {
            last_snapshot = snap;
            mean_field_history.push_back(snap.mean_field);
            if mean_field_history.len() > HISTORY_LEN {
                mean_field_history.pop_front();
            }
        }

        // Render
        clear_background(BLACK);

        // Grid
        let grid_size = (POP_SIZE as f32).sqrt().ceil() as usize; // 10
        let cell_w = sw / grid_size as f32;
        let cell_h = (sh * 0.7) / grid_size as f32;

        for i in 0..POP_SIZE {
            let v = last_snapshot.voltages[i];
            let x = (i % grid_size) as f32 * cell_w;
            let y = (i / grid_size) as f32 * cell_h;

            // Color map
            // -80 (Blue) -> -50 (Green) -> 0 (Yellow) -> 30 (Red)
            let color = if v < -60.0 {
                let t = ((v + 80.0) / 20.0).clamp(0.0, 1.0);
                Color::new(0.0, 0.0, t.max(0.2), 1.0)
            } else if v < -40.0 {
                let t = ((v + 60.0) / 20.0).clamp(0.0, 1.0);
                Color::new(0.0, t, 1.0 - t, 1.0)
            } else {
                 let t = ((v + 40.0) / 70.0).clamp(0.0, 1.0);
                 Color::new(t, 1.0 - t * 0.5, 0.0, 1.0)
            };

            draw_rectangle(x, y, cell_w - 2.0, cell_h - 2.0, color);

            // Draw a flash if spiking
            if v > 0.0 {
                draw_rectangle(x, y, cell_w - 2.0, cell_h - 2.0, WHITE);
            }
        }

        // Waveform
        let wave_y = sh * 0.85;
        draw_rectangle(0.0, sh * 0.7, sw, sh * 0.3, Color::new(0.1, 0.1, 0.1, 1.0));

        let step_x = sw / HISTORY_LEN as f32;
        for i in 0..mean_field_history.len() - 1 {
            let v1 = mean_field_history[i];
            let v2 = mean_field_history[i+1];

            // Map -80..0 to height
            let y1 = wave_y - (v1 + 50.0) * 3.0;
            let y2 = wave_y - (v2 + 50.0) * 3.0;

            draw_line(i as f32 * step_x, y1, (i+1) as f32 * step_x, y2, 2.0, GREEN);
        }

        // Info
        draw_text(&format!("Current: {:.2}", current), 10.0, sh - 40.0, 20.0, WHITE);
        draw_text(&format!("Coupling: {:.4}", coupling), 10.0, sh - 20.0, 20.0, WHITE);
        draw_text("Drag Mouse: X=Current, Y=Coupling", 10.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
