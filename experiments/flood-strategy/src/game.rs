use crate::fluid::FluidSim;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Unit {
    pub x: usize,
    pub y: usize,
    pub goal_x: usize,
    pub goal_y: usize,
    pub alive: bool,
}

pub struct Game {
    pub fluid: FluidSim,
    pub units: Vec<Unit>,
    pub width: usize,
    pub height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            fluid: FluidSim::new(width, height),
            units: Vec::new(),
            width,
            height,
        }
    }

    pub fn spawn_unit(&mut self, x: usize, y: usize, goal_x: usize, goal_y: usize) {
        self.units.push(Unit {
            x,
            y,
            goal_x,
            goal_y,
            alive: true,
        });
    }

    pub fn update_units(&mut self) {
        for unit in &mut self.units {
            if !unit.alive {
                continue;
            }

            // Check if drowned
            let idx = self.fluid.index(unit.x, unit.y);
            if self.fluid.water[idx] > 0.5 {
                unit.alive = false;
                continue;
            }

            // Reached goal?
            if unit.x == unit.goal_x && unit.y == unit.goal_y {
                continue; // Safe at goal
            }

            // Move Logic
            // Simple Greedy Best First Search (local)
            let mut best_score = f32::MAX;
            let mut best_move = (unit.x, unit.y);

            let directions = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, 0)]; // N, S, W, E, Stay

            for (dx, dy) in directions.iter() {
                let nx = unit.x as i32 + dx;
                let ny = unit.y as i32 + dy;

                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    let n_idx = self.fluid.index(nx, ny);

                    // Avoid deep water
                    let water_depth = self.fluid.water[n_idx];
                    if water_depth > 0.3 {
                        continue; // Too deep to step into
                    }

                    // Heuristic: Distance to goal
                    let dist = ((nx as f32 - unit.goal_x as f32).powi(2) + (ny as f32 - unit.goal_y as f32).powi(2)).sqrt();

                    // Penalty for water
                    let score = dist + water_depth * 10.0;

                    if score < best_score {
                        best_score = score;
                        best_move = (nx, ny);
                    }
                }
            }

            unit.x = best_move.0;
            unit.y = best_move.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_movement() {
        let mut game = Game::new(10, 10);
        game.spawn_unit(0, 0, 5, 0); // Start (0,0), Goal (5,0)

        game.update_units();

        let unit = &game.units[0];
        assert_eq!(unit.x, 1); // Should move towards goal
        assert_eq!(unit.y, 0);
    }

    #[test]
    fn test_unit_drowning() {
        let mut game = Game::new(10, 10);
        game.spawn_unit(5, 5, 9, 9);

        // Add water at unit pos
        let idx = game.fluid.index(5, 5);
        game.fluid.water[idx] = 1.0;

        game.update_units();

        assert!(!game.units[0].alive);
    }

    #[test]
    fn test_unit_avoids_water() {
        let mut game = Game::new(10, 10);
        game.spawn_unit(0, 0, 2, 0);

        // Put water at (1,0)
        let idx = game.fluid.index(1, 0);
        game.fluid.water[idx] = 0.4; // 0.4 is > 0.3 threshold

        game.update_units();

        let unit = &game.units[0];
        // Should not move to (1,0). Might move to (0,1) if that's better than stay?
        // (0,1) dist to (2,0) is sqrt(4+1)=2.23
        // (0,0) dist to (2,0) is 2.0
        // If (1,0) is blocked, staying is better than moving away?
        // Or going around? (0,1) -> (1,1) -> (2,1) -> (2,0)?

        assert!(unit.x != 1 || unit.y != 0);
    }
}
