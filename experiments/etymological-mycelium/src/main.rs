use macroquad::prelude::*;
use std::env;

mod etymology;
mod fungus;

use etymology::EtymologyRiver;
use fungus::Mycelium;
use strsim::normalized_levenshtein;

const LAYER_WIDTH: f32 = 150.0;
const LINE_HEIGHT: f32 = 12.0;

#[macroquad::main("Etymological Mycelium")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let (repo_path, file_path) = if args.len() > 2 {
        (args[1].as_str(), args[2].as_str())
    } else if args.len() > 1 {
        (".", args[1].as_str())
    } else {
        (".", "Cargo.toml")
    };

    let river_result = EtymologyRiver::new(repo_path, file_path, 50);

    let river = match river_result {
        Ok(r) => Some(r),
        Err(e) => {
            eprintln!("Error loading history: {}", e);
            None
        }
    };

    let mut mycelium = Mycelium::new();
    let mut initialized = false;
    let mut camera_offset = vec2(50.0, screen_height() / 2.0);
    let mut zoom: f32 = 1.0;

    // Mouse interaction
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    // Initial Spawning (can be done progressively, but let's do all at once for the "map")
    // Actually, let's spawn them over time?
    // No, let's spawn all layers but update them.

    if let Some(ref r) = river {
        println!("Loaded {} commits.", r.layers.len());

        for i in 0..r.layers.len().saturating_sub(1) {
            let layer_curr = &r.layers[i];
            let layer_next = &r.layers[i+1];

            let x1 = i as f32 * LAYER_WIDTH;
            let x2 = (i + 1) as f32 * LAYER_WIDTH;

            for (j, line_curr) in layer_curr.lines.iter().enumerate() {
                let y1 = j as f32 * LINE_HEIGHT;

                // Find best match in next layer
                let mut best_k = None;
                let mut best_sim = 0.0;

                // Search window +/- 10 lines
                let start_k = j.saturating_sub(10);
                let end_k = (j + 10).min(layer_next.lines.len());

                for k in start_k..end_k {
                    let line_next = &layer_next.lines[k];
                    let sim = normalized_levenshtein(&line_curr.content, &line_next.content) as f32;

                    if sim > best_sim {
                        best_sim = sim;
                        best_k = Some(k);
                    }
                }

                if let Some(k) = best_k {
                    if best_sim > 0.3 { // Threshold
                        let y2 = k as f32 * LINE_HEIGHT;

                        mycelium.spawn(
                            vec2(x1, y1),
                            vec2(x2, y2),
                            best_sim
                        );
                    } else {
                         // Dead end (deletion)
                         mycelium.spawn(
                            vec2(x1, y1),
                            vec2(x1 + LAYER_WIDTH * 0.5, y1 + 20.0), // Droop down
                            0.0 // Low similarity -> Red/Dead
                        );
                    }
                } else {
                     // No match found (deletion)
                     mycelium.spawn(
                        vec2(x1, y1),
                        vec2(x1 + LAYER_WIDTH * 0.5, y1 + 20.0),
                        0.0
                    );
                }
            }
        }
        initialized = true;
    }

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Dark Gray

        // Input Handling
        if is_mouse_button_pressed(MouseButton::Left) {
            dragging = true;
            last_mouse = mouse_position().into();
        }
        if is_mouse_button_released(MouseButton::Left) {
            dragging = false;
        }
        if dragging {
            let curr = Vec2::from(mouse_position());
            camera_offset += curr - last_mouse;
            last_mouse = curr;
        }

        let (_, wheel) = mouse_wheel();
        if wheel != 0.0 {
            zoom *= if wheel > 0.0 { 1.1 } else { 0.9 };
            zoom = zoom.clamp(0.1, 5.0);
        }

        // Update
        if initialized {
            mycelium.update();
        }

        // Draw
        push_camera_state();
        set_camera(&Camera2D {
            target: vec2(screen_width()/2.0, screen_height()/2.0) - camera_offset,
            zoom: vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0), // Flip Y? No, standard 2D
            ..Default::default()
        });

        // Translate to offset
        // Actually Camera2D target is center.
        // Let's just use transforms.

        // Grid / Timeline
        if let Some(ref r) = river {
            for (i, layer) in r.layers.iter().enumerate() {
                let x = i as f32 * LAYER_WIDTH;
                draw_line(x, -1000.0, x, 1000.0, 1.0, Color::new(0.2, 0.2, 0.2, 0.5));
                draw_text_ex(
                    &layer.oid.to_string()[0..7],
                    x + 5.0,
                    -20.0,
                    TextParams { font_size: 20, color: GRAY, ..Default::default() }
                );
            }
        }

        mycelium.draw();

        pop_camera_state();

        // UI
        draw_text("Etymological Mycelium", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("File: {}", file_path), 10.0, 50.0, 20.0, GRAY);
        draw_text(format!("Spores: {}", mycelium.spores.len()).as_str(), 10.0, 70.0, 20.0, GRAY);

        if river.is_none() {
            draw_text("Failed to load repo/file.", screen_width()/2.0 - 100.0, screen_height()/2.0, 40.0, RED);
        }

        next_frame().await
    }
}
