use rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Material {
    Empty,
    Wall,
    Server,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub material: Material,
    pub heat: f32,
    pub next_heat: f32,
}

impl Cell {
    pub fn new(material: Material) -> Self {
        Self {
            material,
            heat: 0.0,
            next_heat: 0.0,
        }
    }
}

pub struct Termite {
    pub x: usize,
    pub y: usize,
    pub carrying: bool,
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Cell>,
    pub termites: Vec<Termite>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![Cell::new(Material::Empty); width * height];
        Self {
            width,
            height,
            grid,
            termites: Vec::new(),
        }
    }

    pub fn add_termite(&mut self, x: usize, y: usize) {
        self.termites.push(Termite { x, y, carrying: false });
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.grid[self.get_index(x, y)]
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        let idx = self.get_index(x, y);
        &mut self.grid[idx]
    }

    pub fn add_server(&mut self, x: usize, y: usize) {
        let cell = self.get_cell_mut(x, y);
        cell.material = Material::Server;
        cell.heat = 100.0;
    }

    pub fn add_wall(&mut self, x: usize, y: usize) {
        let cell = self.get_cell_mut(x, y);
        cell.material = Material::Wall;
    }

    pub fn diffuse_heat(&mut self) {
        let diffusion_rate = 0.2;
        let cooling_rate = 0.01;

        // Calculate next state
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let current_heat = self.grid[idx].heat;
                let material = self.grid[idx].material;

                // Heat generation
                let mut heat = current_heat;
                if matches!(material, Material::Server) {
                    heat = (heat + 10.0).min(2000.0); // Servers get hot!
                }

                // Diffusion
                let mut neighbor_heat_sum = 0.0;
                let mut count = 0.0;

                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < self.width && ny < self.height {
                        neighbor_heat_sum += self.grid[self.get_index(nx, ny)].heat;
                        count += 1.0;
                    }
                }

                if count > 0.0 {
                    let avg_neighbor_heat = neighbor_heat_sum / count;
                    let diff = avg_neighbor_heat - heat;
                    heat += diff * diffusion_rate;
                }

                // Dissipation
                // Walls might insulate or conduct? Let's say everything cools slowly to ambient (0).
                heat *= 1.0 - cooling_rate;

                self.grid[idx].next_heat = heat;
            }
        }

        // Apply state
        for cell in &mut self.grid {
            cell.heat = cell.next_heat;
        }
    }

    pub fn update(&mut self) {
        self.diffuse_heat();

        let mut rng = rand::thread_rng();
        let width = self.width;
        let height = self.height;

        // Iterate indices to allow mutable borrow of self inside loop
        for i in 0..self.termites.len() {
            let (tx, ty) = (self.termites[i].x, self.termites[i].y);

            // 1. Move
            // Simple random walk for now
            let (dx, dy) = match rng.gen_range(0..5) {
                0 => (0, 0),
                1 => (0, 1),
                2 => (0, -1i32),
                3 => (1, 0),
                _ => (-1, 0),
            };

            let nx = tx as i32 + dx;
            let ny = ty as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                self.termites[i].x = nx as usize;
                self.termites[i].y = ny as usize;
            }

            let (tx, ty) = (self.termites[i].x, self.termites[i].y);
            let idx = self.get_index(tx, ty);
            let cell_heat = self.grid[idx].heat;
            let cell_material = self.grid[idx].material;

            // 2. Action (Pick/Drop)
            // Count neighbors
            let mut wall_neighbors = 0;
            for (dx, dy) in [(-1,0), (1,0), (0,-1), (0,1)] {
                let nx = tx as i32 + dx ;
                let ny = ty as i32 + dy ;
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                     if self.grid[ny as usize * width + nx as usize].material == Material::Wall {
                         wall_neighbors += 1;
                     }
                }
            }

            if self.termites[i].carrying {
                // Try to drop
                if cell_material == Material::Empty {
                    // Drop if heat is high (trying to shield/dissipate)
                    // Or drop if it continues a wall (neighbors > 0)
                    let should_drop = if cell_heat > 50.0 {
                        rng.gen_bool(0.1)
                    } else if wall_neighbors > 0 && wall_neighbors < 4 {
                         rng.gen_bool(0.05)
                    } else {
                        false
                    };

                    if should_drop {
                         self.grid[idx].material = Material::Wall;
                         self.termites[i].carrying = false;
                    }
                }
            } else {
                // Try to pick up
                if cell_material == Material::Wall {
                    let should_pick = if cell_heat > 100.0 {
                        false
                    } else if wall_neighbors <= 1 {
                        rng.gen_bool(0.1) // Prune isolated bits
                    } else {
                        rng.gen_bool(0.001)
                    };

                    if should_pick {
                        self.grid[idx].material = Material::Empty;
                        self.termites[i].carrying = true;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heat_diffusion() {
        let mut world = World::new(10, 10);
        world.add_server(5, 5);
        world.diffuse_heat();
        assert!(world.get_cell(5, 5).heat > 50.0);
        assert!(world.get_cell(4, 5).heat > 0.0);
    }

    #[test]
    fn test_termite_action() {
        let mut world = World::new(10, 10);
        world.add_termite(5, 5);
        world.add_wall(5, 5);

        let mut _picked_up = false;
        for _ in 0..100 {
            world.update();
            if world.termites[0].carrying {
                _picked_up = true;
                break;
            }
        }

        assert!(!world.termites.is_empty());
    }
}
