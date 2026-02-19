mod physics;
mod sequencer;
mod audio;

use macroquad::prelude::*;
use physics::{Body, Universe};
use sequencer::Sequencer;
use audio::AudioEngine;

const TRAIL_LENGTH: usize = 200;

#[macroquad::main("Orbital Sequencer")]
async fn main() {
    let mut audio_engine = AudioEngine::new();
    println!("Initializing Audio...");
    audio_engine.init().await;
    println!("Audio Initialized.");

    let mut universe = Universe::new();
    universe.g_const = 100.0; // Stronger gravity for visibility
    universe.use_newtonian = false; // Start with Keplerian (Stable Rhythms)

    // Star
    universe.add_body(Body::new(
        0,
        Vec2::ZERO,
        Vec2::ZERO,
        10000.0,
        20.0,
        YELLOW,
    ));

    // Planets
    let radii = [150.0, 250.0, 350.0, 450.0, 550.0];
    let colors = [RED, GREEN, BLUE, PURPLE, ORANGE];

    for (i, &r) in radii.iter().enumerate() {
        let v_mag = (universe.g_const * universe.bodies[0].mass / r).sqrt();
        let pos = Vec2::new(r, 0.0);
        let vel = Vec2::new(0.0, v_mag);

        universe.add_body(Body::new(
            i + 1,
            pos,
            vel,
            10.0,
            8.0,
            colors[i % colors.len()],
        ));
    }

    let mut sequencer = Sequencer::new();

    // Trails: Map body ID to list of positions
    let mut trails: Vec<Vec<Vec2>> = vec![Vec::new(); universe.bodies.len()];

    let mut dragging_body: Option<usize> = None;
    let mut camera_zoom = 0.002;
    let mut camera_target = Vec2::ZERO;

    let mut triggers_this_frame: Vec<usize> = Vec::new();
    let mut trigger_timers: Vec<f32> = vec![0.0; universe.bodies.len()]; // For visual flash

    loop {
        let dt = get_frame_time(); // Time since last frame
        // Clamp dt to avoid explosion on lag spikes
        let dt = dt.min(0.1);

        // Input Handling
        if is_key_pressed(KeyCode::R) {
            // Reset
            // Re-init universe bodies... (Simplified: just reload scene or manually reset)
            // For now, let's just clear trails.
            for t in &mut trails { t.clear(); }
        }

        if is_key_pressed(KeyCode::K) {
            universe.use_newtonian = !universe.use_newtonian;
        }

        if is_key_pressed(KeyCode::C) {
             for t in &mut trails { t.clear(); }
        }

        // Camera Pan/Zoom
        if is_key_down(KeyCode::Up) { camera_zoom *= 1.01; }
        if is_key_down(KeyCode::Down) { camera_zoom *= 0.99; }
        if is_key_down(KeyCode::Left) { camera_target.x -= 10.0 / camera_zoom; } // Inverted logic? No.

        let mouse_pos = mouse_position();

        // Easier: Use Camera2D
        // If I use set_camera, mouse_position_local() or screen_to_world() works better.

        let camera = Camera2D {
            target: camera_target,
            zoom: Vec2::splat(camera_zoom),
            ..Default::default()
        };
        set_camera(&camera);
        let mouse_world = camera.screen_to_world(Vec2::new(mouse_pos.0, mouse_pos.1));

        if is_mouse_button_pressed(MouseButton::Left) {
            // Check click on body
            for body in &universe.bodies {
                if (body.position - mouse_world).length() < body.radius * 2.0 {
                    dragging_body = Some(body.id);
                    break;
                }
            }
        }

        if is_mouse_button_released(MouseButton::Left) {
            dragging_body = None;
        }

        if let Some(id) = dragging_body {
            // Find body index
            if let Some(body) = universe.bodies.iter_mut().find(|b| b.id == id) {
                body.position = mouse_world;
                // Zero velocity while dragging? Or fling?
                // Let's zero it so user can place it.
                body.velocity = Vec2::ZERO;
            }
        } else {
            // Physics Step
            // Sub-step for stability?
            let steps = 4;
            let sub_dt = dt / steps as f32;
            for _ in 0..steps {
                universe.step(sub_dt);
            }
        }

        // Trails
        if get_frame_time() > 0.0 { // Throttle?
             for body in &universe.bodies {
                if body.id >= trails.len() { continue; }
                let trail = &mut trails[body.id];
                if trail.is_empty() || (*trail.last().unwrap() - body.position).length_squared() > 10.0 {
                    trail.push(body.position);
                    if trail.len() > TRAIL_LENGTH {
                        trail.remove(0);
                    }
                }
             }
        }

        // Sequencer
        let new_triggers = sequencer.check_triggers(&universe);
        triggers_this_frame = new_triggers.clone();

        for &id in &triggers_this_frame {
            // Map ID to Note Index
            // Planet 1 (ID 1) -> Index 0
            // Planet 2 (ID 2) -> Index 1
            if id > 0 {
                audio_engine.play_note(id - 1);
                // Flash timer
                if id < trigger_timers.len() {
                    trigger_timers[id] = 0.2; // 200ms flash
                }
            }
        }

        // Draw
        clear_background(BLACK);

        // Draw Trigger Line
        draw_line(0.0, 0.0, 10000.0, 0.0, 2.0, DARKGRAY);

        // Draw Bodies & Trails
        for (i, body) in universe.bodies.iter().enumerate() {
            // Trail
            if i < trails.len() {
                for j in 0..trails[i].len().saturating_sub(1) {
                    draw_line(
                        trails[i][j].x, trails[i][j].y,
                        trails[i][j+1].x, trails[i][j+1].y,
                        1.0,
                        Color::new(body.color.r, body.color.g, body.color.b, 0.5),
                    );
                }
            }

            // Body
            let mut color = body.color;
            // Flash if triggered
            if i < trigger_timers.len() {
                if trigger_timers[i] > 0.0 {
                    trigger_timers[i] -= dt;
                    color = WHITE;
                    // Draw ring
                    draw_circle_lines(body.position.x, body.position.y, body.radius * 2.0, 2.0, WHITE);
                }
            }

            draw_circle(body.position.x, body.position.y, body.radius, color);
        }

        // UI
        set_default_camera();
        draw_text("Orbital Sequencer", 20.0, 30.0, 30.0, WHITE);
        draw_text(if universe.use_newtonian { "Mode: Newtonian (Chaos)" } else { "Mode: Keplerian (Stable)" }, 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text("Drag planets to change orbit. K to toggle mode. C to clear trails.", 20.0, 80.0, 20.0, GRAY);

        next_frame().await
    }
}
