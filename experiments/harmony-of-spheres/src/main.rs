pub mod audio;
pub mod physics;

use audio::{init_audio, Oscillator, SharedState};
use macroquad::prelude::*;
use physics::{Body, Universe, G};
use std::sync::{Arc, RwLock};

#[macroquad::main("Harmony of Spheres")]
async fn main() {
    let mut universe = Universe::new();

    // Central Star
    universe.add_body(Body::new(0.0, 0.0, 5000.0, 20.0, YELLOW));

    // Planet 1
    let r1 = 200.0;
    let v1 = (G * 5000.0 / r1).sqrt();
    universe.add_body(Body::new(r1, 0.0, 10.0, 8.0, BLUE).with_velocity(0.0, v1));

    // Planet 2
    let r2 = 350.0;
    let v2 = (G * 5000.0 / r2).sqrt();
    universe.add_body(Body::new(r2, 0.0, 20.0, 12.0, RED).with_velocity(0.0, v2));

    // Planet 3
    let r3 = 500.0;
    let v3 = (G * 5000.0 / r3).sqrt() * 0.8;
    universe.add_body(Body::new(r3, 0.0, 5.0, 6.0, GREEN).with_velocity(0.0, v3));

    // Audio Setup
    let audio_state = Arc::new(RwLock::new(SharedState::new()));

    #[allow(unused_variables)]
    let stream = init_audio(audio_state.clone()).unwrap_or_else(|e| {
        eprintln!("Audio init failed: {}", e);
        audio::AudioHandle {
            #[cfg(feature = "audio")]
            _stream: None,
        }
    });

    let mut cam_zoom = 0.002;
    let mut cam_target = Vec2::ZERO;
    let mut paused = false;

    loop {
        if is_key_down(KeyCode::Minus) || is_key_down(KeyCode::KpSubtract) {
            cam_zoom *= 0.98;
        }
        if is_key_down(KeyCode::Equal) || is_key_down(KeyCode::KpAdd) {
            cam_zoom *= 1.02;
        }

        if is_key_down(KeyCode::Left) { cam_target.x -= 10.0 / cam_zoom / 60.0; }
        if is_key_down(KeyCode::Right) { cam_target.x += 10.0 / cam_zoom / 60.0; }
        if is_key_down(KeyCode::Up) { cam_target.y += 10.0 / cam_zoom / 60.0; }
        if is_key_down(KeyCode::Down) { cam_target.y -= 10.0 / cam_zoom / 60.0; }

        if is_key_pressed(KeyCode::Space) { paused = !paused; }

        if is_key_pressed(KeyCode::R) {
            universe = Universe::new();
            universe.add_body(Body::new(0.0, 0.0, 5000.0, 20.0, YELLOW));
        }

        if is_key_pressed(KeyCode::C) {
            for body in &mut universe.bodies {
                body.trail.clear();
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let world_pos = Camera2D {
                target: cam_target,
                zoom: Vec2::splat(cam_zoom),
                ..Default::default()
            }.screen_to_world(Vec2::new(mpos.0, mpos.1));

            let dist = world_pos.length();
            if dist > 10.0 {
                let v_circ = (G * 5000.0 / dist).sqrt();
                let v_dir = Vec2::new(-world_pos.y, world_pos.x).normalize();

                let mass = rand::gen_range(2.0, 10.0);
                let radius = mass;
                let color = Color::new(
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    1.0,
                );

                universe.add_body(
                    Body::new(world_pos.x, world_pos.y, mass, radius, color)
                        .with_velocity(v_dir.x * v_circ, v_dir.y * v_circ),
                );
            }
        }

        if !paused {
            let dt = 0.05;
            let physics_steps = 4;
            let physics_dt = dt / physics_steps as f32;

            for _ in 0..physics_steps {
                universe.step(physics_dt);
            }
        }

        if let Ok(mut state) = audio_state.write() {
            state.oscillators.clear();
            for (i, body) in universe.bodies.iter().enumerate() {
                if i == 0 { continue; }
                let speed = body.vel.length();
                let dist = body.pos.length();
                let freq = 50.0 + speed * 10.0;
                let vol = (1000.0 / (dist + 100.0)).clamp(0.0, 0.5);
                state.oscillators.push(Oscillator { frequency: freq, amplitude: vol });
            }
        }

        clear_background(BLACK);
        set_camera(&Camera2D {
            target: cam_target,
            zoom: Vec2::splat(cam_zoom),
            ..Default::default()
        });

        for body in &universe.bodies {
            if body.trail.len() < 2 { continue; }
            for i in 0..body.trail.len() - 1 {
                let p1 = body.trail[i];
                let p2 = body.trail[i+1];
                let alpha = (i as f32 / body.trail.len() as f32).powf(2.0);
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0 * alpha, Color::new(body.color.r, body.color.g, body.color.b, alpha));
            }
        }

        for body in &universe.bodies {
            draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
        }

        set_default_camera();
        draw_text("Harmony of Spheres", 10.0, 20.0, 30.0, GOLD);
        draw_text("Controls: Arrows (Pan), +/- (Zoom), Click (Spawn), Space (Pause), R (Reset)", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Bodies: {}", universe.bodies.len()), 10.0, screen_height() - 40.0, 20.0, WHITE);

        next_frame().await
    }
}
