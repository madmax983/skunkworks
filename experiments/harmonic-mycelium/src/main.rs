mod audio;
mod fungus;
mod physics;

use crate::audio::AudioEngine;
use crate::fungus::HyphaeNetwork;
use crate::physics::{check_crossings, update, Body, G};
use macroquad::prelude::*;

const STAR_MASS: f32 = 50000.0;
const GRID_W: usize = 150;
const GRID_H: usize = 100;

#[macroquad::main("Harmonic Mycelium")]
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

    let mut old_positions: Vec<Vec2> = bodies.iter().map(|b| b.pos).collect();

    // Initialize Fungus starting at center (Star)
    let start_pos = IVec2::new(GRID_W as i32 / 2, GRID_H as i32 / 2);
    let mut fungus = HyphaeNetwork::new(GRID_W, GRID_H, start_pos);

    loop {
        let dt = get_frame_time().min(0.05);
        let screen_w = screen_width();
        let screen_h = screen_height();

        // --- Input ---
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            // Map screen to world (center origin)
            let pos = Vec2::new(mpos.0 - screen_w / 2.0, mpos.1 - screen_h / 2.0);
            let dist = pos.length();

            if dist > 20.0 {
                // Circular orbit velocity v = sqrt(GM/r)
                let v_mag = (G * STAR_MASS / dist).sqrt();
                let v_dir = Vec2::new(-pos.y, pos.x).normalize();
                let vel = v_dir * v_mag;

                bodies.push(Body::new(
                    pos,
                    vel,
                    100.0, // Planet mass
                    8.0,
                    Color::new(
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(0.5, 1.0),
                        rand::gen_range(1.0, 1.0),
                        1.0,
                    ),
                ));
            }
        }

        if is_key_pressed(KeyCode::R) {
            bodies.truncate(1); // Keep star
            fungus.reset();
        }

        // --- Physics ---
        if old_positions.len() != bodies.len() {
            old_positions = bodies.iter().map(|b| b.pos).collect();
        } else {
            for (i, b) in bodies.iter().enumerate() {
                old_positions[i] = b.pos;
            }
        }

        update(&mut bodies, dt);

        // --- Audio ---
        let events = check_crossings(&bodies, &old_positions);
        for freq in events {
            audio.play_closest(freq);
        }

        // --- Fungus ---
        // Update targets (Planets)
        let targets: Vec<IVec2> = bodies
            .iter()
            .skip(1)
            .map(|b| {
                // Map world pos to grid pos
                // World: (-w/2, -h/2) to (w/2, h/2)
                // Grid: (0, 0) to (GRID_W, GRID_H)

                // world_x = (grid_x / GRID_W - 0.5) * screen_w
                // grid_x = (world_x / screen_w + 0.5) * GRID_W

                let gx = ((b.pos.x / screen_w) + 0.5) * GRID_W as f32;
                let gy = ((b.pos.y / screen_h) + 0.5) * GRID_H as f32;
                IVec2::new(gx as i32, gy as i32)
            })
            .collect();

        fungus.set_targets(targets);

        // Grow
        // Cost Function: Inverse Potential
        fungus.update(
            |x, y| {
                // Convert grid (x,y) to world
                let wx = (x as f32 / GRID_W as f32 - 0.5) * screen_w;
                let wy = (y as f32 / GRID_H as f32 - 0.5) * screen_h;
                let p = Vec2::new(wx, wy);

                let mut potential = 0.0;
                for b in &bodies {
                    let r = p.distance(b.pos).max(10.0); // Avoid singularity
                    potential += (G * b.mass) / r; // Potential is negative, but we use magnitude here
                }

                // Higher potential (closer to mass) -> Lower cost
                // Potential near star ~ 1000 * 50000 / 20 ~ 2,500,000
                // Potential far away ~ 1000 * 50000 / 500 ~ 100,000

                // We want cost ~1.0 near star and ~25.0 far away to create a gradient.
                // Cost = K / Potential
                // K = 2,500,000

                let cost = 2_500_000.0 / (potential + 1.0);
                cost.max(1.0)
            },
            100,
        ); // 100 steps per frame

        // --- Draw ---
        clear_background(BLACK);

        // Draw Fungus
        let cell_size = Vec2::new(screen_w / GRID_W as f32, screen_h / GRID_H as f32);

        // Use a camera to center (0,0) for bodies, but fungus draws in screen space (0..w, 0..h)
        // Wait, fungus.draw uses screen coordinates 0..screen_w derived from cell_size.
        // But bodies use center-origin coordinates.
        // So we need to draw bodies with a camera, and fungus without?
        // Or shift fungus drawing?
        // Fungus logic assumes (0,0) is top-left of grid.
        // My coordinate mapping: grid(0,0) -> world(-w/2, -h/2).
        // This corresponds to top-left of screen if camera is centered.

        // Let's draw bodies first with camera.
        set_camera(&Camera2D {
            target: Vec2::new(0.0, 0.0),
            zoom: Vec2::new(2.0 / screen_w, 2.0 / screen_h), // Y up? No, macroquad default is Y up for Camera2D?
            // "Camera2D::default().zoom is (1.0, 1.0). (0, 0) is the center of the screen."
            // "The coordinate system is: (-1, -1) bottom-left, (1, 1) top-right."
            // We want screen_w/2 to be 1.0. So zoom = 2.0 / screen_w.
            // Macroquad Y is UP in Camera2D?
            // Standard macroquad screen space (draw_line etc without camera) is Y DOWN.
            // When using Camera2D, Y is usually UP unless we flip it.
            // Harmony of Spheres used: `zoom: Vec2::new(1.0 / (screen_height() / 2.0), -1.0 / (screen_height() / 2.0))`
            // This suggests flipping Y to match screen space? Or maybe it uses standard math coords.
            // Let's check harmony-of-spheres main.rs again.
            // `zoom: Vec2::new(1.0 / (screen_height() / 2.0), -1.0 / (screen_height() / 2.0))`
            // This maps height/2 to 1.0. And flips Y.

            // Let's use standard screen coordinates for everything to avoid confusion.
            // Shift bodies by screen_w/2, screen_h/2.
            ..Default::default()
        });

        // Draw Fungus (Grid covers -w/2 to w/2)
        // fungus.draw iterates 0..W.
        // p = index * cell_size.
        // If cell_size = screen_w / GRID_W, then p goes from 0 to screen_w.
        // But we are in camera space where -w/2 is left.
        // So we need to subtract screen_w/2 from fungus draw coordinates.
        // This requires modifying fungus.draw or using `draw_line` with offset.

        // Simpler: Use screen space for everything.
        // Don't use `set_camera`.
        // Add `center` offset to body drawing.
        set_default_camera();

        let center = Vec2::new(screen_w / 2.0, screen_h / 2.0);

        // Draw Fungus
        // Fungus draws from 0,0 to screen_w, screen_h. This matches default camera.
        // But we computed fungus logic assuming center is (w/2, h/2).
        // "let gx = ((b.pos.x / screen_w) + 0.5) * GRID_W" implies b.pos is relative to center.
        // Yes.
        // So fungus grid covers the screen.
        fungus.draw(cell_size);

        // Draw Bodies (shifted)
        for body in &bodies {
            // Trails
            for i in 0..body.trail.len().saturating_sub(1) {
                let p1 = body.trail[i] + center;
                let p2 = body.trail[i + 1] + center;
                draw_line(
                    p1.x,
                    p1.y,
                    p2.x,
                    p2.y,
                    1.0,
                    Color::new(body.color.r, body.color.g, body.color.b, 0.5),
                );
            }

            let p = body.pos + center;
            draw_circle(p.x, p.y, body.radius, body.color);
        }

        // UI
        draw_text("Harmonic Mycelium", 10.0, 30.0, 20.0, WHITE);
        draw_text("Click to add planet. R to reset.", 10.0, 50.0, 20.0, GRAY);
        draw_text(
            &format!("Bodies: {}", bodies.len()),
            10.0,
            70.0,
            20.0,
            WHITE,
        );
        draw_text(
            &format!("Tips: {}", fungus.active_tips.len()),
            10.0,
            90.0,
            20.0,
            GREEN,
        );

        next_frame().await
    }
}
