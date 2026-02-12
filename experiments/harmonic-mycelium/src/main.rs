mod physics;
mod audio;
mod fungus;
mod gravity;

use macroquad::prelude::*;
use crate::audio::AudioEngine;
use crate::physics::{Body, update, G}; // check_crossings removed, inlined
use crate::fungus::HyphaeNetwork;
use crate::gravity::GravityMap;

const STAR_MASS: f32 = 50000.0;
const GRID_W: usize = 150;
const GRID_H: usize = 100;

#[macroquad::main("Harmonic Mycelium")]
async fn main() {
    let audio = AudioEngine::new().await;
    let mut bodies = Vec::new();

    // Create Star
    bodies.push(Body::new(
        Vec2::ZERO,
        Vec2::ZERO,
        STAR_MASS,
        20.0,
        YELLOW,
    ));

    // Previous positions for crossing detection
    let mut old_positions: Vec<Vec2> = bodies.iter().map(|b| b.pos).collect();

    // Initialize Fungus
    let start_pos = IVec2::new(GRID_W as i32 / 2, GRID_H as i32 / 2);
    let mut fungus = HyphaeNetwork::new(GRID_W, GRID_H, start_pos);

    // Connected status tracking for planets (by index)
    let mut connected_status = vec![false; 1]; // Only Star initially

    loop {
        let dt = get_frame_time().min(0.05);

        // Input: Add Planet
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            // Map mouse (screen) to world (center 0,0, Y up)
            // Camera zoom is (2/w, -2/h).
            // Mouse (0,0) -> (-w/2, h/2)?
            // Let's match the camera transform I use for drawing.
            // Screen center is (w/2, h/2).
            // World 0,0 is at screen center.
            // Mouse x - screen_w/2 is dist from center.
            // Mouse y - screen_h/2 is dist from center.
            // Since camera Y is flipped (-2/h), world Y is -(mouse_y - h/2).

            let pos = Vec2::new(mpos.0 - screen_width() / 2.0, -(mpos.1 - screen_height() / 2.0));

            let dist = pos.length();
            if dist > 20.0 {
                let v_mag = (G * STAR_MASS / dist).sqrt();
                let v_dir = Vec2::new(-pos.y, pos.x) / dist;
                let vel = v_dir * v_mag;

                bodies.push(Body::new(
                    pos,
                    vel,
                    100.0, // Planet mass
                    5.0,
                    Color::new(rand::gen_range(0.5, 1.0), rand::gen_range(0.5, 1.0), rand::gen_range(0.5, 1.0), 1.0),
                ));
                connected_status.push(false);
                old_positions.push(pos);
            }
        }

        // Clear
        if is_key_pressed(KeyCode::Space) {
            bodies.truncate(1);
            connected_status.truncate(1);
            old_positions.truncate(1);
            fungus.reset(start_pos);
        }

        // Physics Update
        // Sync old_positions size
        if old_positions.len() != bodies.len() {
            old_positions = bodies.iter().map(|b| b.pos).collect();
        } else {
            for (i, b) in bodies.iter().enumerate() {
                old_positions[i] = b.pos;
            }
        }

        update(&mut bodies, dt);

        // Fungus Update
        let world_size = Vec2::new(screen_width(), screen_height());

        let gravity_map = GravityMap {
            bodies: &bodies,
            width: GRID_W,
            height: GRID_H,
            world_size,
        };

        fungus.reset(start_pos);
        fungus.update(&gravity_map, 20000);

        // Check Connectivity
        for (i, body) in bodies.iter().enumerate() {
            if i == 0 {
                connected_status[0] = true;
                continue;
            }

            let center_x = GRID_W as f32 / 2.0;
            let center_y = GRID_H as f32 / 2.0;
            let scale_x = world_size.x / GRID_W as f32;
            let scale_y = world_size.y / GRID_H as f32;

            let gx = (body.pos.x / scale_x + center_x) as i32;
            let gy = (-body.pos.y / scale_y + center_y) as i32;

            let is_connected = fungus.is_connected(IVec2::new(gx, gy));
            connected_status[i] = is_connected;
        }

        // Check Crossings & Audio
        for (i, body) in bodies.iter().enumerate() {
            if i == 0 { continue; }
            if i >= old_positions.len() { continue; }
            if !connected_status[i] { continue; } // Silence if not connected!

            let p1 = old_positions[i];
            let p2 = body.pos;

            // Check crossing positive X axis (y sign change, x > 0)
            if p1.y.signum() != p2.y.signum() {
                 let denom = p2.y - p1.y;
                if denom.abs() > f32::EPSILON {
                    let t = -p1.y / denom;
                    let x_cross = p1.x + (p2.x - p1.x) * t;
                    if x_cross > 0.0 {
                        let freq = body.vel.length();
                        audio.play_closest(freq);
                    }
                }
            }
        }

        // Draw
        clear_background(BLACK);

        let center_x = GRID_W as f32 / 2.0;
        let center_y = GRID_H as f32 / 2.0;
        let scale_x = world_size.x / GRID_W as f32;
        let scale_y = world_size.y / GRID_H as f32;

        set_camera(&Camera2D {
            target: Vec2::ZERO,
            zoom: Vec2::new(2.0 / screen_width(), -2.0 / screen_height()),
            ..Default::default()
        });

        // Draw Fungus
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let idx = y * GRID_W + x;
                if let Some(parent) = fungus.parent[idx] {
                     let wx = (x as f32 - center_x) * scale_x;
                     let wy = -(y as f32 - center_y) * scale_y;

                     let px = (parent.x as f32 - center_x) * scale_x;
                     let py = -(parent.y as f32 - center_y) * scale_y;

                     let col = Color::new(0.3, 0.3, 0.3, 0.3);
                     draw_line(px, py, wx, wy, 1.0, col);
                }
            }
        }

        // Draw Bodies
        for (i, body) in bodies.iter().enumerate() {
             for j in 0..body.trail.len().saturating_sub(1) {
                let p1 = body.trail[j];
                let p2 = body.trail[j+1];
                let alpha = (j as f32 / body.trail.len() as f32) * 0.5;
                draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, Color::new(body.color.r, body.color.g, body.color.b, alpha));
            }

            let mut col = body.color;
            if i > 0 && !connected_status[i] {
                col.a = 0.2;
            } else {
                col.a = 1.0;
            }
            draw_circle(body.pos.x, body.pos.y, body.radius, col);

            if i > 0 && connected_status[i] {
                draw_circle_lines(body.pos.x, body.pos.y, body.radius + 3.0, 1.0, WHITE);
            }
        }

        draw_line(0.0, 0.0, 1000.0, 0.0, 1.0, Color::new(1.0, 1.0, 1.0, 0.1));

        set_default_camera();
        draw_text("Left Click: Add Planet | Space: Clear", 10.0, 30.0, 20.0, WHITE);
        draw_text(&format!("Bodies: {}", bodies.len()), 10.0, 50.0, 20.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 70.0, 20.0, WHITE);
        draw_text("Planets only sing when connected by Mycelium", 10.0, 90.0, 20.0, GREEN);

        next_frame().await
    }
}
