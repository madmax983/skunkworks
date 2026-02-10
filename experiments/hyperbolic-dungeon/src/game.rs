use crate::dungeon::{Dungeon, TileType};
use crate::entity::{Entity, EntityKind};
use poincare_disk::{mobius_add, neighbor_transform_a, Mobius, Point, TilingConsts};

pub struct Game {
    pub dungeon: Dungeon,
    pub player_id: usize,
    pub entities: Vec<Entity>,
    pub tiling_consts: TilingConsts,
    pub message: String,
    pub next_entity_id: usize,
}

impl Game {
    pub fn new() -> Self {
        let dungeon = Dungeon::new(12345);
        let tiling_consts = TilingConsts::new_4_5();

        let mut entities = Vec::new();
        // Player is ID 0
        let player = Entity::new(0, vec![], EntityKind::Player);
        entities.push(player);

        // Add an enemy at [0]
        let enemy = Entity::new(1, vec![0], EntityKind::Enemy);
        entities.push(enemy);

        Self {
            dungeon,
            player_id: 0,
            entities,
            tiling_consts,
            message: "Welcome to the Hyperbolic Dungeon!".to_string(),
            next_entity_id: 2,
        }
    }

    pub fn get_player(&self) -> &Entity {
        self.entities
            .iter()
            .find(|e| e.id == self.player_id)
            .expect("Player not found")
    }

    pub fn get_player_mut(&mut self) -> &mut Entity {
        self.entities
            .iter_mut()
            .find(|e| e.id == self.player_id)
            .expect("Player not found")
    }

    pub fn update(&mut self) {
        self.update_enemies();
    }

    pub fn update_enemies(&mut self) {
        let player_path = self.get_player().path.clone();
        let player_offset = self.get_player().offset;
        let player_id = self.player_id;

        // Collect updates to avoid borrowing self while iterating
        let mut updates: Vec<(usize, Point, Vec<usize>)> = Vec::new();
        let mut attacks: Vec<(usize, usize)> = Vec::new(); // attacker, victim

        for entity in &self.entities {
            if entity.kind != EntityKind::Enemy {
                continue;
            }

            // Simple AI: Move towards player
            // 1. Determine direction
            let next_step = Self::get_next_step(&entity.path, &player_path);

            let target_point_in_local = if let Some(dir) = next_step {
                neighbor_transform_a(dir, &self.tiling_consts)
            } else {
                player_offset
            };

            // 2. Move
            let speed: f64 = 0.02;

            let t_inv = Mobius::inverse_translation(entity.offset);
            let target_at_origin = t_inv.apply(target_point_in_local);

            let dist = target_at_origin.norm();

            if dist < 0.05 && next_step.is_none() {
                attacks.push((entity.id, player_id));
                continue;
            }

            let move_dist = speed.min(dist);
            if move_dist < 1e-4 {
                continue;
            }

            let direction = target_at_origin / dist; // Unit complex
            let step = direction * move_dist;

            let new_pos_local = Mobius::translation(entity.offset).apply(step);
            updates.push((entity.id, new_pos_local, entity.path.clone()));
        }

        // Apply updates
        for (id, new_pos, current_path) in updates {
            let mut best_neighbor = None;
            let mut best_dist_sq = new_pos.norm_sqr();

            for i in 0..4 {
                let neighbor_pos = neighbor_transform_a(i, &self.tiling_consts);
                let t = Mobius::inverse_translation(neighbor_pos);
                let p_in_neighbor = t.apply(new_pos);
                let dist_sq = p_in_neighbor.norm_sqr();
                if dist_sq < best_dist_sq {
                    best_dist_sq = dist_sq;
                    best_neighbor = Some((i, p_in_neighbor));
                }
            }

            if let Some((idx, neighbor_pos)) = best_neighbor {
                let next_path = Dungeon::canonicalize_step(current_path.clone(), idx);
                if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
                    e.path = next_path;
                    e.offset = neighbor_pos;
                }
            } else {
                if let Some(e) = self.entities.iter_mut().find(|e| e.id == id) {
                    e.offset = new_pos;
                }
            }
        }

        if !attacks.is_empty() {
            self.message = "Attacked by Enemy!".to_string();
        }
    }

    fn get_next_step(start: &[usize], end: &[usize]) -> Option<usize> {
        if start == end {
            return None;
        }

        if end.starts_with(start) {
            return Some(end[start.len()]);
        }

        if let Some(&last) = start.last() {
            return Some((last + 2) % 4);
        }

        None
    }

    pub fn move_player(&mut self, dx: f64, dy: f64) {
        let delta = Point::new(dx, dy);
        if delta.norm() > 0.2 {
            return;
        }

        let (current_path, current_offset) = {
            let player = self.get_player();
            (player.path.clone(), player.offset)
        };

        let candidate_offset = mobius_add(current_offset, delta);

        let mut best_neighbor = None;
        let mut best_dist_sq = candidate_offset.norm_sqr();

        for i in 0..4 {
            let neighbor_pos = neighbor_transform_a(i, &self.tiling_consts);
            let t = Mobius::inverse_translation(neighbor_pos);
            let p_in_neighbor = t.apply(candidate_offset);

            let dist_sq = p_in_neighbor.norm_sqr();
            if dist_sq < best_dist_sq {
                best_dist_sq = dist_sq;
                best_neighbor = Some((i, p_in_neighbor));
            }
        }

        if let Some((idx, new_pos)) = best_neighbor {
            let next_path = Dungeon::canonicalize_step(current_path.clone(), idx);

            let (is_wall, tile_seed, tile_type) = {
                let tile = self.dungeon.get_tile(&next_path);
                (
                    matches!(tile.tile_type, TileType::Wall),
                    tile.color_seed,
                    tile.tile_type,
                )
            };

            if is_wall {
                self.message = "Blocked by Wall!".to_string();
                return;
            }

            if let Some(player) = self.entities.iter_mut().find(|e| e.id == self.player_id) {
                player.path = next_path.clone();
                player.offset = new_pos;
            }

            self.dungeon.mark_visited(&next_path);

            self.message = format!("Entered {:?} (Seed: {})", tile_type, tile_seed);
        } else {
            if let Some(player) = self.entities.iter_mut().find(|e| e.id == self.player_id) {
                player.offset = candidate_offset;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_player_basic() {
        let mut game = Game::new();
        let initial_offset = game.get_player().offset;

        // Move right
        game.move_player(0.05, 0.0);

        let new_offset = game.get_player().offset;

        // Should have moved
        // Compare values explicitly
        assert!(new_offset.re > initial_offset.re);
    }
}
