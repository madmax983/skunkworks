use macroquad::prelude::*;
use rusttype::{Font, Scale, point};

mod outline;
mod particles;
mod audio;

use outline::{GlyphOutline, tessellate};
use particles::ParticleSystem;
use audio::AudioSim;

#[macroquad::main("Sonic Glyphs")]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

    let text = "SONIC GLYPHS";
    let font_size = 120.0;
    let scale = Scale::uniform(font_size);
    let v_metrics = font.v_metrics(scale);

    // Center text
    let width_est = text.len() as f32 * font_size * 0.6; // Rough estimate
    let start_x = (screen_width() - width_est) / 2.0;
    let start_y = (screen_height() + v_metrics.ascent) / 2.0;
    let offset = point(start_x, start_y);

    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    let mut particle_sys = ParticleSystem::new();

    for glyph in glyphs {
        let pos = glyph.position();
        let glyph = glyph.into_unpositioned();
        let mut outline = GlyphOutline::new();
        glyph.build_outline(&mut outline);

        // Use higher steps for smoother curves
        let contours = tessellate(&outline, 8);

        for contour in contours {
            // Offset points by glyph position
            let offset_contour: Vec<Vec2> = contour.iter().map(|p| {
                *p + vec2(pos.x, pos.y)
            }).collect();

            particle_sys.add_contour(&offset_contour);
        }
    }

    let mut audio = AudioSim::new();
    let mut show_springs = true;

    loop {
        let dt = get_frame_time();
        audio.update(dt as f64);

        let mouse_pos = vec2(mouse_position().0, mouse_position().1);

        // Toggle view
        if is_key_pressed(KeyCode::Space) {
            show_springs = !show_springs;
        }

        // Update particles
        // Use bass for "shockwave" energy
        let energy = if audio.is_beat { audio.bass } else { 0.0 };
        particle_sys.update(dt, energy, mouse_pos);

        clear_background(BLACK);

        // Visualize Audio Background
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);
        draw_circle(center.x, center.y, audio.bass * 300.0, Color::new(0.2, 0.0, 0.0, 0.5));
        draw_circle(center.x, center.y, audio.mid * 200.0, Color::new(0.0, 0.2, 0.0, 0.5));

        // Render Particles
        // Draw springs
        if show_springs {
            for &(i, j) in &particle_sys.springs {
                let p1 = particle_sys.particles[i].pos;
                let p2 = particle_sys.particles[j].pos;

                // Color based on audio
                let hue = (particle_sys.particles[i].target.x * 0.005 + audio.time as f32).sin() * 0.5 + 0.5;
                let color = hsl_to_rgb(hue, 1.0, 0.5);

                draw_line(p1.x, p1.y, p2.x, p2.y, 1.5, color);
            }
        } else {
            // Draw Points
            for p in &particle_sys.particles {
                let hue = (p.target.x * 0.005 + audio.time as f32).sin() * 0.5 + 0.5;
                let color = hsl_to_rgb(hue, 1.0, 0.5);
                draw_circle(p.pos.x, p.pos.y, 2.0, color);
            }
        }

        // UI
        draw_text(
            &format!("BPM: {:.0} | Bass: {:.2} | Space: Toggle View", audio.bpm, audio.bass),
            20.0,
            30.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
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
