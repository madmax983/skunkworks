//! # Arthropod Resonance 🌊
//!
//! **Concept:** Interactive Acoustic Morphogenesis.
//!
//! This hybrid visualizer crosses the immediate-mode UI library of `arthropod` with the
//! continuous FDTD acoustic wave grid of `resonance-audio`. By interacting with the buttons,
//! the user manually injects physical acoustic pressure into the simulation.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
//! - **Parent B (crates/resonance-audio):** Provides the 2D finite-difference time-domain (FDTD)
//!   acoustic wave simulation and AudioModel.
//!
//! ## Novel Trait
//! The discrete interaction of UI clicks (`arthropod`) maps directly to continuous physical acoustic waves
//! (`resonance-audio`). Clicking buttons spawns localized plucks and oscillators in the fluid simulation,
//! rendering the sound pressure visibly on the screen.
//!
//! ## Predicted Phenotype
//! An interactive acoustic laboratory where manual, abstract button clicks manifest as physical,
//! propagating acoustic waves, turning the GUI into a literal musical instrument driving cymatic interference patterns.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p arthropod-resonance --release
//! ```
//!
//! *(Note: Use `cargo run -p arthropod-resonance --release -- --headless` to safely bypass X11 UI panics in CI environments.)*

use arthropod::Button;
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use ::rand::Rng;
use resonance_audio::{AudioCommand, AudioModel};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Resonance".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

// Bypass macroquad::main to support headless execution
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running in headless mode. Bypassing macroquad initialization.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let mut rng = ::rand::thread_rng();

    let grid_width = 200;
    let grid_height = 150;

    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, snap_rx) = bounded(1);

    // Spawn AudioModel in a background thread to process audio logic continuously
    let mut model = AudioModel::new(grid_width, grid_height, cmd_rx, snap_tx, None);
    std::thread::spawn(move || {
        let mut buffer = vec![0.0; 512];
        loop {
            model.process(&mut buffer);
        }
    });

    let btn_pluck = Button::new("Pluck", 20.0, 20.0, 150.0, 40.0)
        .with_colors(BLUE, SKYBLUE, DARKBLUE);
    let btn_oscillate = Button::new("Oscillate", 20.0, 80.0, 150.0, 40.0)
        .with_colors(GREEN, LIME, DARKGREEN);

    let cell_w = 800.0 / grid_width as f32;
    let cell_h = 600.0 / grid_height as f32;

    let mut last_snapshot = None;

    loop {
        clear_background(color_u8!(20, 20, 30, 255));

        if btn_pluck.draw() {
            // Send multiple plucks to different locations
            for _ in 0..3 {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: rng.gen_range(20..grid_width - 20),
                    y: rng.gen_range(20..grid_height - 20),
                    strength: rng.gen_range(2.0..5.0),
                });
            }
        }

        if btn_oscillate.draw() {
            // Drop an oscillator for a short burst
            let x = rng.gen_range(20..grid_width - 20);
            let y = rng.gen_range(20..grid_height - 20);
            let freq = rng.gen_range(200.0..800.0);
            let _ = cmd_tx.send(AudioCommand::Oscillate {
                x,
                y,
                frequency: freq,
                strength: 2.0,
            });

            // Remove it after a slight delay via another thread, or simply add a Tone
            let tx_clone = cmd_tx.clone();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(200));
                let _ = tx_clone.send(AudioCommand::Oscillate {
                    x,
                    y,
                    frequency: freq,
                    strength: 0.0, // Remove it
                });
            });
        }

        // Add some ambient noise plucks
        if rng.gen_bool(0.02) {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: rng.gen_range(10..grid_width - 10),
                y: rng.gen_range(10..grid_height - 10),
                strength: rng.gen_range(0.5..1.5),
            });
        }

        // Try receiving the latest snapshot
        if let Ok(snap) = snap_rx.try_recv() {
            last_snapshot = Some(snap);
        }

        // Render the wave tank
        if let Some(snap) = &last_snapshot {
            for y in 0..grid_height {
                for x in 0..grid_width {
                    let idx = y * grid_width + x;
                    let pressure = snap.pressure[idx];

                    if pressure.abs() > 0.01 {
                        let intensity = (pressure.abs() * 255.0).clamp(0.0, 255.0) as u8;
                        let color = if pressure > 0.0 {
                            color_u8!(intensity, intensity, 255, 255)
                        } else {
                            color_u8!(255, intensity, intensity, 255)
                        };

                        draw_rectangle(
                            x as f32 * cell_w,
                            y as f32 * cell_h,
                            cell_w,
                            cell_h,
                            color,
                        );
                    }
                }
            }
        }

        draw_text("Interactive Acoustic Morphogenesis", 200.0, 30.0, 20.0, WHITE);

        next_frame().await;
    }
}
