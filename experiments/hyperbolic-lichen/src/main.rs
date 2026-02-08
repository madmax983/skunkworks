mod render;
mod lichen;

use macroquad::prelude::*;
use poincare_disk::{
    mobius_add, neighbor_transform_a, hyperbolic_dist, Mobius, Point, TilingConsts,
};
use render::{draw_tile_recursive};
use lichen::{LichenState};

struct GameState {
    player_pos: Point,
    player_path: Vec<usize>,
    view_angle: f64,
    tiling: TilingConsts,
    lichen: LichenState,
    message_timer: f32,
}

impl GameState {
    fn new() -> Self {
        Self {
            player_pos: Point::new(0.0, 0.0),
            player_path: Vec::new(),
            view_angle: 0.0,
            tiling: TilingConsts::new_4_5(),
            lichen: LichenState::new(),
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

        // Lichen Simulation Update
        self.lichen.update();

        // Input & Movement
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
                move_vec.x * angle.sin() + move_vec.y * angle.cos(),
            );

            let delta = Point::new(
                rot_vec.x as f64 * speed as f64,
                rot_vec.y as f64 * speed as f64,
            );
            let candidate_pos = mobius_add(self.player_pos, delta);

            let mut best_neighbor = None;
            let mut best_dist = hyperbolic_dist(candidate_pos, Point::new(0.0, 0.0));

            for i in 0..4 {
                let neighbor_center = neighbor_transform_a(i, &self.tiling);
                let dist = hyperbolic_dist(candidate_pos, neighbor_center);
                if dist < best_dist {
                    best_dist = dist;
                    best_neighbor = Some((i, neighbor_center));
                }
            }

            if let Some((idx, neighbor_center)) = best_neighbor {
                 // Transition!
                let inv_neighbor = Mobius::inverse_translation(neighbor_center);
                let new_pos = inv_neighbor.apply(candidate_pos);

                // Rotation Correction
                let m_old = Mobius::rotation(-self.view_angle)
                    .then(&Mobius::inverse_translation(self.player_pos));
                let trans_neighbor = Mobius::translation(neighbor_center);
                let trans_p_new = Mobius::translation(new_pos);

                let m_target = m_old.then(&trans_neighbor).then(&trans_p_new);
                let phase = m_target.a.arg();
                self.view_angle = -phase;

                self.player_pos = new_pos;

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
    }
}

#[macroquad::main("Hyperbolic Lichen")]
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
        draw_tile_recursive(
            view_transform,
            0,
            hash,
            None,
            &state.tiling,
            incoming,
            &mut state.lichen
        );

        // Draw Player
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 5.0, YELLOW);

        // UI
        draw_text("HYPERBOLIC LICHEN", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Depth: {}", state.player_path.len()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        let active_cells = state.lichen.cells.len();
        draw_text(
            &format!("Active Cells: {}", active_cells),
            20.0,
            90.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
