mod render;
mod vehicle;

use macroquad::prelude::*;
use poincare_disk::{
    hyperbolic_dist, mobius_add, neighbor_transform_a, Mobius, Point, TilingConsts,
};
use render::{draw_tile, is_wall, to_screen};
use std::f64::consts::PI;
use vehicle::Vehicle;

struct GameState {
    player_pos: Point,
    player_path: Vec<usize>,
    view_angle: f64,

    tiling: TilingConsts,

    vehicles: Vec<Vehicle>,

    message: String,
    message_timer: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_pos: Point::new(0.0, 0.0),
            player_path: Vec::new(),
            view_angle: 0.0,
            tiling: TilingConsts::new_4_5(),
            vehicles: Vec::new(),
            message: "WASD to Move. Click to Spawn.".to_string(),
            message_timer: 5.0,
        }
    }

    fn get_path_hash(&self) -> u64 {
        let mut h: u64 = 123456789;
        for &step in &self.player_path {
            h = h
                .wrapping_mul(6364136223846793005)
                .wrapping_add(step as u64);
        }
        h
    }

    fn update(&mut self, dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer -= dt;
        }

        // Spawn on Click
        if is_mouse_button_pressed(MouseButton::Left) {
             let r = macroquad::rand::gen_range(0.1, 0.5);
             let theta = macroquad::rand::gen_range(0.0, 2.0 * PI);
             let p = Point::from_polar(r, theta);
             // Inverse transform to place relative to player
             // Actually, entities are stored in player-centric coordinates?
             // No, in hyperbolic-hell, entities are stored in the current "center tile" frame?
             // Wait, `entity.pos` is transformed by `inv_neighbor`.
             // So entities are in the current local chart.
             self.vehicles.push(Vehicle::new(p, macroquad::rand::rand() as u64));
             self.message = "Vehicle Spawned!".to_string();
             self.message_timer = 2.0;
        }

        // Update Vehicles
        for vehicle in &mut self.vehicles {
            vehicle.update(dt);
        }

        // Remove far vehicles
        self.vehicles
            .retain(|v| hyperbolic_dist(v.pos, Point::new(0.0, 0.0)) < 4.0);

        // Input
        let mut move_vec = Vec2::new(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            move_vec.y += 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_vec.y -= 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_vec.x -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            move_vec.x += 1.0;
        }

        if move_vec.length() > 0.0 {
            move_vec = move_vec.normalize();
            let speed = 0.5 * dt;

            let angle = self.view_angle as f32;
            let rot_vec = Vec2::new(
                move_vec.x * angle.cos() - move_vec.y * angle.sin(),
                move_vec.x * angle.sin() + move_vec.y * angle.cos(),
            );

            let delta = Point::new(
                rot_vec.x as f64 * speed as f64,
                rot_vec.y as f64 * speed as f64,
            );
            let candidate_pos = mobius_add(self.player_pos, delta);

            // Wall Collision Check
            let origin_dist = hyperbolic_dist(candidate_pos, Point::new(0.0, 0.0));
            let mut best_neighbor = None;
            let mut best_dist = origin_dist;

            for i in 0..4 {
                let neighbor_center = neighbor_transform_a(i, &self.tiling);
                let dist = hyperbolic_dist(candidate_pos, neighbor_center);
                if dist < best_dist {
                    best_dist = dist;
                    best_neighbor = Some(i);
                }
            }

            let hash = self.get_path_hash();
            if let Some(idx) = best_neighbor {
                if is_wall(hash, idx) {
                    self.message = "Blocked by Wall!".to_string();
                    self.message_timer = 1.0;
                } else {
                    self.player_pos = candidate_pos;
                }
            } else {
                self.player_pos = candidate_pos;
            }
        }

        // Manual Rotation
        if is_key_down(KeyCode::Q) {
            self.view_angle += 2.0 * dt as f64;
        }
        if is_key_down(KeyCode::E) {
            self.view_angle -= 2.0 * dt as f64;
        }

        // Transition Logic
        let origin_dist = hyperbolic_dist(self.player_pos, Point::new(0.0, 0.0));
        let mut best_neighbor_data = None;
        let mut best_dist = origin_dist;

        for i in 0..4 {
            let neighbor_center = neighbor_transform_a(i, &self.tiling);
            let dist = hyperbolic_dist(self.player_pos, neighbor_center);
            if dist < best_dist {
                best_dist = dist;
                best_neighbor_data = Some((i, neighbor_center));
            }
        }

        if let Some((idx, neighbor_center)) = best_neighbor_data {
            let inv_neighbor = Mobius::inverse_translation(neighbor_center);
            let new_pos = inv_neighbor.apply(self.player_pos);

            // Rotation Correction
            let m_old = Mobius::rotation(-self.view_angle)
                .then(&Mobius::inverse_translation(self.player_pos));
            let trans_neighbor = Mobius::translation(neighbor_center);
            let trans_p_new = Mobius::translation(new_pos);

            let m_target = m_old.then(&trans_neighbor).then(&trans_p_new);
            let phase = m_target.a.arg();
            self.view_angle = -phase;

            self.player_pos = new_pos;

            // Transform Entities
            for vehicle in &mut self.vehicles {
                vehicle.pos = inv_neighbor.apply(vehicle.pos);
                // Note: heading correction is skipped for simplicity/chaos.
            }

            // Update Path
            let inverse_idx = (idx + 2) % 4;
            if let Some(&last) = self.player_path.last() {
                if last == inverse_idx {
                    self.player_path.pop();
                } else {
                    self.player_path.push(idx);
                }
            } else {
                self.player_path.push(idx);
            }

            // Spawn Entities
            let new_hash = self.get_path_hash();
            if new_hash % 5 == 0 {
                // 20% chance
                let r = macroquad::rand::gen_range(0.1, 0.5);
                let theta = macroquad::rand::gen_range(0.0, 2.0 * PI);
                let p = Point::from_polar(r, theta);
                self.vehicles.push(Vehicle::new(p, new_hash));
                self.message = "Vehicle Spawned!".to_string();
                self.message_timer = 2.0;
            }
        }
    }
}

