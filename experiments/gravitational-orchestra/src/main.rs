mod audio;
mod physics;
mod visuals;

use audio::{AudioBody, AudioEngine};
use macroquad::prelude::*;
use physics::{verlet_step, Body, G, MAX_BODIES};
use visuals::Visuals;

#[macroquad::main("Gravitational Orchestra")]
async fn main() {
    // 1. Audio
    let (_audio_engine, audio_state) = AudioEngine::new();

    // 2. Visuals
    let visuals = Visuals::new();

    // 3. Physics
    let mut bodies: Vec<Body> = Vec::new();

    // Add a central star
    bodies.push(Body::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        vec2(0.0, 0.0),
        1000.0,
        YELLOW,
    ));

    // Add some orbiting planets
    for i in 0..3 {
        let dist = 200.0 + i as f32 * 100.0;
        let v = (G * 1000.0 / dist).sqrt();
        bodies.push(Body::new(
            vec2(screen_width() / 2.0 + dist, screen_height() / 2.0),
            vec2(0.0, v),
            10.0 + i as f32 * 5.0,
            match i {
                0 => RED,
                1 => GREEN,
                _ => BLUE,
            },
        ));
    }

    loop {
        let dt = get_frame_time().min(0.05);

        // Input
        if is_mouse_button_pressed(MouseButton::Left) && bodies.len() < MAX_BODIES {
            let mpos = mouse_position();
            let pos = vec2(mpos.0, mpos.1);
            let vel = vec2(rand::gen_range(-50.0, 50.0), rand::gen_range(-50.0, 50.0));
            bodies.push(Body::new(
                pos,
                vel,
                rand::gen_range(10.0, 50.0),
                Color::new(
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    rand::gen_range(0.5, 1.0),
                    1.0,
                ),
            ));
        }

        if is_mouse_button_pressed(MouseButton::Right) && !bodies.is_empty() {
            bodies.pop();
        }

        if is_key_pressed(KeyCode::Space) {
            bodies.clear();
            // Reset to central star
            bodies.push(Body::new(
                vec2(screen_width() / 2.0, screen_height() / 2.0),
                vec2(0.0, 0.0),
                1000.0,
                YELLOW,
            ));
        }

        // Physics
        verlet_step(&mut bodies, dt);

        // Wrap around screen? Or just let them fly?
        // Let's implement wrap-around for a "toroidal universe" effect?
        // Or just bounds.
        // For lensing, it's better if they stay on screen.
        let width = screen_width();
        let height = screen_height();
        for body in &mut bodies {
            if body.pos.x < -100.0 {
                body.pos.x = width + 100.0;
            }
            if body.pos.x > width + 100.0 {
                body.pos.x = -100.0;
            }
            if body.pos.y < -100.0 {
                body.pos.y = height + 100.0;
            }
            if body.pos.y > height + 100.0 {
                body.pos.y = -100.0;
            }
        }

        // Sync Audio
        {
            let mut state = audio_state.lock().unwrap();
            state.count = bodies.len();
            let center = vec2(width / 2.0, height / 2.0);
            for (i, body) in bodies.iter().enumerate() {
                if i >= MAX_BODIES {
                    break;
                }
                state.bodies[i] = AudioBody {
                    vel_sq: body.vel.length_squared(),
                    mass: body.mass,
                    dist_sq_from_center: (body.pos - center).length_squared(),
                };
            }
        }

        // Draw
        clear_background(BLACK);
        visuals.draw(&bodies);

        // UI
        draw_text(
            &format!("Bodies: {}", bodies.len()),
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Left Click: Add Body | Right Click: Remove | Space: Reset",
            10.0,
            40.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
