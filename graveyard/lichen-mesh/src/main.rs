use ::rand::Rng;
use macroquad::prelude::*;

const GRID_WIDTH: usize = 120;
const GRID_HEIGHT: usize = 80;

#[derive(Clone, Copy, PartialEq, Debug)]
enum CellType {
    Empty,
    Algae,  // Frontend / Logic
    Fungus, // Backend / Database
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    cell_type: CellType,
    energy: f32,
    water: f32,
    age: u32,
    active: bool,
}

impl Cell {
    fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            energy: 10.0,
            water: 10.0,
            age: 0,
            active: true,
        }
    }
}

struct Packet {
    x: f32,
    y: f32,
    target_idx: usize,
    resource_type: usize, // 0 = Energy (Yellow), 1 = Water (Blue)
    speed: f32,
}

struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
    packets: Vec<Packet>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = ::rand::thread_rng();
        let mut cells = vec![
            Cell {
                cell_type: CellType::Empty,
                energy: 0.0,
                water: 0.0,
                age: 0,
                active: false
            };
            width * height
        ];

        // Seed the center
        let cx = width / 2;
        let cy = height / 2;

        for dy in -3..=3 {
            for dx in -3..=3 {
                let idx = (cy as isize + dy) as usize * width + (cx as isize + dx) as usize;
                if rng.gen::<f32>() < 0.5 {
                    cells[idx] = Cell::new(CellType::Algae);
                } else {
                    cells[idx] = Cell::new(CellType::Fungus);
                }
            }
        }

        Self {
            cells,
            width,
            height,
            packets: Vec::new(),
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    neighbors.push(ny as usize * self.width + nx as usize);
                }
            }
        }
        neighbors
    }

    fn update(&mut self) {
        let mut rng = ::rand::thread_rng();

        // 1. Calculate Flows & Create Packets
        let mut energy_delta = vec![0.0; self.cells.len()];
        let mut water_delta = vec![0.0; self.cells.len()];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = self.cells[idx];

                if !cell.active {
                    continue;
                }

                let neighbors = self.get_neighbors(x, y);

                match cell.cell_type {
                    CellType::Algae => {
                        if cell.water > 1.0 {
                            energy_delta[idx] += 2.0;
                            water_delta[idx] -= 0.5;
                        }

                        let fungus_neighbors: Vec<usize> = neighbors
                            .iter()
                            .filter(|&&ni| self.cells[ni].cell_type == CellType::Fungus)
                            .cloned()
                            .collect();

                        if !fungus_neighbors.is_empty() && cell.energy > 5.0 {
                            let share_amount = 1.0;
                            energy_delta[idx] -= share_amount;
                            for &ni in &fungus_neighbors {
                                energy_delta[ni] += share_amount / fungus_neighbors.len() as f32;
                                // Spawn visual packet (Energy)
                                if rng.gen::<f32>() < 0.1 {
                                    self.packets.push(Packet {
                                        x: x as f32,
                                        y: y as f32,
                                        target_idx: ni,
                                        resource_type: 0,
                                        speed: 0.1 + rng.gen::<f32>() * 0.1,
                                    });
                                }
                            }
                        }
                    }
                    CellType::Fungus => {
                        if cell.energy > 1.0 {
                            water_delta[idx] += 2.0;
                            energy_delta[idx] -= 0.5;
                        }

                        let algae_neighbors: Vec<usize> = neighbors
                            .iter()
                            .filter(|&&ni| self.cells[ni].cell_type == CellType::Algae)
                            .cloned()
                            .collect();

                        if !algae_neighbors.is_empty() && cell.water > 5.0 {
                            let share_amount = 1.0;
                            water_delta[idx] -= share_amount;
                            for &ni in &algae_neighbors {
                                water_delta[ni] += share_amount / algae_neighbors.len() as f32;
                                // Spawn visual packet (Water)
                                if rng.gen::<f32>() < 0.1 {
                                    self.packets.push(Packet {
                                        x: x as f32,
                                        y: y as f32,
                                        target_idx: ni,
                                        resource_type: 1,
                                        speed: 0.1 + rng.gen::<f32>() * 0.1,
                                    });
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // 2. Apply Flows & Update State
        let mut next_cells = self.cells.clone();

        for i in 0..self.cells.len() {
            if !next_cells[i].active {
                continue;
            }

            next_cells[i].energy += energy_delta[i];
            next_cells[i].water += water_delta[i];
            next_cells[i].age += 1;

            next_cells[i].energy = next_cells[i].energy.clamp(0.0, 100.0);
            next_cells[i].water = next_cells[i].water.clamp(0.0, 100.0);

            // Death
            if next_cells[i].energy <= 0.1 || next_cells[i].water <= 0.1 {
                if rng.gen::<f32>() < 0.05 {
                    next_cells[i] = Cell {
                        cell_type: CellType::Empty,
                        active: false,
                        ..next_cells[i]
                    };
                }
            }
        }

        // 3. Growth
        let current_cells_state = next_cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let cell = current_cells_state[idx];

                if !cell.active {
                    continue;
                }

                let neighbors = self.get_neighbors(x, y);
                let empty_neighbors: Vec<usize> = neighbors
                    .iter()
                    .filter(|&&ni| !current_cells_state[ni].active)
                    .cloned()
                    .collect();

                if empty_neighbors.is_empty() {
                    continue;
                }

                if cell.cell_type == CellType::Algae && cell.energy > 15.0 && cell.water > 5.0 {
                    if let Some(&target) =
                        empty_neighbors.get(rng.gen_range(0..empty_neighbors.len()))
                    {
                        let new_type = if rng.gen::<f32>() < 0.3 {
                            CellType::Fungus
                        } else {
                            CellType::Algae
                        };
                        next_cells[target] = Cell::new(new_type);
                        next_cells[idx].energy -= 10.0;
                    }
                } else if cell.cell_type == CellType::Fungus
                    && cell.water > 15.0
                    && cell.energy > 5.0
                {
                    if let Some(&target) =
                        empty_neighbors.get(rng.gen_range(0..empty_neighbors.len()))
                    {
                        let new_type = if rng.gen::<f32>() < 0.3 {
                            CellType::Algae
                        } else {
                            CellType::Fungus
                        };
                        next_cells[target] = Cell::new(new_type);
                        next_cells[idx].water -= 10.0;
                    }
                }
            }
        }

        self.cells = next_cells;

        // Update Packets
        let width = self.width;
        let mut survived_packets = Vec::new();
        for mut p in self.packets.drain(..) {
            let tx = p.target_idx % width;
            let ty = p.target_idx / width;
            let dx = tx as f32 - p.x;
            let dy = ty as f32 - p.y;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < p.speed {
                // Arrived
                continue;
            } else {
                p.x += (dx / dist) * p.speed;
                p.y += (dy / dist) * p.speed;
                survived_packets.push(p);
            }
        }
        self.packets = survived_packets;
    }
}

#[macroquad::main("Lichen Mesh")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0)); // Dark substrate

        // Update multiple times per frame for faster growth? No, keeping it realtime.
        grid.update();

        let cell_w = screen_width() / GRID_WIDTH as f32;
        let cell_h = screen_height() / GRID_HEIGHT as f32;

        // Draw Cells
        for y in 0..grid.height {
            for x in 0..grid.width {
                let idx = grid.get_index(x, y);
                let cell = grid.cells[idx];

                if cell.active {
                    let (color, size_factor) = match cell.cell_type {
                        CellType::Algae => {
                            let brightness = (cell.energy / 30.0).clamp(0.4, 1.0);
                            (
                                Color::new(0.0, brightness, 0.2 + brightness * 0.1, 1.0),
                                0.9,
                            )
                        }
                        CellType::Fungus => {
                            let brightness = (cell.water / 30.0).clamp(0.3, 0.8);
                            (Color::new(brightness, brightness, brightness, 1.0), 0.7)
                        }
                        _ => (BLACK, 0.0),
                    };

                    let px = x as f32 * cell_w;
                    let py = y as f32 * cell_h;

                    // Draw cell slightly smaller than grid to show structure
                    draw_rectangle(
                        px + (cell_w * (1.0 - size_factor) / 2.0),
                        py + (cell_h * (1.0 - size_factor) / 2.0),
                        cell_w * size_factor,
                        cell_h * size_factor,
                        color,
                    );
                }
            }
        }

        // Draw Packets (Microservice Traffic)
        for p in &grid.packets {
            let px = p.x * cell_w + cell_w / 2.0;
            let py = p.y * cell_h + cell_h / 2.0;
            let color = if p.resource_type == 0 {
                Color::new(1.0, 1.0, 0.0, 0.8) // Energy (Requests) - Yellow
            } else {
                Color::new(0.0, 0.5, 1.0, 0.8) // Water (Data) - Blue
            };
            draw_circle(px, py, cell_w * 0.3, color);
        }

        // Overlay UI
        draw_text(
            "LICHEN MESH // SYMBIOTIC MICROSERVICES",
            10.0,
            30.0,
            30.0,
            WHITE,
        );

        let algae_count = grid
            .cells
            .iter()
            .filter(|c| c.active && c.cell_type == CellType::Algae)
            .count();
        let fungus_count = grid
            .cells
            .iter()
            .filter(|c| c.active && c.cell_type == CellType::Fungus)
            .count();

        draw_text(
            &format!("FRONTEND NODES: {}", algae_count),
            10.0,
            60.0,
            20.0,
            GREEN,
        );
        draw_text(
            &format!("BACKEND NODES: {}", fungus_count),
            10.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("PACKETS: {}", grid.packets.len()),
            10.0,
            100.0,
            20.0,
            YELLOW,
        );

        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_initialization() {
        let cell = Cell::new(CellType::Algae);
        assert_eq!(cell.cell_type, CellType::Algae);
        assert_eq!(cell.energy, 10.0);
        assert_eq!(cell.water, 10.0);
        assert!(cell.active);
    }

    #[test]
    fn test_grid_initialization() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);
    }
}
