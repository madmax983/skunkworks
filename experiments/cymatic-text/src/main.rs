mod audio;
mod deformation;
mod outline;

use audio::AudioReactor;
use deformation::deform;
use macroquad::prelude::*;
use outline::{load_glyph, BezierSegment, GlyphCurve};
use rusttype::Font;

#[macroquad::main("Cymatic Text")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let mut reactor = AudioReactor::new();
    let text = "CYMATIC";
    let font_size = 150.0;

    let mut base_glyphs = Vec::new();
    let mut total_width = 0.0;

    for c in text.chars() {
        let curve = load_glyph(&font, c, font_size);
        base_glyphs.push(curve);
    }

    // Calculate total width based on base glyphs
    for g in &base_glyphs {
        total_width += g.advance_width;
    }

    loop {
        reactor.update(get_frame_time());

        if is_key_pressed(KeyCode::Space) {
            reactor.trigger_beat();
        }

        clear_background(BLACK);

        // Center the text
        // Recalculate width based on deformation factor (low freq)
        let scale_factor = 1.0 + reactor.spectrum.low * 0.1;
        let current_total_width = total_width * scale_factor;

        let start_x = (screen_width() - current_total_width) / 2.0;
        let start_y = screen_height() / 2.0 + font_size * 0.3;

        let mut x_offset = start_x;

        let color = Color::new(
            0.5 + reactor.spectrum.low * 0.5,
            0.2 + reactor.spectrum.mid * 0.8,
            0.4 + reactor.spectrum.high * 0.6,
            1.0,
        );

        for base_curve in &base_glyphs {
            let mut positioned_curve = base_curve.clone();

            // Apply scale to advance width
            positioned_curve.advance_width *= scale_factor;

            let offset = Vec2::new(x_offset, start_y);

            // Translate
            for seg in &mut positioned_curve.segments {
                match seg {
                    BezierSegment::Line { start, end } => {
                        *start += offset;
                        *end += offset;
                    }
                    BezierSegment::Quad { start, ctrl, end } => {
                        *start += offset;
                        *ctrl += offset;
                        *end += offset;
                    }
                    BezierSegment::Cubic {
                        start,
                        ctrl1,
                        ctrl2,
                        end,
                    } => {
                        *start += offset;
                        *ctrl1 += offset;
                        *ctrl2 += offset;
                        *end += offset;
                    }
                }
            }
            positioned_curve.min += offset;
            positioned_curve.max += offset;

            // Deform
            let deformed = deform(&positioned_curve, &reactor.spectrum, reactor.time);

            // Draw Bloom (Thick, transparent)
            let mut bloom_color = color;
            bloom_color.a = 0.2 + reactor.spectrum.high * 0.2;
            draw_curve(&deformed, bloom_color, 8.0 + reactor.spectrum.low * 10.0);

            // Draw Core (Thin, bright)
            let mut core_color = color;
            core_color.r += 0.2;
            core_color.g += 0.2;
            core_color.b += 0.2;
            core_color.a = 1.0;
            draw_curve(&deformed, core_color, 2.0);

            x_offset += positioned_curve.advance_width;
        }

        // HUD
        draw_text("SPACE: Drop Bass", 20.0, 30.0, 20.0, DARKGRAY);

        // Spectrum Bars
        let h = screen_height();
        let w = screen_width();
        draw_rectangle(
            w - 100.0,
            h - 50.0 - reactor.spectrum.low * 200.0,
            20.0,
            reactor.spectrum.low * 200.0,
            RED,
        );
        draw_rectangle(
            w - 70.0,
            h - 50.0 - reactor.spectrum.mid * 200.0,
            20.0,
            reactor.spectrum.mid * 200.0,
            GREEN,
        );
        draw_rectangle(
            w - 40.0,
            h - 50.0 - reactor.spectrum.high * 200.0,
            20.0,
            reactor.spectrum.high * 200.0,
            BLUE,
        );

        next_frame().await
    }
}

fn draw_curve(curve: &GlyphCurve, color: Color, thickness: f32) {
    for seg in &curve.segments {
        match seg {
            BezierSegment::Line { start, end } => {
                draw_line(start.x, start.y, end.x, end.y, thickness, color);
            }
            BezierSegment::Quad { start, ctrl, end } => {
                draw_quad_bezier(*start, *ctrl, *end, color, thickness);
            }
            BezierSegment::Cubic {
                start,
                ctrl1,
                ctrl2,
                end,
            } => {
                draw_cubic_bezier(*start, *ctrl1, *ctrl2, *end, color, thickness);
            }
        }
    }
}

fn draw_quad_bezier(p0: Vec2, p1: Vec2, p2: Vec2, color: Color, thickness: f32) {
    let steps = 10;
    let mut prev = p0;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let inv_t = 1.0 - t;
        let next = inv_t * inv_t * p0 + 2.0 * inv_t * t * p1 + t * t * p2;
        draw_line(prev.x, prev.y, next.x, next.y, thickness, color);
        prev = next;
    }
}

fn draw_cubic_bezier(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, color: Color, thickness: f32) {
    let steps = 15;
    let mut prev = p0;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let inv_t = 1.0 - t;
        let next = inv_t * inv_t * inv_t * p0
            + 3.0 * inv_t * inv_t * t * p1
            + 3.0 * inv_t * t * t * p2
            + t * t * t * p3;
        draw_line(prev.x, prev.y, next.x, next.y, thickness, color);
        prev = next;
    }
}
