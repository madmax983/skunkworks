use crate::grid::Grid;
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AgentKind {
    Red,  // Chaos seeker (increases r)
    Blue, // Order seeker (decreases r)
}

pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub kind: AgentKind,
    pub energy: f32,
}

impl Agent {
    pub fn new(x: f32, y: f32, kind: AgentKind) -> Self {
        Self {
            x,
            y,
            kind,
            energy: 1.0,
        }
    }

    pub fn update(&mut self, grid: &mut Grid) {
        let width = grid.width as f32;
        let height = grid.height as f32;
        let mut rng = rand::thread_rng();

        // 1. Check local environment
        let ix = self.x as isize;
        let iy = self.y as isize;
        let idx = grid.get_idx(ix, iy);
        let r = grid.params_r[idx];

        // 2. Metabolism & Energy Gain
        self.energy -= 0.005; // Base cost

        let fitness = match self.kind {
            AgentKind::Red => {
                // Thrives in chaos (r > 3.5)
                if r > 3.5 {
                    0.02
                } else if r > 3.0 {
                    0.005
                } else {
                    -0.01
                }
            }
            AgentKind::Blue => {
                // Thrives in order (r < 3.0)
                if r < 3.0 {
                    0.02
                } else if r < 3.5 {
                    0.005
                } else {
                    -0.01
                }
            }
        };
        self.energy += fitness;
        self.energy = self.energy.clamp(0.0, 2.0);

        if self.energy <= 0.0 {
            return; // Dead (handled by caller removing)
        }

        // 3. Terraforming
        // Modify local r
        let strength = 0.01;
        match self.kind {
            AgentKind::Red => {
                grid.params_r[idx] = (grid.params_r[idx] + strength).min(4.0);
            }
            AgentKind::Blue => {
                grid.params_r[idx] = (grid.params_r[idx] - strength).max(2.0);
            }
        }

        // 4. Movement (Sensing)
        // Look at neighbors and pick best r
        let mut best_dx = 0.0;
        let mut best_dy = 0.0;
        let mut best_val = -1000.0;

        // Random jitter
        if rng.gen_bool(0.1) {
            best_dx = rng.gen_range(-1.0..=1.0);
            best_dy = rng.gen_range(-1.0..=1.0);
        } else {
            // Check 3x3 neighborhood
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let check_x = ix + dx as isize;
                    let check_y = iy + dy as isize;
                    let n_idx = grid.get_idx(check_x, check_y);
                    let nr = grid.params_r[n_idx];

                    let score = match self.kind {
                        AgentKind::Red => nr,   // Higher r is better
                        AgentKind::Blue => -nr, // Lower r is better
                    };

                    // Add some noise to decision
                    let noisy_score = score + rng.gen_range(-0.1..0.1);

                    if noisy_score > best_val {
                        best_val = noisy_score;
                        best_dx = dx as f32;
                        best_dy = dy as f32;
                    }
                }
            }
        }

        // Move
        let speed = 0.5;
        self.x = (self.x + best_dx * speed + width) % width;
        self.y = (self.y + best_dy * speed + height) % height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_terraforming() {
        let mut grid = Grid::new(10, 10);
        let center_idx = grid.get_idx(5, 5);
        grid.params_r[center_idx] = 3.0;

        let mut agent = Agent::new(5.0, 5.0, AgentKind::Red);

        // Update agent multiple times
        for _ in 0..10 {
            agent.update(&mut grid);
            // Agent might move, but initially it's at 5,5.
            // Let's force it back to 5,5 to test terraforming accumulation
            agent.x = 5.0;
            agent.y = 5.0;
        }

        // Red agent should increase r
        assert!(grid.params_r[center_idx] > 3.0);
    }
}
