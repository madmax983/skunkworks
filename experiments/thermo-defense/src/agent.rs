use crate::grid::{Grid, HEIGHT, Material, WIDTH};
use glam::Vec2;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AgentType {
    Termite,
    Locust,
}

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub kind: AgentType,
    pub carrying: bool,
    pub health: f32,
}

#[derive(Debug)]
pub enum GridAction {
    UpdateCell {
        idx: usize,
        heat_delta: f32,
        defense_delta: f32,
        attack_delta: f32,
        new_material: Option<Material>,
    },
}

impl Agent {
    pub fn new(x: f32, y: f32, kind: AgentType) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::ZERO,
            kind,
            carrying: false,
            health: 100.0,
        }
    }

    pub fn update(&mut self, grid: &Grid, rng: &mut impl Rng) -> Option<GridAction> {
        let (x, y) = (self.position.x as usize, self.position.y as usize);
        if x >= WIDTH || y >= HEIGHT {
            return None;
        }
        let idx = Grid::get_index(x, y);

        let action = match self.kind {
            AgentType::Termite => self.update_termite(grid, rng, idx, x, y),
            AgentType::Locust => self.update_locust(grid, rng, idx, x, y),
        };

        // Movement (common)
        let speed = 0.5;
        self.velocity += Vec2::new(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));

        if self.velocity.length() > speed {
            self.velocity = self.velocity.normalize() * speed;
        }

        self.position += self.velocity;

        if self.position.x < 0.0 || self.position.x >= WIDTH as f32 {
            self.velocity.x *= -1.0;
            self.position.x = self.position.x.clamp(0.0, WIDTH as f32 - 0.1);
        }
        if self.position.y < 0.0 || self.position.y >= HEIGHT as f32 {
            self.velocity.y *= -1.0;
            self.position.y = self.position.y.clamp(0.0, HEIGHT as f32 - 0.1);
        }

        action
    }

    fn update_termite(
        &mut self,
        grid: &Grid,
        rng: &mut impl Rng,
        idx: usize,
        x: usize,
        y: usize,
    ) -> Option<GridAction> {
        // 1. Gather Info (Immutable)
        let (heat, defense, material) = {
            let c = &grid.cells[idx];
            (c.heat, c.pheromone_defense, c.material)
        };

        let mut neighbors = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                    let nidx = Grid::get_index(nx as usize, ny as usize);
                    if grid.cells[nidx].material == Material::Wall {
                        neighbors += 1;
                    }
                }
            }
        }

        let defense_delta = 1.0;
        let mut new_material = None;

        if self.carrying {
            let should_drop = if heat > 50.0 {
                rng.gen_bool(0.1)
            } else if neighbors > 0 && neighbors < 6 {
                rng.gen_bool(0.05)
            } else if defense > 50.0 {
                rng.gen_bool(0.1)
            } else {
                rng.gen_bool(0.001)
            };

            if should_drop && material == Material::Empty {
                new_material = Some(Material::Wall);
                self.carrying = false;
            }
        } else {
            if material == Material::Wall {
                let should_pickup = if heat > 80.0 {
                    rng.gen_bool(0.1)
                } else if neighbors <= 1 {
                    rng.gen_bool(0.5)
                } else {
                    rng.gen_bool(0.001)
                };

                if should_pickup {
                    new_material = Some(Material::Empty);
                    self.carrying = true;
                }
            }
        }

        Some(GridAction::UpdateCell {
            idx,
            heat_delta: 0.0,
            defense_delta,
            attack_delta: 0.0,
            new_material,
        })
    }

    fn update_locust(
        &mut self,
        grid: &Grid,
        rng: &mut impl Rng,
        idx: usize,
        x: usize,
        y: usize,
    ) -> Option<GridAction> {
        let (current_heat, material) = {
            let c = &grid.cells[idx];
            (c.heat, c.material)
        };

        let mut best_dir = Vec2::ZERO;
        let mut max_heat = -1.0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                    let nidx = Grid::get_index(nx as usize, ny as usize);
                    let n_heat = grid.cells[nidx].heat;
                    if n_heat > max_heat {
                        max_heat = n_heat;
                        best_dir = Vec2::new(dx as f32, dy as f32);
                    }
                }
            }
        }

        if max_heat > current_heat {
            self.velocity += best_dir * 0.2;
        }

        let mut heat_delta = 0.0;
        let mut new_material = None;
        let attack_delta = 1.0;

        if material == Material::Server {
            heat_delta = 10.0;
        } else if material == Material::Wall {
            if rng.gen_bool(0.1) {
                new_material = Some(Material::Empty);
            }
        }

        Some(GridAction::UpdateCell {
            idx,
            heat_delta,
            defense_delta: 0.0,
            attack_delta,
            new_material,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_termite_pickup() {
        let mut grid = Grid::new();
        let idx = Grid::get_index(10, 10);
        grid.cells[idx].material = Material::Wall; // Place a wall

        let mut agent = Agent::new(10.0, 10.0, AgentType::Termite);
        let mut rng = rand::thread_rng();

        // Run multiple times to overcome probability
        for _ in 0..100 {
            if let Some(GridAction::UpdateCell {
                new_material: Some(m),
                ..
            }) = agent.update(&grid, &mut rng)
            {
                if m == Material::Empty {
                    // Applied!
                    grid.cells[idx].material = m;
                }
            }
            if agent.carrying {
                break;
            }
            // Reset position if it moved away
            agent.position = Vec2::new(10.0, 10.0);
        }

        assert!(agent.carrying, "Agent should have picked up the wall");
        assert_eq!(
            grid.cells[idx].material,
            Material::Empty,
            "Wall should be gone"
        );
    }
}
