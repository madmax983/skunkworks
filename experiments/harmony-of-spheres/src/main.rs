mod audio;
mod physics;

use crate::audio::AudioEngine;
use crate::physics::{check_crossings, update, Body, G};
use macroquad::prelude::*;

const STAR_MASS: f32 = 50000.0;

#[macroquad::main("Harmony of Spheres")]
async fn main() {
    let audio = AudioEngine::new().await;
    let mut bodies = Vec::new();

    // Create Star
    bodies.push(Body::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 0.0),
        STAR_MASS,
        20.0,
        YELLOW,
    ));

    // String visual state
    let mut string_flash = 0.0;

    // Harmony Mode
    let mut harmony_mode = false;
    let base_radius = 200.0;
    let ratios = [
        (1.0, 1.0),
        (4.0, 3.0),
        (3.0, 2.0),
        (2.0, 1.0),
        (3.0, 1.0),
        (4.0, 1.0),
    ];

    // Previous positions for crossing detection
    let mut old_positions: Vec<Vec2> = bodies.iter().map(|b| b.pos).collect();

    loop {
        let dt = get_frame_time().min(0.05);

        // Input
        if is_key_pressed(KeyCode::H) {
            harmony_mode = !harmony_mode;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let mut pos = Vec2::new(
                mpos.0 - screen_width() / 2.0,
                mpos.1 - screen_height() / 2.0,
            ); // Center at screen center

            let mut dist = pos.length();

            if harmony_mode {
                // Snap to closest resonant ring
                let mut best_r = dist;
                let mut min_diff = f32::MAX;

                for (n, d) in ratios {
                    let ratio: f32 = n / d;
                    // Period ratio T/T_base = ratio
                    // r/r_base = ratio^(2/3)
                    let r_target = base_radius * ratio.powf(2.0 / 3.0);
                    let diff = (dist - r_target).abs();
                    if diff < min_diff {
                        min_diff = diff;
                        best_r = r_target;
                    }
                }

                if min_diff < 30.0 {
                    // Snap threshold
                    pos = pos.normalize() * best_r;
                    dist = best_r;
                }
            }

            if dist > 10.0 {
                let v_mag = (G * STAR_MASS / dist).sqrt(); // v = sqrt(GM/r)
                                                           // Tangent direction: (-y, x) / r
                let v_dir = Vec2::new(-pos.y, pos.x) / dist;
                let vel = v_dir * v_mag;

                bodies.push(Body::new(
                    pos,
                    vel,
                    1.0,
                    5.0,
                    Color::new(
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        1.0,
                    ),
                ));
                old_positions.push(pos);
            }
        }

        if is_key_pressed(KeyCode::Space) {
            bodies.truncate(1); // Keep star
            old_positions.truncate(1);
        }

        if is_mouse_button_pressed(MouseButton::Right) && bodies.len() > 1 {
            bodies.pop();
            old_positions.pop();
        }

        // Physics
        // Store old positions for checking crossings
        // We need to update old_positions carefully.
        // Actually, update() updates bodies in place.
        // We should capture old positions BEFORE update.
        // But the list size might change (add/remove above).
        // Sync old_positions size first.

        if old_positions.len() != bodies.len() {
            old_positions = bodies.iter().map(|b| b.pos).collect();
        } else {
            for (i, b) in bodies.iter().enumerate() {
                old_positions[i] = b.pos;
            }
        }

        update(&mut bodies, dt);

        // Check Crossings
        let events = check_crossings(&bodies, &old_positions);
        for freq in events {
            audio.play_closest(freq);
            string_flash = 1.0;
        }

        // Draw
        clear_background(BLACK);

        // Center Camera
        set_camera(&Camera2D {
            target: Vec2::new(0.0, 0.0),
            zoom: Vec2::new(
                1.0 / (screen_height() / 2.0),
                -1.0 / (screen_height() / 2.0),
            ), // Flip Y
            ..Default::default()
        });

        // Draw String
        let string_color = Color::new(1.0, 1.0, 1.0, 0.2 + 0.8 * string_flash);
        draw_line(
            0.0,
            0.0,
            2000.0,
            0.0,
            2.0 + 3.0 * string_flash,
            string_color,
        );
        if string_flash > 0.0 {
            string_flash -= dt * 5.0;
        }

        // Draw Resonant Rings
        if harmony_mode {
            for (n, d) in ratios {
                let ratio: f32 = n / d;
                let r = base_radius * ratio.powf(2.0 / 3.0);
                draw_circle_lines(0.0, 0.0, r, 1.0, Color::new(1.0, 1.0, 1.0, 0.1));
            }
        }

        // Draw Bodies
        for body in &bodies {
            // Draw Trail
            for i in 0..body.trail.len().saturating_sub(1) {
                let p1 = body.trail[i];
                let p2 = body.trail[i + 1];
                let alpha = (i as f32 / body.trail.len() as f32) * 0.5;
                draw_line(
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    1.0,
                    Color::new(body.color.r, body.color.g, body.color.b, alpha),
                );
            }

            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
        }

        // UI (Screen Space)
        set_default_camera();
        draw_text(
            "Left Click: Add Planet | Right Click: Remove | Space: Clear",
            10.0,
            30.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Bodies: {}", bodies.len()),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 70.0, 20.0, WHITE);
        if harmony_mode {
            draw_text(
                "Harmony Mode: ON (Snapping to Resonant Orbits)",
                10.0,
                90.0,
                20.0,
                GREEN,
            );
        } else {
            draw_text(
                "Harmony Mode: OFF (Press H to toggle)",
                10.0,
                90.0,
                20.0,
                GRAY,
            );
        }

        next_frame().await
    }
}
