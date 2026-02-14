use rayon::prelude::*;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 150;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Material {
    Empty,
    Wall,
    Server,
    Vent,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub heat: f32,
    // Let's use separate channels for clarity
    pub pheromone_defense: f32,
    pub pheromone_attack: f32,
    pub material: Material,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            heat: 0.0,
            pheromone_defense: 0.0,
            pheromone_attack: 0.0,
            material: Material::Empty,
        }
    }
}

pub struct Grid {
    pub cells: Vec<Cell>,
    pub next_cells: Vec<Cell>, // Double buffering for parallel updates
}

impl Grid {
    pub fn new() -> Self {
        let size = WIDTH * HEIGHT;
        Self {
            cells: vec![Cell::default(); size],
            next_cells: vec![Cell::default(); size],
        }
    }

    pub fn get_index(x: usize, y: usize) -> usize {
        y * WIDTH + x
    }

    pub fn get_coords(index: usize) -> (usize, usize) {
        (index % WIDTH, index / WIDTH)
    }

    pub fn update_diffusion(&mut self) {
        // Parallel diffusion using chunks or just parallel iterator if we can structure it right
        // For simplicity with double buffering, we can read from `cells` and write to `next_cells`.

        let cells = &self.cells;
        let next_cells = &mut self.next_cells;

        // Use rayon to iterate over next_cells in parallel
        next_cells
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, next_cell)| {
                let (x, y) = Self::get_coords(i);

                // Get current state from the read-only buffer
                let current = &cells[i];

                // Default behavior: copy current state
                *next_cell = *current;

                // Skip if it's a wall (blocks diffusion)
                if current.material == Material::Wall {
                    return;
                }

                let mut heat_sum = 0.0;
                let mut defense_sum = 0.0;
                let mut attack_sum = 0.0;
                let mut count = 0.0;

                // Check 4 neighbors
                let neighbors = [
                    (x as isize - 1, y as isize),
                    (x as isize + 1, y as isize),
                    (x as isize, y as isize - 1),
                    (x as isize, y as isize + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx >= 0 && nx < WIDTH as isize && ny >= 0 && ny < HEIGHT as isize {
                        let idx = Self::get_index(nx as usize, ny as usize);
                        let neighbor = &cells[idx];
                        if neighbor.material != Material::Wall {
                            heat_sum += neighbor.heat;
                            defense_sum += neighbor.pheromone_defense;
                            attack_sum += neighbor.pheromone_attack;
                            count += 1.0;
                        }
                    }
                }

                // Diffusion Parameters
                let diffusion_rate = 0.1;
                let evaporation_rate = 0.01;

                if count > 0.0 {
                    let avg_heat = heat_sum / count;
                    let avg_defense = defense_sum / count;
                    let avg_attack = attack_sum / count;

                    next_cell.heat =
                        current.heat * (1.0 - diffusion_rate) + avg_heat * diffusion_rate;
                    next_cell.pheromone_defense = (current.pheromone_defense
                        * (1.0 - diffusion_rate)
                        + avg_defense * diffusion_rate)
                        * (1.0 - evaporation_rate);
                    next_cell.pheromone_attack = (current.pheromone_attack
                        * (1.0 - diffusion_rate)
                        + avg_attack * diffusion_rate)
                        * (1.0 - evaporation_rate);
                } else {
                    next_cell.heat = current.heat;
                    next_cell.pheromone_defense =
                        current.pheromone_defense * (1.0 - evaporation_rate);
                    next_cell.pheromone_attack =
                        current.pheromone_attack * (1.0 - evaporation_rate);
                }

                // If server, generate heat
                if next_cell.material == Material::Server {
                    next_cell.heat = (next_cell.heat + 5.0).min(1000.0);
                }
            });

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_diffusion() {
        let mut grid = Grid::new();
        let idx = Grid::get_index(10, 10);
        grid.cells[idx].heat = 100.0;

        // Run update
        grid.update_diffusion();

        // Check neighbor (10, 11)
        let neighbor_idx = Grid::get_index(10, 11);

        // After one step, neighbor heat should be > 0.
        // neighbor (0) * 0.9 + avg(neighbors including 100) * 0.1
        // neighbor (10, 11) has neighbors: (10,10)=100, others=0.
        // avg = 100 / 4 = 25.
        // next_heat = 0 * 0.9 + 25 * 0.1 = 2.5

        assert!(
            grid.cells[neighbor_idx].heat > 0.0,
            "Heat should diffuse to neighbor"
        );
    }
}
