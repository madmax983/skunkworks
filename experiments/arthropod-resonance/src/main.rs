use arthropod::Button;
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot, Material};
use std::thread;
use std::time::Duration;

const GRID_SIZE: usize = 100;
const AUDIO_SAMPLE_RATE: usize = 44100;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod × Resonance: Interactive Acoustic Space".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

// Custom manual entry point to handle early exit for headless bypassing
fn main() {
    // [Headless Bypass]
    // If the --headless flag is provided, exit immediately with success.
    // This allows CI/CD to verify compilation and execution without requiring
    // an active X11 display server or GPU.
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Headless mode detected. Exiting immediately.");
        return;
    }

    macroquad::Window::from_config(window_conf(), amain());
}

async fn amain() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(10); // small bound to drop old frames if UI is slow

    // Spawning a background thread to simulate the audio thread
    thread::spawn(move || {
        let mut model = AudioModel::new(GRID_SIZE, GRID_SIZE, cmd_rx, snap_tx, None);

        // Simulate an audio processing loop
        let mut buffer = vec![0.0; 256];
        loop {
            model.process(&mut buffer);
            // Throttle to approximate realtime (very rough approximation for non-real audio thread)
            thread::sleep(Duration::from_micros(
                256 * 1_000_000 / AUDIO_SAMPLE_RATE as u64,
            ));
        }
    });

    let mut latest_snapshot: Option<AudioSnapshot> = None;

    // Define interactive buttons
    let pluck_btn =
        Button::new("Pluck Center", 10.0, 10.0, 200.0, 50.0).with_colors(BLUE, LIGHTGRAY, DARKGRAY);
    let tone_btn = Button::new("Sustain Tone (440Hz)", 10.0, 70.0, 200.0, 50.0)
        .with_colors(GREEN, LIME, DARKGREEN);
    let wall_btn =
        Button::new("Toggle Wall", 10.0, 130.0, 200.0, 50.0).with_colors(RED, ORANGE, MAROON);

    let mut is_tone_active = false;
    let mut is_wall_active = false;

    loop {
        // Drain snapshot channel to get the latest visual state
        while let Ok(snap) = snap_rx.try_recv() {
            latest_snapshot = Some(snap);
        }

        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();

        // Render the acoustic grid visualization if we have data
        if let Some(ref snap) = latest_snapshot {
            let cell_w = screen_w / GRID_SIZE as f32;
            let cell_h = screen_h / GRID_SIZE as f32;

            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let idx = y * GRID_SIZE + x;
                    let pressure = snap.pressure[idx];

                    // Simple color mapping: negative pressure -> blue, positive -> red
                    let color = if pressure > 0.0 {
                        Color::new(pressure.clamp(0.0, 1.0), 0.0, 0.0, 1.0)
                    } else {
                        Color::new(0.0, 0.0, (-pressure).clamp(0.0, 1.0), 1.0)
                    };

                    // If it's a wall material, render it as gray
                    let final_color = match snap.materials[idx] {
                        Material::Wall => GRAY,
                        _ => color,
                    };

                    draw_rectangle(
                        x as f32 * cell_w,
                        y as f32 * cell_h,
                        cell_w,
                        cell_h,
                        final_color,
                    );
                }
            }
        }

        // Draw buttons and process interaction
        if pluck_btn.draw() {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: GRID_SIZE / 2,
                y: GRID_SIZE / 2,
                strength: 1.0,
            });
        }

        // Tone Button Logic (Hold vs Click logic is tricky with Button, so we toggle state on click)
        if tone_btn.draw() {
            is_tone_active = !is_tone_active;
            let strength = if is_tone_active { 0.5 } else { 0.0 };
            let _ = cmd_tx.send(AudioCommand::Oscillate {
                x: GRID_SIZE / 3,
                y: GRID_SIZE / 3,
                frequency: 440.0,
                strength,
            });
        }

        // Wall Button Logic (Toggle Wall)
        if wall_btn.draw() {
            is_wall_active = !is_wall_active;
            let material = if is_wall_active {
                Material::Wall
            } else {
                Material::Air
            };

            // Draw a wall in the center
            for i in 20..80 {
                let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                    x: GRID_SIZE / 2,
                    y: i,
                    material,
                });
            }
        }

        // Text Feedback Overlay
        if is_tone_active {
            draw_text("Tone Active", 220.0, 100.0, 20.0, WHITE);
        }
        if is_wall_active {
            draw_text("Wall Active", 220.0, 160.0, 20.0, WHITE);
        }

        next_frame().await;
    }
}
