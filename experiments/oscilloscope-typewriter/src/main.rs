mod outline;
mod wave;

use macroquad::prelude::*;
use rusttype::{Font, Scale};
use outline::extract_outline;
use wave::{Voice, WaveType};

#[macroquad::main("Oscilloscope Typewriter")]
async fn main() {
    let font_bytes = match load_file("assets/font.ttf").await {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to load font: {}", e);
            // Fallback to loading from relative path if execution context differs
             match std::fs::read("experiments/oscilloscope-typewriter/assets/font.ttf") {
                Ok(bytes) => bytes,
                Err(_) => panic!("Failed to load font from assets/font.ttf or relative path"),
             }
        }
    };

    let font = Font::try_from_bytes(&font_bytes).expect("Error constructing Font");

    let mut text = String::from("GENESIS");
    let mut voices = Vec::new();

    // Initial voices
    for c in text.chars() {
        voices.push(create_voice(c));
    }

    let scale = Scale::uniform(100.0);
    let mut time = 0.0;

    loop {
        clear_background(BLACK);

        // Input
        while let Some(c) = get_char_pressed() {
            if c.is_control() || c == '\n' || c == '\r' || c == '\u{8}' {
                continue;
            }
            text.push(c);
            voices.push(create_voice(c));
        }

        if is_key_pressed(KeyCode::Backspace) {
            text.pop();
            voices.pop();
        }

        // Update
        time += get_frame_time();

        // Render
        let mut cursor_x = 50.0;
        let base_y = screen_height() / 2.0;

        for (i, c) in text.chars().enumerate() {
            let advance = font.glyph(c).scaled(scale).h_metrics().advance_width;

            if let Some(glyph) = extract_outline(&font, c, scale) {
                // Draw glyph
                let voice = &voices[i];

                // Draw multiple passes for glow/beam effect
                // Pass 0: Core (bright, thin)
                // Pass 1: Glow (dimmer, thick)
                for pass in 0..2 {
                    let (width, alpha) = if pass == 0 {
                        (2.0, 1.0)
                    } else {
                        (6.0, 0.3)
                    };

                    let color = Color::new(0.2, 1.0, 0.2, alpha); // Green phosphor

                    for j in 0..glyph.points.len() {
                        let p1_orig = glyph.points[j];
                        let n1 = glyph.normals[j];

                        // Wrap around for closed loop visualization
                        // Note: fonts have multiple contours. My outline builder flattens them into one list.
                        // Ideally I should handle contours separately to avoid connecting separate parts.
                        // But for "oscilloscope", the beam has to travel between parts anyway, often leaving a trace.
                        // I'll filter long jumps to avoid ugly lines across the glyph.

                        let next_idx = (j + 1) % glyph.points.len();
                        let p2_orig = glyph.points[next_idx];
                        let n2 = glyph.normals[next_idx];

                        // Check for large jump (new contour)
                        if p1_orig.distance(p2_orig) > 50.0 {
                            continue;
                        }

                        // Calculate displacement
                        // Frequency modulation based on character index or voice
                        let d1 = voice.synthesize(time, j as f32 * 0.1) * 5.0;
                        let d2 = voice.synthesize(time, next_idx as f32 * 0.1) * 5.0;

                        // Apply to position
                        // We invert Y because font coordinates are Y-up (usually) but screen is Y-down?
                        // rusttype: +Y is down?
                        // "The coordinate system ... has +y pointing down".
                        // So we are good.
                        // However, we need to center it on base_y.
                        // p1_orig is relative to (0,0) baseline.
                        // So p1_orig.y can be negative (ascender) or positive (descender).

                        let p1 = p1_orig + n1 * d1 + vec2(cursor_x, base_y);
                        let p2 = p2_orig + n2 * d2 + vec2(cursor_x, base_y);

                        draw_line(p1.x, p1.y, p2.x, p2.y, width, color);
                    }
                }
            }
            cursor_x += advance;
        }

        // Draw cursor
        let cursor_wave = (time * 10.0).sin() * 10.0;
        draw_line(cursor_x, base_y - 50.0 + cursor_wave, cursor_x, base_y + 50.0 - cursor_wave, 2.0, GREEN);

        draw_text("Type to add waves. Backspace to remove.", 20.0, 30.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn create_voice(c: char) -> Voice {
    let freq = 2.0 + (c as u32 % 10) as f32 * 1.0;
    let wave_type = match (c as u32) % 4 {
        0 => WaveType::Sine,
        1 => WaveType::Square,
        2 => WaveType::Saw,
        _ => WaveType::Triangle,
    };
    Voice::new(freq, 1.0, wave_type)
}
