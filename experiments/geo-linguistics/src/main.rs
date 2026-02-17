use crate::erosion::{hydraulic_erosion, thermal_weathering};
use crate::phonology::{char_to_terrain, terrain_to_char, TerrainPoint};
use macroquad::prelude::*;

mod erosion;
mod phonology;

const TERRAIN_SCALE_X: f32 = 20.0;
const TERRAIN_SCALE_Y: f32 = 150.0;
const BASELINE_Y: f32 = 400.0;

#[macroquad::main("Geo-Linguistics")]
async fn main() {
    let mut input_text = String::from("pater");
    let mut terrain: Vec<TerrainPoint> = text_to_terrain(&input_text);
    let mut strata: Vec<Vec<TerrainPoint>> = Vec::new();

    let mut auto_erode = false;
    let mut last_reconstruction = input_text.clone();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.15, 1.0)); // Dark Blue-Grey

        // --- Input Handling ---
        let char_pressed = get_char_pressed();
        if let Some(c) = char_pressed {
            if c.is_alphabetic() || c == ' ' {
                input_text.push(c);
                terrain = text_to_terrain(&input_text); // Reset terrain on edit
                strata.clear();
            }
        }

        if is_key_pressed(KeyCode::Back) && !input_text.is_empty() {
            input_text.pop();
            terrain = text_to_terrain(&input_text);
            strata.clear();
        }

        if is_key_pressed(KeyCode::Space) {
            auto_erode = !auto_erode;
        }

        if is_key_pressed(KeyCode::S) {
            // Fossilize current layer
            if strata.len() > 5 {
                strata.remove(0); // Keep history limited
            }
            strata.push(terrain.clone());
        }

        if is_key_pressed(KeyCode::R) {
            terrain = text_to_terrain(&input_text);
            strata.clear();
        }

        // --- Simulation ---
        if auto_erode || is_key_down(KeyCode::E) {
            hydraulic_erosion(&mut terrain);
            thermal_weathering(&mut terrain);
        }

        // Reconstruction
        // We only reconstruct every few frames to avoid flickering text
        if get_frame_time() > 0.0 {
            // Reconstruct logic
            last_reconstruction = reconstruct_text(&terrain);
        }

        // --- Drawing ---

        // Draw Strata (History)
        for (idx, layer) in strata.iter().enumerate() {
            let offset_y = BASELINE_Y + (strata.len() - idx) as f32 * 50.0 + 50.0;
            let color = Color::new(0.4, 0.4, 0.4, 0.5 - (idx as f32 * 0.1));
            draw_terrain(layer, offset_y, color);
        }

        // Draw Current Terrain
        draw_terrain(&terrain, BASELINE_Y, GREEN);

        // Draw Text Labels on Peaks
        for (i, point) in terrain.iter().enumerate() {
            if point.height > 0.2 {
                // Show char if significant
                let c = terrain_to_char(point);
                if c != ' ' {
                    let x = i as f32 * TERRAIN_SCALE_X + 50.0;
                    let y = BASELINE_Y - point.height * TERRAIN_SCALE_Y - 10.0;
                    draw_text(&c.to_string(), x, y, 20.0, YELLOW);
                }
            }
        }

        // UI
        draw_text(
            "Geo-Linguistics: The Geology of Language",
            20.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Input: {}", input_text),
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Current: {}", last_reconstruction),
            20.0,
            90.0,
            20.0,
            GOLD,
        );

        draw_text("Controls:", 20.0, screen_height() - 100.0, 20.0, GRAY);
        draw_text("Type to set word | Space: Toggle Erosion | E: Hold to Erode | S: Save Stratum | R: Reset", 20.0, screen_height() - 70.0, 20.0, GRAY);

        next_frame().await
    }
}

fn text_to_terrain(text: &str) -> Vec<TerrainPoint> {
    let mut terrain = Vec::new();

    // Padding
    for _ in 0..5 {
        terrain.push(TerrainPoint {
            height: 0.0,
            hardness: 0.0,
        });
    }

    for c in text.chars() {
        let center = char_to_terrain(c);

        // Interpolate / Spread the phoneme
        // A phoneme is not a single point, it's a region
        // Peaks are sharp, Vowels are wide valleys

        // Left shoulder
        terrain.push(TerrainPoint {
            height: center.height * 0.5,
            hardness: center.hardness * 0.8,
        });

        // Center
        terrain.push(center);

        // Right shoulder
        terrain.push(TerrainPoint {
            height: center.height * 0.5,
            hardness: center.hardness * 0.8,
        });

        // Gap
        terrain.push(TerrainPoint {
            height: 0.1,
            hardness: 0.1,
        });
    }

    // Padding
    for _ in 0..5 {
        terrain.push(TerrainPoint {
            height: 0.0,
            hardness: 0.0,
        });
    }

    terrain
}

fn reconstruct_text(terrain: &Vec<TerrainPoint>) -> String {
    let mut s = String::new();

    // Simple sampling: Check every 4th point (since we expand each char to 4 points)
    // Or better: Find local maxima

    let mut i = 5;
    while i < terrain.len().saturating_sub(5) {
        // We pushed 4 points per char in `text_to_terrain`
        // Left, Center, Right, Gap.
        // So checking index i+1 (Center) is a good approximation if we stride by 4

        let point = &terrain[i + 1];
        let c = terrain_to_char(point);
        if c != ' ' {
            s.push(c);
        }

        i += 4;
    }

    s
}

fn draw_terrain(terrain: &Vec<TerrainPoint>, base_y: f32, color: Color) {
    for i in 0..terrain.len().saturating_sub(1) {
        let x1 = i as f32 * TERRAIN_SCALE_X + 50.0;
        let y1 = base_y - terrain[i].height * TERRAIN_SCALE_Y;

        let x2 = (i + 1) as f32 * TERRAIN_SCALE_X + 50.0;
        let y2 = base_y - terrain[i + 1].height * TERRAIN_SCALE_Y;

        draw_line(x1, y1, x2, y2, 2.0, color);

        // Fill below?
        draw_line(
            x1,
            y1,
            x1,
            base_y,
            1.0,
            Color::new(color.r, color.g, color.b, 0.1),
        );
    }
}