#[macroquad::main("Hyperbolic Automaton")]
async fn main() {
    let mut state = GameState::new();

    loop {
        state.update(get_frame_time());

        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let view_transform = Mobius::rotation(-state.view_angle)
            .then(&Mobius::inverse_translation(state.player_pos));

        let incoming = if let Some(&last) = state.player_path.last() {
            Some((last + 2) % 4)
        } else {
            None
        };

        let hash = state.get_path_hash();

        // Draw World
        draw_tile(view_transform, 0, hash, &state.tiling, incoming);

        // Draw Vehicles
        let scale = screen_height().min(screen_width()) * 0.45;
        for vehicle in &state.vehicles {
            let screen_pos = to_screen(view_transform.apply(vehicle.pos), scale);

            // Draw Body
            draw_circle(screen_pos.x, screen_pos.y, 6.0, vehicle.color);

            // Draw Heading Indicator
            // We need to transform the heading vector too to show it correctly on screen.
            // Screen mapping is Euclidean, but the projection is conformal (Poincare disk).
            // So angles are preserved *locally*.
            // But we need to account for the view_transform rotation.
            // The view_transform is a Mobius map. M(z).
            // The local rotation at z is arg(M'(z)).

            // Just assume screen rotation matches "visual" rotation for now.
            // If view_angle is 0 and we are at origin, it matches.
            // view_transform includes rotation -view_angle.
            // Let's just draw a line in the direction of `vehicle.heading` relative to the screen,
            // corrected by the fact that `view_transform` rotates things.
            // Actually, simply projecting a point slightly ahead works best.

            let tip_local = mobius_add(vehicle.pos, Point::from_polar(0.05, vehicle.heading));
            let tip_screen = to_screen(view_transform.apply(tip_local), scale);

            draw_line(screen_pos.x, screen_pos.y, tip_screen.x, tip_screen.y, 2.0, WHITE);
        }

        // Draw Player (Camera Center)
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 5.0, YELLOW);

        // UI
        draw_text(&state.message, 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Depth: {}", state.player_path.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            &format!("Vehicles: {}", state.vehicles.len()),
            20.0,
            90.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
