use crate::quantum::{add_qubit, apply_gate, get_probability, measure, Gate, QubitSystem};
use rand::Rng;
use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Wall,
    Exit,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub glyph: char,
    pub color: Color,
    // Quantum properties
    pub is_qubit: bool,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub x: usize,
    pub y: usize,
    pub inventory: Vec<Gate>,
    pub score: usize,
}

pub struct GameState {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Tile>>,
    pub systems: HashMap<usize, QubitSystem>,
    pub entity_map: HashMap<usize, (usize, usize)>,
    pub next_system_id: usize,
    pub entities: Vec<Entity>,
    pub player: Player,
    pub message: String,
    #[allow(dead_code)]
    pub next_entity_id: usize,
}

impl GameState {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Initialize Grid
        let mut grid = vec![vec![Tile::Wall; width]; height];

        // Simple Room Carving (for now just a box)
        for row in grid.iter_mut().take(height - 1).skip(1) {
            for cell in row.iter_mut().take(width - 1).skip(1) {
                *cell = Tile::Empty;
            }
        }

        // Place Exit
        grid[height / 2][width - 2] = Tile::Exit;

        let mut next_system_id = 0;
        let mut systems = HashMap::new();
        let mut entity_map = HashMap::new();
        let mut entities = Vec::new();
        let mut next_entity_id = 0;

        // Spawn Qubits
        for _ in 0..20 {
            let x = rng.gen_range(2..width - 2);
            let y = rng.gen_range(2..height - 2);

            // Random State: 0, 1, or Superposition
            let id = next_entity_id;
            next_entity_id += 1;

            let r = rng.gen_range(0..3);
            match r {
                0 => add_qubit(&mut next_system_id, &mut systems, &mut entity_map, id, false), // |0>
                1 => add_qubit(&mut next_system_id, &mut systems, &mut entity_map, id, true),  // |1>
                _ => {
                    add_qubit(&mut next_system_id, &mut systems, &mut entity_map, id, false); // Start |0>
                    apply_gate(&mut systems, &entity_map, Gate::H, id).unwrap(); // Apply H -> |+>
                }
            }

            entities.push(Entity {
                id,
                x,
                y,
                glyph: 'Q',
                color: Color::Gray, // Will be updated by probability
                is_qubit: true,
            });
        }

        Self {
            width,
            height,
            grid,
            systems,
            entity_map,
            next_system_id,
            entities,
            player: Player {
                x: 2,
                y: height / 2,
                inventory: vec![Gate::H, Gate::H, Gate::X, Gate::X, Gate::CNOT, Gate::CNOT],
                score: 0,
            },
            message: "Welcome to Quantum Rogue! Walk into Qubits to Measure them.".to_string(),
            next_entity_id,
        }
    }

    pub fn update_entities(&mut self) {
        // Update colors based on quantum state
        for entity in &mut self.entities {
            if entity.is_qubit {
                let p = get_probability(&self.systems, &self.entity_map, entity.id);
                if p < 0.1 {
                    entity.color = Color::Blue; // Mostly |0>
                    entity.glyph = '0';
                } else if p > 0.9 {
                    entity.color = Color::Red; // Mostly |1>
                    entity.glyph = '1';
                } else {
                    entity.color = Color::Magenta; // Superposition
                    entity.glyph = '?';
                }
            }
        }
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32) {
        let nx = (self.player.x as i32 + dx).clamp(0, self.width as i32 - 1) as usize;
        let ny = (self.player.y as i32 + dy).clamp(0, self.height as i32 - 1) as usize;

        match self.grid[ny][nx] {
            Tile::Wall => {
                self.message = "Bump!".to_string();
            }
            Tile::Exit => {
                let score = self.player.score;
                let inventory = self.player.inventory.clone();

                // Regenerate level
                *self = GameState::new(self.width, self.height);

                // Restore player state
                self.player.score = score;
                self.player.inventory = inventory;
                self.message = "New Level Generated.".to_string();
            }
            Tile::Empty => {
                // Check entity collision
                if let Some(idx) = self.entities.iter().position(|e| e.x == nx && e.y == ny) {
                    let entity = &self.entities[idx];
                    if entity.is_qubit {
                        // Measure!
                        match measure(&mut self.next_system_id, &mut self.systems, &mut self.entity_map, entity.id) {
                            Ok(val) => {
                                if val {
                                    self.message = "Measured |1>! +10 Points.".to_string();
                                    self.player.score += 10;
                                    // Remove entity
                                    self.entities.remove(idx);
                                    // Move player
                                    self.player.x = nx;
                                    self.player.y = ny;
                                } else {
                                    self.message = "Measured |0>! It vanished.".to_string();
                                    // Remove entity
                                    self.entities.remove(idx);
                                    // Move player
                                    self.player.x = nx;
                                    self.player.y = ny;
                                }
                            }
                            Err(e) => {
                                self.message = format!("Measurement Error: {}", e);
                            }
                        }
                    }
                } else {
                    self.player.x = nx;
                    self.player.y = ny;
                }
            }
        }
    }

    pub fn find_nearest_qubits(&self, n: usize) -> Vec<usize> {
        // Simple distance check
        let mut dists: Vec<(usize, f64)> = self
            .entities
            .iter()
            .filter(|e| e.is_qubit)
            .map(|e| {
                let dx = e.x as f64 - self.player.x as f64;
                let dy = e.y as f64 - self.player.y as f64;
                (e.id, dx * dx + dy * dy)
            })
            .collect();

        dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        dists.iter().take(n).map(|(id, _)| *id).collect()
    }
}
