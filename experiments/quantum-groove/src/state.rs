use rand::Rng;
use ratatui::style::Color;
use crate::quantum::{Gate, QuantumManager};
use crate::audio::{AudioEvent, Waveform};
use std::sync::mpsc::Sender;

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
    pub quantum: QuantumManager,
    pub entities: Vec<Entity>,
    pub player: Player,
    pub message: String,
    pub next_entity_id: usize,

    // Rhythm properties
    pub bpm: u32,
    pub last_beat_time: f64,
    pub audio_tx: Sender<AudioEvent>,
}

impl GameState {
    pub fn new(width: usize, height: usize, audio_tx: Sender<AudioEvent>) -> Self {
        let mut rng = rand::thread_rng();

        // Initialize Grid
        let mut grid = vec![vec![Tile::Wall; width]; height];

        // Simple Room Carving (for now just a box)
        for y in 1..height-1 {
            for x in 1..width-1 {
                grid[y][x] = Tile::Empty;
            }
        }

        // Place Exit
        grid[height/2][width-2] = Tile::Exit;

        let mut quantum = QuantumManager::new();
        let mut entities = Vec::new();
        let mut next_entity_id = 0;

        // Spawn Qubits
        for _ in 0..20 {
            let x = rng.gen_range(2..width-2);
            let y = rng.gen_range(2..height-2);

            // Random State: 0, 1, or Superposition
            let id = next_entity_id;
            next_entity_id += 1;

            let r = rng.gen_range(0..3);
            match r {
                0 => quantum.add_qubit(id, false), // |0>
                1 => quantum.add_qubit(id, true),  // |1>
                _ => {
                    quantum.add_qubit(id, false); // Start |0>
                    quantum.apply_gate(Gate::H, id).unwrap(); // Apply H -> |+>
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
            quantum,
            entities,
            player: Player {
                x: 2,
                y: height / 2,
                inventory: vec![Gate::H, Gate::H, Gate::X, Gate::X, Gate::CNOT, Gate::CNOT],
                score: 0,
            },
            message: "Welcome to Quantum Groove! Move to the Beat!".to_string(),
            next_entity_id,
            bpm: 120,
            last_beat_time: 0.0,
            audio_tx,
        }
    }

    pub fn update_entities(&mut self) {
        // Update colors based on quantum state
        for entity in &mut self.entities {
            if entity.is_qubit {
                let p = self.quantum.get_probability(entity.id);
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

    pub fn trigger_beat(&mut self) {
        // Play beat sound
        let _ = self.audio_tx.send(AudioEvent {
            waveform: Waveform::Square, // Metronome tick
            frequency: 220.0,
            duration: 0.1,
            volume: 0.3,
            start_time: 0.0, // Handled by writer
        });

        // "Quantum Pulse": Randomly apply Hadamard to a few qubits to keep things uncertain?
        // Or maybe just flash the UI?
        // Let's keep it simple: Just sound for now.
    }

    pub fn try_move_player(&mut self, dx: i32, dy: i32, on_beat: bool) {
        if !on_beat {
            self.message = "Missed Beat! Movement unstable.".to_string();
            // Punishment: Teleport slightly randomly? Or just fail to measure?
            // Let's just make it a "fizzle" sound.
             let _ = self.audio_tx.send(AudioEvent {
                waveform: Waveform::Noise,
                frequency: 100.0,
                duration: 0.1,
                volume: 0.2,
                start_time: 0.0,
            });
            // Still allow movement but maybe no score or no measurement?
        } else {
             let _ = self.audio_tx.send(AudioEvent {
                waveform: Waveform::Sine,
                frequency: 440.0, // Harmonious movement
                duration: 0.1,
                volume: 0.4,
                start_time: 0.0,
            });
        }

        let nx = (self.player.x as i32 + dx).clamp(0, self.width as i32 - 1) as usize;
        let ny = (self.player.y as i32 + dy).clamp(0, self.height as i32 - 1) as usize;

        match self.grid[ny][nx] {
            Tile::Wall => {
                self.message = "Bump!".to_string();
            }
            Tile::Exit => {
                // Next level logic... simplified for demo
                self.message = "Level Complete!".to_string();
                // Generate sound
                 let _ = self.audio_tx.send(AudioEvent {
                    waveform: Waveform::Sawtooth,
                    frequency: 880.0,
                    duration: 0.5,
                    volume: 0.5,
                    start_time: 0.0,
                });
            }
            Tile::Empty => {
                // Check entity collision
                if let Some(idx) = self.entities.iter().position(|e| e.x == nx && e.y == ny) {
                    let entity = &self.entities[idx];
                    if entity.is_qubit {
                        if on_beat {
                            // Measure!
                            match self.quantum.measure(entity.id) {
                                Ok(val) => {
                                    if val {
                                        self.message = "Collapsed to |1>! +10 Points.".to_string();
                                        self.player.score += 10;
                                        self.entities.remove(idx);
                                        self.player.x = nx;
                                        self.player.y = ny;
                                         let _ = self.audio_tx.send(AudioEvent {
                                            waveform: Waveform::Square,
                                            frequency: 660.0, // Major 5th
                                            duration: 0.2,
                                            volume: 0.5,
                                            start_time: 0.0,
                                        });
                                    } else {
                                        self.message = "Collapsed to |0>. Vanished.".to_string();
                                        self.entities.remove(idx);
                                        self.player.x = nx;
                                        self.player.y = ny;
                                         let _ = self.audio_tx.send(AudioEvent {
                                            waveform: Waveform::Sine,
                                            frequency: 330.0, // Major 3rd
                                            duration: 0.2,
                                            volume: 0.5,
                                            start_time: 0.0,
                                        });
                                    }
                                }
                                Err(e) => {
                                    self.message = format!("Error: {}", e);
                                }
                            }
                        } else {
                            self.message = "Off-beat! Wave function did not collapse.".to_string();
                            // Pass through ghostly
                            self.player.x = nx;
                            self.player.y = ny;
                        }
                    }
                } else {
                    self.player.x = nx;
                    self.player.y = ny;
                }
            }
        }
    }
}
