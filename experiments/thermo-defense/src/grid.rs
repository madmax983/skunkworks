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

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
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
        // Parallel diffusion using chunks (rows) to avoid div/mod per cell and optimize access
        let cells = &self.cells;
        let next_cells = &mut self.next_cells;
        let width = WIDTH;
        let height = HEIGHT;

        // Use rayon to iterate over rows in parallel
        next_cells
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, next_cell) in row.iter_mut().enumerate() {
                    let i = y * width + x;

                    // Get current state from the read-only buffer
                    let current = &cells[i];

                    // Default behavior: copy current state
                    *next_cell = *current;

                    // Skip if it's a wall (blocks diffusion)
                    if current.material == Material::Wall {
                        continue;
                    }

                    let mut heat_sum = 0.0;
                    let mut defense_sum = 0.0;
                    let mut attack_sum = 0.0;
                    let mut count = 0.0;

                    // Neighbors logic without div/mod
                    // We directly calculate indices based on x and y

                    // Left
                    if x > 0 {
                        let idx = i - 1;
                        let neighbor = &cells[idx];
                        if neighbor.material != Material::Wall {
                            heat_sum += neighbor.heat;
                            defense_sum += neighbor.pheromone_defense;
                            attack_sum += neighbor.pheromone_attack;
                            count += 1.0;
                        }
                    }

                    // Right
                    if x < width - 1 {
                        let idx = i + 1;
                        let neighbor = &cells[idx];
                        if neighbor.material != Material::Wall {
                            heat_sum += neighbor.heat;
                            defense_sum += neighbor.pheromone_defense;
                            attack_sum += neighbor.pheromone_attack;
                            count += 1.0;
                        }
                    }

                    // Up
                    if y > 0 {
                        let idx = i - width;
                        let neighbor = &cells[idx];
                        if neighbor.material != Material::Wall {
                            heat_sum += neighbor.heat;
                            defense_sum += neighbor.pheromone_defense;
                            attack_sum += neighbor.pheromone_attack;
                            count += 1.0;
                        }
                    }

                    // Down
                    if y < height - 1 {
                        let idx = i + width;
                        let neighbor = &cells[idx];
                        if neighbor.material != Material::Wall {
                            heat_sum += neighbor.heat;
                            defense_sum += neighbor.pheromone_defense;
                            attack_sum += neighbor.pheromone_attack;
                            count += 1.0;
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
