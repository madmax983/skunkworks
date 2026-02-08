use std::collections::HashMap;
use macroquad::prelude::*;
use ::rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellType {
    Empty,
    Algae,
    Fungus,
}

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_type: CellType,
    pub energy: f32,
    pub water: f32,
    pub age: u32,
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Self {
            cell_type,
            energy: 10.0,
            water: 10.0,
            age: 0,
        }
    }
}

pub struct LichenState {
    pub cells: HashMap<u64, Cell>,
    pub neighbors: HashMap<u64, Vec<u64>>,
}

impl LichenState {
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            neighbors: HashMap::new(),
        }
    }

    pub fn get_color(&self, hash: u64) -> Color {
        if let Some(cell) = self.cells.get(&hash) {
            match cell.cell_type {
                CellType::Algae => {
                    let brightness = (cell.energy / 50.0).clamp(0.2, 1.0);
                    Color::new(0.0, brightness, 0.2 + brightness * 0.1, 0.8)
                },
                CellType::Fungus => {
                    let brightness = (cell.water / 50.0).clamp(0.2, 0.8);
                    Color::new(brightness, brightness, brightness, 0.8)
                },
                CellType::Empty => Color::new(0.0, 0.0, 0.0, 0.0),
            }
        } else {
             Color::new(0.0, 0.0, 0.0, 0.0)
        }
    }

    pub fn register_node(&mut self, hash: u64, neighbor_hashes: Vec<u64>) {
        if !self.neighbors.contains_key(&hash) {
            self.neighbors.insert(hash, neighbor_hashes);
        }

        if !self.cells.contains_key(&hash) {
            // Pseudo-random generation
            let seed = hash.wrapping_mul(6364136223846793005);
            let r = (seed >> 32) as f32 / u32::MAX as f32;

            let cell_type = if r < 0.2 {
                CellType::Algae
            } else if r > 0.8 {
                CellType::Fungus
            } else {
                CellType::Empty
            };

            self.cells.insert(hash, Cell::new(cell_type));
        }
    }

    pub fn update(&mut self) {
        let mut energy_delta: HashMap<u64, f32> = HashMap::new();
        let mut water_delta: HashMap<u64, f32> = HashMap::new();
        let mut rng = ::rand::thread_rng();

        // 1. Calculate Flows
        for (&hash, cell) in &self.cells {
            let mut e_change = 0.0;
            let mut w_change = 0.0;

            // Base metabolism
            e_change -= 0.05;
            w_change -= 0.05;

            match cell.cell_type {
                CellType::Algae => {
                    if cell.water > 1.0 {
                        e_change += 1.0;
                        w_change -= 0.2;
                    }

                    if let Some(nbs) = self.neighbors.get(&hash) {
                         for &nb in nbs {
                             if let Some(nb_cell) = self.cells.get(&nb) {
                                 if nb_cell.cell_type == CellType::Fungus {
                                     e_change -= 0.5;
                                     *energy_delta.entry(nb).or_insert(0.0) += 0.5;
                                 }
                             }
                         }
                    }
                },
                CellType::Fungus => {
                    if cell.energy > 1.0 {
                        w_change += 1.0;
                        e_change -= 0.2;
                    }

                    if let Some(nbs) = self.neighbors.get(&hash) {
                         for &nb in nbs {
                             if let Some(nb_cell) = self.cells.get(&nb) {
                                 if nb_cell.cell_type == CellType::Algae {
                                     w_change -= 0.5;
                                     *water_delta.entry(nb).or_insert(0.0) += 0.5;
                                 }
                             }
                         }
                    }
                },
                _ => {}
            }
            *energy_delta.entry(hash).or_insert(0.0) += e_change;
            *water_delta.entry(hash).or_insert(0.0) += w_change;
        }

        // 2. Apply Deltas
        for (hash, cell) in self.cells.iter_mut() {
            if let Some(d) = energy_delta.get(hash) {
                cell.energy = (cell.energy + d).clamp(0.0, 100.0);
            }
            if let Some(d) = water_delta.get(hash) {
                cell.water = (cell.water + d).clamp(0.0, 100.0);
            }
        }

        // 3. Colonization (Growth)
        let mut new_sprouts: Vec<(u64, CellType)> = Vec::new();
        for (&hash, cell) in &self.cells {
             if cell.energy > 20.0 && cell.water > 20.0 {
                if let Some(nbs) = self.neighbors.get(&hash) {
                    for &nb in nbs {
                        if let Some(nb_cell) = self.cells.get(&nb) {
                            if nb_cell.cell_type == CellType::Empty {
                                if rng.gen::<f32>() < 0.05 {
                                    new_sprouts.push((nb, cell.cell_type));
                                }
                            }
                        }
                    }
                }
             }
        }

        for (hash, c_type) in new_sprouts {
            if let Some(c) = self.cells.get_mut(&hash) {
                c.cell_type = c_type;
                c.energy = 5.0;
                c.water = 5.0;
            }
        }
    }
}
