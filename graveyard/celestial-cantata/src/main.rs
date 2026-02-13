pub mod audio;
pub mod physics;

use audio::{AudioEngine, AudioEvent};
use macroquad::prelude::*;
use physics::{Body, Universe, G};
use std::collections::VecDeque;

#[macroquad::main("Celestial Cantata")]
async fn main() {
    let mut universe = Universe::new();
    let audio = AudioEngine::new();

    // Central Star
    universe.add_body(Body {
        pos: Vec2::ZERO,
        vel: Vec2::ZERO,
        mass: 5000.0,
        radius: 30.0,
        color: GOLD,
        trail: VecDeque::new(),
    });

    // Random Planets
    for i in 1..8 {
        let r = 150.0 + i as f32 * 60.0;
        let v = (G * 5000.0 / r).sqrt();
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let pos = Vec2::new(angle.cos() * r, angle.sin() * r);
        let vel = Vec2::new(-angle.sin() * v, angle.cos() * v);

        universe.add_body(Body {
            pos,
            vel,
            mass: rand::gen_range(5.0, 20.0),
            radius: rand::gen_range(5.0, 10.0),
            color: Color::new(
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                rand::gen_range(0.5, 1.0),
                1.0,
            ),
            trail: VecDeque::new(),
        });
    }

    let mut radar_angle = 0.0f32;
    let radar_speed = 1.0; // Radians per second

    let mut cam_zoom = 0.002;
    let mut cam_target = Vec2::ZERO;
    let mut paused = false;

    // Track last angles to detect crossings
    let mut last_radar_angle = 0.0f32;

    loop {
        // Input Handling
        if is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract) {
            cam_zoom *= 0.98;
        }
        if is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd) {
            cam_zoom *= 1.02;
        }

        if is_key_down(KeyCode::Left) {
            cam_target.x -= 10.0 / cam_zoom / 60.0;
        }
        if is_key_down(KeyCode::Right) {
            cam_target.x += 10.0 / cam_zoom / 60.0;
        }
        if is_key_down(KeyCode::Up) {
            cam_target.y += 10.0 / cam_zoom / 60.0;
        }
        if is_key_down(KeyCode::Down) {
            cam_target.y -= 10.0 / cam_zoom / 60.0;
        }

        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset logic could go here, but for now just respawn random planets?
            // Or reload the scene. Simplest is to clear bodies except sun.
            universe.bodies.truncate(1);
            for i in 1..8 {
                let r = 150.0 + i as f32 * 60.0;
                let v = (G * 5000.0 / r).sqrt();
                let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
                let pos = Vec2::new(angle.cos() * r, angle.sin() * r);
                let vel = Vec2::new(-angle.sin() * v, angle.cos() * v);

                universe.add_body(Body {
                    pos,
                    vel,
                    mass: rand::gen_range(5.0, 20.0),
                    radius: rand::gen_range(5.0, 10.0),
                    color: Color::new(
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        1.0,
                    ),
                    trail: VecDeque::new(),
                });
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let world_pos = Camera2D {
                target: cam_target,
                zoom: Vec2::splat(cam_zoom),
                ..Default::default()
            }
            .screen_to_world(Vec2::new(mpos.0, mpos.1));

            // Spawn a body with velocity perpendicular to center
            let r = world_pos.length();
            let v_mag = (G * 5000.0 / r).sqrt();
            let v_dir = Vec2::new(-world_pos.y, world_pos.x).normalize();

            universe.add_body(Body {
                pos: world_pos,
                vel: v_dir * v_mag,
                mass: rand::gen_range(5.0, 15.0),
                radius: rand::gen_range(5.0, 8.0),
                color: WHITE,
                trail: VecDeque::new(),
            });
        }

        // Physics & Logic
        if !paused {
            let dt = 0.016; // Fixed step
            let steps = 4;
            let sub_dt = dt / steps as f32;

            for _ in 0..steps {
                universe.step(sub_dt);
            }

            // Update Radar
            radar_angle += radar_speed * dt;
            if radar_angle > std::f32::consts::PI * 2.0 {
                radar_angle -= std::f32::consts::PI * 2.0;
            }

            // Check Crossings
            for (i, body) in universe.bodies.iter().enumerate() {
                if i == 0 {
                    continue;
                } // Skip Sun

                // Calculate body angle in [0, 2PI)
                let mut body_angle = body.pos.y.atan2(body.pos.x);
                if body_angle < 0.0 {
                    body_angle += std::f32::consts::PI * 2.0;
                }

                // Check if radar crossed body
                // We need to handle wrapping.
                // Simplest: Check if body_angle is in [last_radar_angle, radar_angle]
                // Normal case: last < current
                // Wrap case: current < last (handled by modulo subtraction above, but let's be careful)

                let crossed = if last_radar_angle <= radar_angle {
                    body_angle >= last_radar_angle && body_angle < radar_angle
                } else {
                    // Wrapped around 0/2PI
                    // e.g. last = 6.0, current = 0.1
                    // check if body is in [6.0, 2PI) OR [0, 0.1)
                    body_angle >= last_radar_angle || body_angle < radar_angle
                };

                if crossed {
                    let dist = body.pos.length();
                    audio.play_event(AudioEvent::CrossingZero(i, dist));
                    // Visual feedback? (TODO)
                }
            }
            last_radar_angle = radar_angle;
        }

        // Render
        clear_background(BLACK);

        set_camera(&Camera2D {
            target: cam_target,
            zoom: Vec2::splat(cam_zoom),
            ..Default::default()
        });

        // Draw Orbits/Trails
        for body in &universe.bodies {
            if body.trail.len() < 2 {
                continue;
            }
            for i in 0..body.trail.len() - 1 {
                let p1 = body.trail[i];
                let p2 = body.trail[i + 1];
                let alpha = (i as f32 / body.trail.len() as f32).powf(2.0) * 0.5;
                draw_line(
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    1.0,
                    Color::new(body.color.r, body.color.g, body.color.b, alpha),
                );
            }
        }

        // Draw Radar
        let radar_len = 2000.0;
        let rx = radar_angle.cos() * radar_len;
        let ry = radar_angle.sin() * radar_len;
        draw_line(0.0, 0.0, rx, ry, 2.0, Color::new(1.0, 1.0, 1.0, 0.3));

        // Draw Bodies
        for (i, body) in universe.bodies.iter().enumerate() {
            let color = if i == 0 { YELLOW } else { body.color };
            draw_circle(body.pos.x, body.pos.y, body.radius, color);

            // Glow (simulated)
            draw_circle(
                body.pos.x,
                body.pos.y,
                body.radius * 2.0,
                Color::new(color.r, color.g, color.b, 0.2),
            );
        }

        set_default_camera();
        draw_text("Celestial Cantata", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            "Controls: Arrows (Pan), +/- (Zoom), Space (Pause), R (Reset), Click (Spawn)",
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
