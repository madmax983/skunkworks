mod render;

use macroquad::prelude::*;
use poincare_disk::{Mobius, Point, TilingConsts, neighbor_transform_a, hyperbolic_dist, mobius_add, mobius_sub};
use render::{draw_tile, is_wall, to_screen};
use std::f64::consts::PI;

struct Entity {
    pos: Point,
    #[allow(dead_code)]
    seed: u64,
}

struct GameState {
    player_pos: Point,
    player_path: Vec<usize>,
    view_angle: f64,

    tiling: TilingConsts,

    entities: Vec<Entity>,

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
            entities: Vec::new(),
            message: "WASD to Move. Welcome to Hell.".to_string(),
            message_timer: 5.0,
        }
    }

    fn get_path_hash(&self) -> u64 {
        let mut h: u64 = 123456789;
        for &step in &self.player_path {
            h = h.wrapping_mul(6364136223846793005).wrapping_add(step as u64);
        }
        h
    }

    fn update(&mut self, dt: f32) {
        if self.message_timer > 0.0 {
            self.message_timer -= dt;
        }

        // Entity AI
        let player_p = self.player_pos;
        for entity in &mut self.entities {
            // Move towards player
            let dir_at_entity = mobius_sub(player_p, entity.pos);

            // Normalize
            let dist = dir_at_entity.norm();
            if dist > 0.001 {
                let speed = 0.2 * dt as f64;
                let step = Point::new(
                    dir_at_entity.re * speed / dist,
                    dir_at_entity.im * speed / dist
                );
                entity.pos = mobius_add(entity.pos, step);
            }
        }

        // Remove far entities
        self.entities.retain(|e| hyperbolic_dist(e.pos, Point::new(0.0, 0.0)) < 4.0);

        // Input
        let mut move_vec = Vec2::new(0.0, 0.0);
        if is_key_down(KeyCode::W) { move_vec.y += 1.0; }
        if is_key_down(KeyCode::S) { move_vec.y -= 1.0; }
        if is_key_down(KeyCode::A) { move_vec.x -= 1.0; }
        if is_key_down(KeyCode::D) { move_vec.x += 1.0; }

        if move_vec.length() > 0.0 {
            move_vec = move_vec.normalize();
            let speed = 0.5 * dt;

            let angle = self.view_angle as f32;
            let rot_vec = Vec2::new(
                move_vec.x * angle.cos() - move_vec.y * angle.sin(),
                move_vec.x * angle.sin() + move_vec.y * angle.cos()
            );

            let delta = Point::new(rot_vec.x as f64 * speed as f64, rot_vec.y as f64 * speed as f64);
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
        if is_key_down(KeyCode::Q) { self.view_angle += 2.0 * dt as f64; }
        if is_key_down(KeyCode::E) { self.view_angle -= 2.0 * dt as f64; }

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
            let m_old = Mobius::rotation(-self.view_angle).then(&Mobius::inverse_translation(self.player_pos));
            let trans_neighbor = Mobius::translation(neighbor_center);
            let trans_p_new = Mobius::translation(new_pos);

            let m_target = m_old.then(&trans_neighbor).then(&trans_p_new);
            let phase = m_target.a.arg();
            self.view_angle = -phase;

            self.player_pos = new_pos;

            // Transform Entities
            for entity in &mut self.entities {
                entity.pos = inv_neighbor.apply(entity.pos);
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
            if new_hash % 5 == 0 { // 20% chance
                 let r = macroquad::rand::gen_range(0.1, 0.5);
                 let theta = macroquad::rand::gen_range(0.0, 2.0 * PI);
                 let p = Point::from_polar(r, theta);
                 self.entities.push(Entity { pos: p, seed: new_hash });
                 self.message = "Entity Spawned!".to_string();
                 self.message_timer = 2.0;
            }
        }
    }
}

#[macroquad::main("Hyperbolic Hell")]
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

        // Draw Entities
        let scale = screen_height().min(screen_width()) * 0.45;
        for entity in &state.entities {
            let screen_pos = to_screen(view_transform.apply(entity.pos), scale);
            draw_circle(screen_pos.x, screen_pos.y, 4.0, GREEN);
        }

        // Draw Player
        draw_circle(screen_width()/2.0, screen_height()/2.0, 5.0, YELLOW);

        // UI
        draw_text(&state.message, 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Depth: {}", state.player_path.len()), 20.0, 60.0, 20.0, GRAY);
        draw_text(&format!("Entities: {}", state.entities.len()), 20.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}
