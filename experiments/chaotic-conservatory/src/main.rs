use macroquad::prelude::*;

pub mod attractor;
pub mod grammar;
pub mod turtle;

use attractor::LorenzParams;
use grammar::LSystem;
use turtle::ChaoticTurtle;

#[macroquad::main("Chaotic Conservatory")]
async fn main() {
    let mut params = LorenzParams::default();

    // L-System Rules: A simple 3D structure
    // F: Forward, +: Turn Left, -: Turn Right
    // &: Pitch Down, ^: Pitch Up
    // \: Roll Left, /: Roll Right (using < and > in code)
    // [: Push, ]: Pop

    // A 3D plant-like structure
    // Axiom: X
    // X -> F[&+X]F[->X][<X]
    // F -> FF

    let rules = vec![('X', "F[&+X]F[->X][<X]"), ('F', "FF")];
    let lsystem = LSystem::new("X", rules);
    let mut iterations = 4;

    let mut cam = Camera3D {
        position: vec3(0.0, 20.0, 50.0),
        target: vec3(0.0, 10.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        ..Default::default()
    };

    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.2;
    let mut cam_dist: f32 = 50.0;

    // Initial generation
    let mut turtle = ChaoticTurtle::new(vec3(0.0, 0.0, 0.0), params);
    let mut expanded = lsystem.expand(iterations);
    let mut need_regen = true;

    loop {
        // Input Handling
        if is_key_pressed(KeyCode::Up) {
            iterations += 1;
            expanded = lsystem.expand(iterations);
            need_regen = true;
        }
        if is_key_pressed(KeyCode::Down) {
            if iterations > 0 {
                iterations -= 1;
            }
            expanded = lsystem.expand(iterations);
            need_regen = true;
        }

        // Chaos Control
        if is_key_down(KeyCode::Right) {
            params.rho += 0.1;
            need_regen = true;
        }
        if is_key_down(KeyCode::Left) {
            params.rho -= 0.1;
            need_regen = true;
        }

        // Camera Control
        if is_key_down(KeyCode::W) {
            cam_pitch += 0.02;
        }
        if is_key_down(KeyCode::S) {
            cam_pitch -= 0.02;
        }
        if is_key_down(KeyCode::A) {
            cam_yaw -= 0.02;
        }
        if is_key_down(KeyCode::D) {
            cam_yaw += 0.02;
        }
        if is_key_down(KeyCode::Q) {
            cam_dist += 0.5;
        }
        if is_key_down(KeyCode::E) {
            cam_dist -= 0.5;
        }

        cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        cam_dist = cam_dist.max(1.0);

        cam.position = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );
        cam.target = vec3(0.0, 10.0, 0.0);

        if need_regen {
            // Reset turtle
            turtle = ChaoticTurtle::new(vec3(0.0, 0.0, 0.0), params);
            turtle.set_flow_influence(0.05); // Moderate influence

            let step_len = 1.0; // Base step length
            let angle = 25.0;

            for c in expanded.chars() {
                match c {
                    'F' => turtle.forward(step_len, GREEN),
                    'X' => {} // Placeholder
                    '+' => turtle.turn(angle),
                    '-' => turtle.turn(-angle),
                    '&' => turtle.pitch(angle),
                    '^' => turtle.pitch(-angle),
                    '<' => turtle.roll(angle),
                    '>' => turtle.roll(-angle),
                    '[' => turtle.push(),
                    ']' => turtle.pop(),
                    _ => {}
                }
            }
            need_regen = false;
        }

        clear_background(BLACK);

        set_camera(&cam);

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        // Draw the "Chaotic Tree"
        for (start, end, color) in &turtle.lines {
            // Check if line is within view distance or something? No, just draw.
            draw_line_3d(*start, *end, *color);
        }

        set_default_camera();

        // UI
        draw_text("Chaotic Conservatory", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Gen: {} (Up/Down)", iterations),
            10.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Chaos (Rho): {:.1} (Left/Right)", params.rho),
            10.0,
            80.0,
            20.0,
            YELLOW,
        );
        draw_text(
            &format!("Segments: {}", turtle.lines.len()),
            10.0,
            100.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
