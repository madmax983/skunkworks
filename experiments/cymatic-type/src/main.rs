mod distortion;
mod mesh;
mod outline;
mod synth;

use macroquad::prelude::*;
use rusttype::{Font, Scale, point};
use outline::GlyphOutline;
use mesh::{tessellate};
use synth::AudioEngine;
use distortion::apply_distortion;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cymatic Type".to_string(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let mut engine = AudioEngine::new();
    let mut text = String::from("CYMATICS");
    let mut meshes = Vec::new();
    let mut needs_layout = true;

    // View params
    let font_size = 150.0;

    loop {
        clear_background(BLACK);
        let dt = get_frame_time();

        // Update Audio
        engine.update(dt);
        let spectrum = engine.get_spectrum();

        // Input
        let mut char_pressed = false;
        while let Some(c) = get_char_pressed() {
            if c.is_ascii_graphic() || c == ' ' {
                text.push(c);
                char_pressed = true;
            }
        }
        if is_key_pressed(KeyCode::Backspace) && !text.is_empty() {
            text.pop();
            char_pressed = true;
        }

        // Layout & Tessellate (only when text changes)
        if char_pressed || needs_layout {
            meshes.clear();
            let scale = Scale::uniform(font_size);
            let start = point(0.0, 0.0);

            // Layout glyphs
            for glyph in font.layout(&text, scale, start) {
                let mut outline = GlyphOutline::new();
                if let Some(_bb) = glyph.pixel_bounding_box() {
                    glyph.build_outline(&mut outline);
                }
                // Tessellate regardless (might be empty)
                let mesh = tessellate(&outline);
                meshes.push(mesh);
            }
            needs_layout = false;
        }

        // Calculate bounds for centering
        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut has_points = false;

        for mesh in &meshes {
            for strip in &mesh.strips {
                for v in strip {
                    has_points = true;
                    if v.position.x < min_x { min_x = v.position.x; }
                    if v.position.x > max_x { max_x = v.position.x; }
                    if v.position.y < min_y { min_y = v.position.y; }
                    if v.position.y > max_y { max_y = v.position.y; }
                }
            }
        }

        let center_offset;
        if has_points {
            let width = max_x - min_x;
            let height = max_y - min_y;
            center_offset = Vec2::new(
                (screen_width() - width) / 2.0 - min_x,
                (screen_height() - height) / 2.0 - min_y
            );
        } else {
            center_offset = Vec2::new(screen_width()/2.0, screen_height()/2.0);
        }

        // Render Distorted Meshes
        let time = engine.time;
        for base_mesh in &meshes {
            // Apply distortion based on spectrum
            let distorted = apply_distortion(base_mesh, &spectrum, time);

            for strip in distorted.strips {
                if strip.len() < 2 { continue; }

                for i in 0..strip.len()-1 {
                    let p1 = strip[i].position + center_offset;
                    let p2 = strip[i+1].position + center_offset;

                    // Dynamic color
                    // Hue cycles with time and position
                    // Brightness pulses with bass
                    let hue = (time * 0.2 + strip[i].uv).fract();
                    let lightness = 0.5 + spectrum.bass * 0.4;
                    let color = hsl_to_rgb(hue, 1.0, lightness);

                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, color);

                    // Bloom (Ghost lines)
                    // If bass is high, add horizontal glitch/offset
                    if spectrum.bass > 0.1 {
                        let offset_val = 2.0 + spectrum.bass * 10.0;
                        let alpha = 0.3 * spectrum.bass;
                        // Manual alpha blend assuming black background
                        let bloom_color = Color::new(color.r, color.g, color.b, alpha);

                        draw_line(p1.x + offset_val, p1.y, p2.x + offset_val, p2.y, 2.0, bloom_color);
                        draw_line(p1.x - offset_val, p1.y, p2.x - offset_val, p2.y, 2.0, bloom_color);
                    }

                    // Vertical jitter bloom for treble
                    if spectrum.treble > 0.5 {
                         let offset_val = spectrum.treble * 5.0;
                         let alpha = 0.2 * spectrum.treble;
                         let bloom_color = Color::new(color.r, color.g, color.b, alpha);
                         draw_line(p1.x, p1.y - offset_val, p2.x, p2.y - offset_val, 1.0, bloom_color);
                    }
                }
            }
        }

        // UI Overlay
        draw_text("TYPE TO SPEAK. MUSIC IS PROCEDURAL.", 20.0, 30.0, 20.0, WHITE);

        // Spectrum Bars
        let bar_w = 20.0;
        let bar_h = 100.0;
        let base_y = screen_height() - 20.0;

        draw_rectangle(20.0, base_y - spectrum.bass * bar_h, bar_w, spectrum.bass * bar_h, RED);
        draw_rectangle(50.0, base_y - spectrum.mid * bar_h, bar_w, spectrum.mid * bar_h, GREEN);
        draw_rectangle(80.0, base_y - spectrum.treble * bar_h, bar_w, spectrum.treble * bar_h, BLUE);

        next_frame().await
    }
}

// Helper
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}
