//! # Arthropod Platter 🌡️
//!
//! **Concept:** Interactive Thermodynamic Field.
//!
//! This hybrid visualizer crosses the immediate-mode UI library of `arthropod` with the continuous scalar heat simulation of `platter`. The user can manually inject thermal scalar values (heat or cold pulses) into the physical grid using discrete GUI buttons.
//!
//! ## Lineage
//! - **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
//! - **Parent B (crates/platter):** Provides the continuous 2D scalar field array that exponentially decays over time, mimicking thermodynamic dissipation.
//!
//! ## Novel Trait
//! The discrete interaction of UI clicks (`arthropod`) maps directly into the continuous thermodynamic scalar field (`platter`). Clicking buttons drops instantaneous scalar thermal mass into the simulation space, proving UI can drive physical field dissipation.
//!
//! ## Predicted Phenotype
//! An interactive laboratory where manual, abstract button clicks manifest as glowing thermal energy dissipation, allowing the user to observe field decay dynamically as a consequence of their direct input.
//!
//! ## Usage
//!
//! ```sh
//! cargo run -p arthropod-platter --release
//! ```
//!
//! *(Note: Use `cargo run -p arthropod-platter --release -- --headless` to safely bypass X11 UI panics in CI environments.)*

use ::rand::Rng;
use arthropod::Button;
use macroquad::prelude::*;
use platter::Platter;

fn window_conf() -> Conf {
    Conf {
        window_title: "Arthropod Platter".to_owned(),
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

    let grid_width = 100;
    let grid_height = 75; // 800/600 aspect ratioish
    let mut field = Platter::new(grid_width, grid_height);

    let btn_heat =
        Button::new("Spawn Heat (+)", 20.0, 20.0, 180.0, 40.0).with_colors(MAROON, RED, ORANGE);
    let btn_cold =
        Button::new("Spawn Cold (-)", 20.0, 80.0, 180.0, 40.0).with_colors(DARKBLUE, BLUE, SKYBLUE);
    let btn_clear =
        Button::new("Clear Field", 20.0, 140.0, 180.0, 40.0).with_colors(DARKGRAY, GRAY, LIGHTGRAY);

    let cell_width = 800.0 / grid_width as f32;
    let cell_height = 600.0 / grid_height as f32;

    loop {
        clear_background(BLACK);

        if btn_heat.draw() {
            // Drop a large thermal mass in the center
            let cx = grid_width / 2;
            let cy = grid_height / 2;
            for y in cy.saturating_sub(10)..=(cy + 10).min(grid_height - 1) {
                for x in cx.saturating_sub(10)..=(cx + 10).min(grid_width - 1) {
                    field.accumulate(x, y, rng.gen_range(5.0..25.0));
                }
            }
        }

        if btn_cold.draw() {
            // Drop a cold spot
            let cx = rng.gen_range(10..grid_width - 10);
            let cy = rng.gen_range(10..grid_height - 10);
            for y in cy.saturating_sub(5)..=(cy + 5).min(grid_height - 1) {
                for x in cx.saturating_sub(5)..=(cx + 5).min(grid_width - 1) {
                    field.accumulate(x, y, -rng.gen_range(5.0..25.0));
                }
            }
        }

        if btn_clear.draw() {
            field.clear();
        }

        // Add some ambient noise
        if rng.gen_bool(0.1) {
            field.accumulate(
                rng.gen_range(0..grid_width),
                rng.gen_range(0..grid_height),
                rng.gen_range(-1.0..2.0),
            );
        }

        // Apply thermodynamic decay
        field.decay(0.97);

        // Draw the field
        for y in 0..grid_height {
            for x in 0..grid_width {
                let val = field.get(x, y);
                if val.abs() < 0.05 {
                    continue;
                }

                let color = if val > 0.0 {
                    // Heat: Red -> Yellow -> White
                    let intensity = (val / 10.0).clamp(0.0, 1.0) as f32;
                    Color::new(intensity, intensity * 0.5, intensity * 0.1, 1.0)
                } else {
                    // Cold: Blue -> Cyan -> White
                    let intensity = (val.abs() / 10.0).clamp(0.0, 1.0) as f32;
                    Color::new(intensity * 0.1, intensity * 0.5, intensity, 1.0)
                };

                draw_rectangle(
                    x as f32 * cell_width,
                    y as f32 * cell_height,
                    cell_width,
                    cell_height,
                    color,
                );
            }
        }

        // Draw buttons again on top
        btn_heat.draw();
        btn_cold.draw();
        btn_clear.draw();

        next_frame().await;
    }
}
