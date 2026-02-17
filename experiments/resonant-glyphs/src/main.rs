mod distortion;
mod font;

use distortion::{distort, flatten};
use font::{load_glyph, GlyphOutline};
use macroquad::prelude::*;
use macroquad::color::hsl_to_rgb;
use ttf_parser::Face;

#[macroquad::main("Resonant Glyphs")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let face = Face::parse(font_data, 0).expect("Failed to parse font");

    let text = "RESONANCE";
    let mut glyphs: Vec<(GlyphOutline, f32)> = Vec::new();
    let mut current_x = 0.0;

    // Scale and spacing
    let scale = 0.15;
    let spacing = 50.0; // Font units spacing, roughly

    for c in text.chars() {
        if let Some(outline) = load_glyph(&face, c) {
            glyphs.push((outline.clone(), current_x));
            current_x += outline.advance_width + spacing;
        }
    }

    // Center logic
    let total_width = current_x * scale;

    let mut time = 0.0;
    let mut freq: f32 = 0.005;
    let mut amp: f32 = 200.0;

    loop {
        // Trails: draw semi-transparent black rect
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.1),
        );

        if is_key_down(KeyCode::Up) {
            freq += 0.0001;
        }
        if is_key_down(KeyCode::Down) {
            freq -= 0.0001;
        }
        if is_key_down(KeyCode::Right) {
            amp += 5.0;
        }
        if is_key_down(KeyCode::Left) {
            amp -= 5.0;
        }

        let start_x = (screen_width() - total_width) / 2.0;
        let baseline_y = screen_height() / 2.0 + 100.0; // Shift down slightly

        time += get_frame_time() as f64 * 5.0;

        for (base_glyph, x_offset) in &glyphs {
            // Apply distortion
            // x_offset is used to offset the *time* phase for wave propagation
            let distorted = distort(base_glyph, time + (*x_offset as f64 * 0.01), freq, amp);

            // Flatten to points
            let contours = flatten(&distorted, 5);

            for contour_points in contours {
                if contour_points.len() < 2 {
                    continue;
                }

                for i in 0..contour_points.len() - 1 {
                    let p1 = contour_points[i];
                    let p2 = contour_points[i+1];

                    let sx1 = start_x + (x_offset * scale) + (p1.x * scale);
                    let sy1 = baseline_y - (p1.y * scale);
                    let sx2 = start_x + (x_offset * scale) + (p2.x * scale);
                    let sy2 = baseline_y - (p2.y * scale);

                    // Dynamic Color
                    let hue = ((sx1 / screen_width()) + (time as f32 * 0.05)) % 1.0;
                    let color = hsl_to_rgb(hue, 0.8, 0.6);

                    draw_line(sx1, sy1, sx2, sy2, 2.0, color);
                }
            }
        }

        draw_text(
            &format!("Freq: {:.5} | Amp: {:.1}", freq, amp),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Use Arrow Keys to modify wave parameters",
            10.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
