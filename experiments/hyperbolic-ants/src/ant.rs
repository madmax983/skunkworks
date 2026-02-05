use crate::dungeon::{Dungeon, TileType};
use poincare_disk::{mobius_add, neighbor_transform_a, Mobius, Point, TilingConsts};
use rand::Rng;

pub struct Ant {
    pub path: Vec<usize>,
    pub offset: Point,
    pub carrying_food: bool,
    // Movement state
    target_dir: usize, // Direction index we are currently moving towards
}

impl Ant {
    pub fn new(rng: &mut impl Rng) -> Self {
        Self {
            path: Vec::new(),
            offset: Point::new(0.0, 0.0),
            carrying_food: false,
            target_dir: rng.gen_range(0..4),
        }
    }

    pub fn update(&mut self, dungeon: &Dungeon, consts: &TilingConsts, rng: &mut impl Rng) {
        // 1. Logic: Interact with current tile
        {
            let tile = dungeon.get_tile(&self.path);
            if matches!(tile.tile_type, TileType::Wall) {
                // Stuck in wall? Should not happen if collision works.
                // Respawn at nest if stuck
                self.path.clear();
                self.offset = Point::new(0.0, 0.0);
                self.carrying_food = false;
                return;
            }
        }

        // Pick up food
        if !self.carrying_food {
            if dungeon.try_pick_food(&self.path) {
                self.carrying_food = true;
                // Turn around to go home?
                // Home is "parent" of current node.
                // If path is [0, 1], parent is via opposite of 1 (3).
                // Just let the navigation logic handle "Home" direction.
            }
        } else {
            // Drop food if at nest (empty path)
            if self.path.is_empty() {
                self.carrying_food = false;
                // Successfully foraged!
                // Maybe increase nest score?
            } else {
                // Deposit pheromone while carrying food
                dungeon.deposit_pheromone(&self.path, 1.0);
            }
        }

        // 2. Navigation: Choose direction
        // We update target_dir occasionally or if we reached center?
        // Let's say we just move towards target_dir.

        // If close to center, pick new target?
        if self.offset.norm() < 0.1 {
            self.choose_new_target(dungeon, rng);
        }

        // 3. Movement Physics
        let speed = 0.05;
        let move_vec = neighbor_transform_a(self.target_dir, consts);
        // Normalize move_vec is tricky in hyperbolic, but roughly:
        let dir_vec = move_vec / move_vec.norm();

        // Apply movement
        let delta = dir_vec * speed;
        let candidate_offset = mobius_add(self.offset, delta);

        // 4. Check for transition
        let mut best_neighbor = None;
        let mut best_dist_sq = candidate_offset.norm_sqr();

        for i in 0..4 {
            let neighbor_pos = neighbor_transform_a(i, consts);
            let t = Mobius::inverse_translation(neighbor_pos);
            let p_in_neighbor = t.apply(candidate_offset);

            if p_in_neighbor.norm_sqr() < best_dist_sq {
                best_dist_sq = p_in_neighbor.norm_sqr();
                best_neighbor = Some((i, p_in_neighbor));
            }
        }

        if let Some((idx, new_pos)) = best_neighbor {
            let next_path = Dungeon::canonicalize_step(self.path.clone(), idx);
            let tile = dungeon.get_tile(&next_path);

            if matches!(tile.tile_type, TileType::Wall) {
                // Bounce
                self.target_dir = (self.target_dir + 2) % 4; // Turn back
                                                             // Or random turn
                                                             // self.target_dir = rng.gen_range(0..4);
            } else {
                // Transition
                self.path = next_path;
                self.offset = new_pos;

                // When entering new tile, incoming direction is opposite of idx.
                // We want to continue moving "forward" relative to where we came from?
                // Or just pick new target?
                // If we were moving towards neighbor idx, now we are IN neighbor idx.
                // We entered from (idx + 2) % 4.
                // We should pick a new target != entry direction to avoid backtracking immediately.

                self.choose_new_target(dungeon, rng);
            }
        } else {
            self.offset = candidate_offset;
        }
    }

    fn choose_new_target(&mut self, dungeon: &Dungeon, rng: &mut impl Rng) {
        // If carrying food, head Home (towards parent)
        if self.carrying_food {
            if let Some(&last_step) = self.path.last() {
                // Parent is opposite of last step
                self.target_dir = (last_step + 2) % 4;
                return;
            } else {
                // At nest. Pick random.
                self.target_dir = rng.gen_range(0..4);
                return;
            }
        }

        // If foraging: Weighted random choice based on pheromones
        // We want to go where there IS pheromone (trail to food).

        // Check all 4 neighbors
        let mut weights = [1.0; 4]; // Base weight for exploration
        let current_parent_dir = if let Some(&last) = self.path.last() {
            Some((last + 2) % 4)
        } else {
            None
        };

        for i in 0..4 {
            // Don't go back to parent immediately unless dead end (wall)
            if let Some(parent) = current_parent_dir {
                if i == parent {
                    weights[i] = 0.1; // Discourage backtracking
                }
            }

            let next_path = Dungeon::canonicalize_step(self.path.clone(), i);
            let tile = dungeon.get_tile(&next_path);

            if matches!(tile.tile_type, TileType::Wall) {
                weights[i] = 0.0;
            } else {
                // Add weight based on pheromone
                // Logarithmic response? Or linear?
                weights[i] += tile.pheromone_food * 5.0;
            }
        }

        // Weighted sample
        let total_weight: f64 = weights.iter().sum();
        if total_weight <= 0.001 {
            // All walls? Or trapped?
            self.target_dir = (self.target_dir + 2) % 4; // Turn around
            return;
        }

        let mut r = rng.gen::<f64>() * total_weight;
        for (i, &w) in weights.iter().enumerate() {
            r -= w;
            if r <= 0.0 {
                self.target_dir = i;
                return;
            }
        }
        self.target_dir = 3; // Fallback
    }
}
