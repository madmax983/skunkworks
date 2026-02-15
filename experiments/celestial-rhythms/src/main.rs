mod physics;
mod audio;

use macroquad::prelude::*;
use physics::{Body, System};
use audio::AudioEngine;

#[macroquad::main("Celestial Rhythms")]
async fn main() -> anyhow::Result<()> {
    let mut system = System::new();

    // Attempt to initialize audio, but don't crash if it fails (e.g. in CI without audio device)
    let audio_engine = match AudioEngine::new() {
        Ok(engine) => Some(engine),
        Err(e) => {
            eprintln!("Audio initialization failed: {}", e);
            None
        }
    };

    // Setup Solar System
    let sun = Body::new(
        Vec2::ZERO,
        Vec2::ZERO,
        50000.0, // Massive Sun
        30.0,
        YELLOW,
    );
    system.add_body(sun);

    // Planets
    let colors = [
        RED, BLUE, GREEN, ORANGE, PURPLE, PINK, SKYBLUE, MAGENTA, GOLD, LIME,
    ];

    // Scale for "Earth"
    let base_dist = 300.0;

    // Create 8 planets
    for i in 0..8 {
        // Distance roughly following Bode's law or just linear for fun
        let dist = base_dist + (i as f32) * 80.0;

        // Circular orbit velocity: v = sqrt(GM/r)
        // We want stable circular orbits initially
        let vel_mag = (system.g_const * system.bodies[0].mass / dist).sqrt();

        // Randomize initial angle
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        let pos = vec2(angle.cos() * dist, angle.sin() * dist);

        // Perpendicular velocity for circular orbit
        // Tangent is (-sin, cos)
        let v_vec = vec2(-angle.sin(), angle.cos()) * vel_mag;

        // Add some eccentricity randomly?
        // let v_vec = v_vec * rand::gen_range(0.8, 1.2);

        system.add_body(Body::new(
            pos,
            v_vec,
            10.0, // Small mass relative to Sun
            8.0 + (i as f32) * 2.0,
            colors[i % colors.len()],
        ));
    }

    let mut scale_factor = 200000.0; // Audio frequency scale (needs to be high for slow orbits)
    // Earth omega ~ 2pi / year.
    // Here, G=1000, M=50000, R=300. v=sqrt(5e7/300)=408. T=2pi*300/408 = 4.6s.
    // f = 1/4.6 = 0.2 Hz.
    // To get 200Hz, scale needs to be 1000.
    // Let's adjust based on hearing.

    let mut paused = false;
    let mut trails: Vec<Vec<Vec2>> = vec![Vec::new(); 9]; // Sun + 8 planets

    loop {
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_down(KeyCode::Up) {
            scale_factor *= 1.01;
        }
        if is_key_down(KeyCode::Down) {
            scale_factor *= 0.99;
        }
        if is_key_pressed(KeyCode::R) {
             // Reset? Maybe later.
        }

        let dt = get_frame_time();

        if !paused {
            // Physics Update (substepping for stability)
            let steps = 4;
            let sub_dt = dt / steps as f32;
            for _ in 0..steps {
                system.update(sub_dt);
            }
        }

        // Update Trails
        if !paused {
            // Ensure trails vector matches bodies
            if trails.len() != system.bodies.len() {
                trails.resize(system.bodies.len(), Vec::new());
            }

            for (i, body) in system.bodies.iter().enumerate() {
                // Only add point if moved enough? Or every N frames.
                // Adding every frame makes smooth lines but uses memory.
                if trails[i].is_empty() || (body.pos - *trails[i].last().unwrap()).length_squared() > 4.0 {
                    trails[i].push(body.pos);
                    if trails[i].len() > 500 {
                        trails[i].remove(0);
                    }
                }
            }
        }

        // Audio Update
        if let Some(engine) = &audio_engine {
            let sun_pos = system.bodies[0].pos;
            let mut audio_params = Vec::new();

            for (i, body) in system.bodies.iter().enumerate() {
                if i == 0 { continue; } // Skip Sun (or make it a drone?)

                // Angular velocity relative to Sun
                let omega = body.angular_velocity(sun_pos).abs();

                // Map to frequency
                // Clamp to human hearing range roughly (20Hz - 20kHz)
                let freq = (omega * scale_factor).clamp(20.0, 2000.0);

                // Amplitude
                // Distance decay: 1/r
                let dist = (body.pos - sun_pos).length();
                let amp = (500.0 / (dist + 1.0)).clamp(0.0, 0.5); // Max 0.5 per oscillator

                audio_params.push((freq, amp));
            }

            engine.update_oscillators(&audio_params);
        }

        // Render
        clear_background(BLACK);

        // Dynamic Camera
        // Center on centroid? Or Sun?
        // Let's center on Sun for stability.
        let sun_pos = system.bodies[0].pos;

        // Simple zoom control?
        let zoom = 0.001; // Fits ~2000 units width

        set_camera(&Camera2D {
            zoom: vec2(zoom, -zoom * screen_width() / screen_height()), // Fix aspect ratio
            target: sun_pos,
            ..Default::default()
        });

        // Draw Grid
        draw_circle(0., 0., 5.0, WHITE); // Origin marker

        // Draw Trails
        for (i, trail) in trails.iter().enumerate() {
            if trail.len() < 2 { continue; }
            let color = system.bodies[i].color;
            // Draw as connected lines
            for j in 0..trail.len()-1 {
                draw_line(trail[j].x, trail[j].y, trail[j+1].x, trail[j+1].y, 2.0, color);
            }
        }

        // Draw Bodies
        for body in &system.bodies {
             draw_circle(body.pos.x, body.pos.y, body.radius, body.color);
        }

        set_default_camera();

        // HUD
        draw_text("CELESTIAL RHYTHMS", 10.0, 30.0, 30.0, WHITE);
        draw_text("Space: Pause | Up/Down: Tune Scale", 10.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Scale: {:.1}", scale_factor), 10.0, 70.0, 20.0, GOLD);

        // Visualize frequencies
        if let Some(_) = &audio_engine {
             let mut y = 100.0;
             for (i, body) in system.bodies.iter().skip(1).enumerate() {
                 let omega = body.angular_velocity(sun_pos).abs();
                 let freq = omega * scale_factor;
                 draw_text(&format!("Planet {}: {:.1} Hz", i+1, freq), 10.0, y, 20.0, body.color);
                 y += 20.0;
             }
        }

        next_frame().await
    }
}
