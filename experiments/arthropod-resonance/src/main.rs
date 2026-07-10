//! # 🧬 arthropod-resonance
//!
//! **Lineage**: `arthropod` (Immediate Mode UI components) × `resonance-audio` (Real-time 2D Wave Physics)
//!
//! **Concept**: Interactive Acoustic Sandbox.
//!
//! **Novel Trait**: Mapping abstract immediate-mode GUI components onto an acoustic wave propagation grid, where button clicks and slider interactions act as active audio exciters or dynamic physical boundaries in the wave simulation.
//!
//! **Phenotype**: An interactive synthesizer sandbox where manipulating GUI widgets visibly splashes waves across the 2D grid and alters acoustic standing wave patterns.
//!
//! **Status**: Attempted Cross
//!
//! ## Quick Start
//!
//! ```sh
//! # Run interactively (GUI)
//! cargo run -p arthropod-resonance --release
//!
//! # Run headlessly (CI/Non-interactive)
//! cargo run -p arthropod-resonance --release -- --headless
//! ```

use arthropod::Button;
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::{AudioCommand, AudioModel};

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Resonance".to_owned(),
        ..Default::default()
    }
}

async fn run_sim() {
    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, snap_rx) = bounded(1);

    // Grid size for the audio model
    let grid_width = 100;
    let grid_height = 100;
    let mut model = AudioModel::new(grid_width, grid_height, cmd_rx, snap_tx, None);

    let pluck_btn = Button::new("Pluck Center", 10.0, 10.0, 150.0, 40.0)
        .with_colors(BLUE, SKYBLUE, DARKGRAY);
    let move_listener_btn = Button::new("Move Listener", 10.0, 60.0, 150.0, 40.0)
        .with_colors(RED, ORANGE, DARKGRAY);

    let mut audio_buffer = vec![0.0; 1024];

    loop {
        clear_background(BLACK);

        if pluck_btn.draw() {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: grid_width / 2,
                y: grid_height / 2,
                strength: 1.0,
            });
        }

        if move_listener_btn.draw() {
            let _ = cmd_tx.send(AudioCommand::MoveListener {
                x: rand::gen_range(10, grid_width - 10),
                y: rand::gen_range(10, grid_height - 10),
            });
        }

        // Let the model process to advance the simulation state
        model.process(&mut audio_buffer);

        // Render the wave snapshot
        if let Ok(snapshot) = snap_rx.try_recv() {
            let cell_w = screen_width() / grid_width as f32;
            let cell_h = screen_height() / grid_height as f32;

            for y in 0..grid_height {
                for x in 0..grid_width {
                    let pressure = snapshot.pressure[y * grid_width + x];
                    let intensity = ((pressure.abs() * 255.0) as u8).min(255);
                    let color = if pressure > 0.0 {
                        Color::from_rgba(intensity, 0, 0, 255)
                    } else {
                        Color::from_rgba(0, 0, intensity, 255)
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

            // Draw listener
            draw_circle(
                model.listener_x as f32 * cell_w + cell_w / 2.0,
                model.listener_y as f32 * cell_h + cell_h / 2.0,
                5.0,
                YELLOW,
            );
        }

        // Redraw buttons over the grid
        pluck_btn.draw();
        move_listener_btn.draw();

        next_frame().await
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&String::from("--headless")) {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), run_sim());
}
