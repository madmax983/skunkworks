mod outline;
mod wave;

use macroquad::prelude::*;
use outline::GlyphOutline;
use rusttype::{point, Font, Scale};
use wave::{tessellate_and_distort, WaveParams};

#[macroquad::main("Glyph Wave")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // Initial Params
    let mut params = WaveParams {
        time: 0.0,
        frequency: 2.0,
        amplitude: 2.0,
        phase_speed: 4.0,
    };

    let text = "GLYPH WAVE";

    loop {
        clear_background(BLACK);

        // Update Params
        params.time = get_time() as f32;

        if is_key_down(KeyCode::Space) {
            params.amplitude = lerp(params.amplitude, 15.0, 0.1);
            params.frequency = lerp(params.frequency, 8.0, 0.1);
            params.phase_speed = lerp(params.phase_speed, 10.0, 0.1);
        } else {
            params.amplitude = lerp(params.amplitude, 3.0, 0.1);
            params.frequency = lerp(params.frequency, 3.0, 0.1);
            params.phase_speed = lerp(params.phase_speed, 3.0, 0.1);
        }

        // Layout Text
        // Center text roughly
        let font_size = 120.0;
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        // Simple centering logic
        let width_est = text.len() as f32 * font_size * 0.6;
        let start_x = (screen_width() - width_est) / 2.0;
        let start_y = (screen_height() + v_metrics.ascent) / 2.0;

        let offset = point(start_x, start_y);

        let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

        for glyph in glyphs {
            let mut outline = GlyphOutline::new();
            glyph.build_outline(&mut outline);

            // Distort
            let distorted_strips = tessellate_and_distort(&outline, &params);

            // Render
            for strip in distorted_strips {
                if strip.len() < 2 {
                    continue;
                }
                for i in 0..strip.len() - 1 {
                    // Gradient Color
                    // Base color on time + x position
                    let hue = (params.time + strip[i].x * 0.005).sin() * 0.5 + 0.5;
                    // Neon Cyan/Magenta/Green palette
                    let color = hsl_to_rgb(hue, 1.0, 0.5);

                    draw_line(
                        strip[i].x,
                        strip[i].y,
                        strip[i + 1].x,
                        strip[i + 1].y,
                        2.0,
                        color,
                    );
                }
            }
        }

        draw_text(
            "Hold SPACE for STORM",
            20.0,
            screen_height() - 20.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

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
