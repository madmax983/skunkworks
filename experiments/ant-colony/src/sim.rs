use crate::scan::FileNode;
use rand::Rng;

#[derive(Clone)]
pub struct Ant {
    pub x: usize,
    pub y: usize,
    pub has_food: bool,
}

#[derive(Clone)]
pub struct Cell {
    pub file: Option<FileNode>,
    pub food_pheromone: f32, // Trails leading to food
    pub home_pheromone: f32, // Trails leading home
    #[allow(dead_code)]
    pub visited_count: usize,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub ants: Vec<Ant>,
    pub total_food_collected: usize,
}

impl World {
    pub fn new(files: Vec<FileNode>) -> Self {
        // Calculate grid size
        let count = files.len();
        // Ensure at least 10x10
        let side = (count as f64).sqrt().ceil() as usize;
        let width = side.max(10);
        let height = side.max(10);

        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            cells.push(Cell {
                file: None,
                food_pheromone: 0.0,
                home_pheromone: 0.0,
                visited_count: 0,
            });
        }

        // Populate cells with files
        // We start from (1,1) to leave (0,0) as Nest?
        // Or just fill them sequentially.
        for (i, file) in files.into_iter().enumerate() {
            if i < cells.len() {
                cells[i].file = Some(file);
            }
        }

        // Spawn ants at (0,0)
        let ant_count = 50;
        let mut ants = Vec::with_capacity(ant_count);
        for _ in 0..ant_count {
            ants.push(Ant {
                x: 0,
                y: 0,
                has_food: false,
            });
        }

        // Set high home pheromone at 0,0
        cells[0].home_pheromone = 1000.0;

        Self {
            width,
            height,
            cells,
            ants,
            total_food_collected: 0,
        }
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Evaporate Pheromones
        for cell in &mut self.cells {
            cell.food_pheromone *= 0.98;
            cell.home_pheromone *= 0.98;

            // Clamp
            if cell.food_pheromone < 0.01 {
                cell.food_pheromone = 0.0;
            }
            if cell.home_pheromone < 0.01 {
                cell.home_pheromone = 0.0;
            }
        }

        // Keep Nest Home Pheromone strong
        self.cells[0].home_pheromone = 1000.0;

        // 2. Move Ants
        // We need to mutate ants, and read/write cells.
        // To avoid borrow checker hell, we can iterate indices or collect moves.

        for i in 0..self.ants.len() {
            let ant = &mut self.ants[i];
            let x = ant.x;
            let y = ant.y;

            // Drop Pheromones
            let idx = y * self.width + x;
            if ant.has_food {
                // Leaving trail to food (dropping food pheromone)
                // Actually, if I have food, I am marking the path BACK from food.
                // So I drop "Food Pheromone" so others can find the food source.
                self.cells[idx].food_pheromone += 10.0;
            } else {
                // Leaving trail from home (dropping home pheromone)
                // So I drop "Home Pheromone" so I can find my way back.
                self.cells[idx].home_pheromone += 10.0;
            }

            // Pick up / Drop Food Logic
            if let Some(file) = &mut self.cells[idx].file {
                if !ant.has_food && file.todo_count > 0 {
                    // Take food
                    file.todo_count -= 1;
                    ant.has_food = true;
                    // Flip direction? No, just look for home pheromone next step.
                }
            }

            // Drop food at nest (0,0)
            if ant.has_food && x == 0 && y == 0 {
                ant.has_food = false;
                self.total_food_collected += 1;
            }

            // Decide Move
            // Neighbors
            let neighbors = [
                (x.wrapping_sub(1), y), // Left
                (x + 1, y),             // Right
                (x, y.wrapping_sub(1)), // Up
                (x, y + 1),             // Down
            ];

            let mut best_move = (x, y);
            let mut max_score = -1.0;

            // Randomness factor to prevent getting stuck
            let noise = 0.5;

            for (nx, ny) in neighbors {
                if nx >= self.width || ny >= self.height {
                    continue;
                }

                let n_idx = ny * self.width + nx;
                let cell = &self.cells[n_idx];

                // Score depends on what we are looking for
                let score = if ant.has_food {
                    // Look for Home
                    cell.home_pheromone
                } else {
                    // Look for Food
                    // Also favor cells with actual food (TODOs) if we can smell them (adjacent)
                    let food_bonus = if let Some(f) = &cell.file {
                        if f.todo_count > 0 {
                            100.0
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    };
                    cell.food_pheromone + food_bonus
                };

                let random_score = score + rng.gen_range(0.0..noise);

                if random_score > max_score {
                    max_score = random_score;
                    best_move = (nx, ny);
                }
            }

            // Move
            ant.x = best_move.0;
            ant.y = best_move.1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_world_init() {
        let files = vec![
            FileNode {
                path: PathBuf::from("a.rs"),
                todo_count: 5,
                line_count: 10,
            },
            FileNode {
                path: PathBuf::from("b.rs"),
                todo_count: 0,
                line_count: 20,
            },
        ];

        let world = World::new(files);
        assert!(world.width >= 10);
        assert!(world.cells.len() >= 100);
        // We can't guarantee order if I used a map, but I used a vec.
        // And I assigned sequentially.
        assert_eq!(
            world.cells[0].file.as_ref().unwrap().path.to_str().unwrap(),
            "a.rs"
        );
    }
}
