use crate::world::{Tile, World, GRID_HEIGHT, GRID_WIDTH};
use chimera_lang::prelude::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct Agent {
    pub id: u64,
    pub vm: ChimeraVM,
    pub pos: (usize, usize),
    pub anchor: Option<(usize, usize)>,
    pub energy: i32,
    pub color: Color,
    pub age: u32,
}

impl Agent {
    pub fn new(id: u64, x: usize, y: usize) -> Self {
        // Random DNA
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        let mut genes = Vec::new();
        for _ in 0..64 {
            let op = match rng.gen_range(0..14) {
                0 => OpCode::Push,
                1 => OpCode::Drop,
                2 => OpCode::Add,
                3 => OpCode::Sub,
                4 => OpCode::Mul,
                5 => OpCode::Div,
                6 => OpCode::Signal, // Important for Portal
                7 => OpCode::Receive,
                8 => OpCode::Jump,
                9 => OpCode::Brz,
                10 => OpCode::Dup,
                11 => OpCode::Swap,
                _ => OpCode::Nop,
            };

            let args = if op == OpCode::Push {
                vec![Nucleotide::Number(rng.gen_range(0..10))]
            } else if op == OpCode::Jump || op == OpCode::Brz {
                vec![Nucleotide::Number(rng.gen_range(0..64))]
            } else {
                vec![]
            };

            genes.push(Gene { op, args });
        }

        let dna = chimera_lang::prelude::Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
            evolution_config: None,
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000; // Agent energy separate from VM energy?
                          // Let's link them.

        Self {
            id,
            vm,
            pos: (x, y),
            anchor: None,
            energy: 1000,
            color: Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0),
            age: 0,
        }
    }

    pub fn update(&mut self, world: &mut World) {
        self.age += 1;
        self.energy -= 1; // Metabolic cost

        // --- 1. Sensors (Input) ---
        // Input 0: Smell Food (Vector encoded as X, Y)
        // Simple search for nearest food
        let (fx, fy) = self.find_nearest_food(world);
        let dx = (fx as i64) - (self.pos.0 as i64);
        let dy = (fy as i64) - (self.pos.1 as i64);

        self.push_input(0, dx);
        self.push_input(0, dy);

        // Input 1: Walls (N, S, E, W) - 1 if wall, 0 if empty
        let (x, y) = self.pos;
        let n = if y > 0 && world.get_tile(x, y - 1) == Tile::Wall {
            1
        } else {
            0
        };
        let s = if y < GRID_HEIGHT - 1 && world.get_tile(x, y + 1) == Tile::Wall {
            1
        } else {
            0
        };
        let e = if x < GRID_WIDTH - 1 && world.get_tile(x + 1, y) == Tile::Wall {
            1
        } else {
            0
        };
        let w = if x > 0 && world.get_tile(x - 1, y) == Tile::Wall {
            1
        } else {
            0
        };

        self.push_input(1, n);
        self.push_input(1, s);
        self.push_input(1, e);
        self.push_input(1, w);

        // Input 2: Portal Sensor (Nearby Portal?)
        // TODO

        // --- 2. Run VM ---
        // Give VM energy
        self.vm.energy = self.energy.max(0) as i64;
        for _ in 0..10 {
            self.vm.step();
        }
        // Sync energy back (if VM consumed execution energy)
        self.energy = self.vm.energy as i32;

        // --- 3. Actuators (Output) ---
        // Check Signals for Portal Spawning
        // Signal(0): Drop Anchor
        // Signal(1): Drop Link (Connect to Anchor)

        if let Some(queue) = self.vm.ether.get_mut(&0) {
            // Channel 0 for Portals
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    if v == 0 {
                        // Anchor
                        self.anchor = Some(self.pos);
                        self.energy -= 50; // Cost
                    } else if v == 1 {
                        // Link
                        if let Some(anchor_pos) = self.anchor {
                            if anchor_pos != self.pos {
                                // Check if distance is reasonable? Or infinite range?
                                // Check if tile is valid (not wall)
                                world.add_portal(anchor_pos, self.pos, self.id);
                                self.anchor = None; // Reset anchor
                                self.energy -= 100; // Cost
                            }
                        }
                    }
                }
            }
        }

        // Check Move Volition (Channel 1)
        let move_x = self.sum_channel(1);
        let move_y = self.sum_channel(2);

        let mut next_x = self.pos.0;
        let mut next_y = self.pos.1;

        if move_x > 50.0 {
            next_x += 1;
        }
        // Right
        else if move_x < -50.0 {
            next_x = next_x.saturating_sub(1);
        } // Left

        if move_y > 50.0 {
            next_y += 1;
        }
        // Down
        else if move_y < -50.0 {
            next_y = next_y.saturating_sub(1);
        } // Up

        // Clamp
        next_x = next_x.min(GRID_WIDTH - 1);
        next_y = next_y.min(GRID_HEIGHT - 1);

        // Collision Check
        let tile = world.get_tile(next_x, next_y);
        if tile != Tile::Wall {
            self.pos = (next_x, next_y);
        }

        // --- 4. Interactions ---
        // Eat Food
        if world.consume_food(self.pos.0, self.pos.1) {
            self.energy += 500;
        }

        // Use Portal
        if let Some(dest) = world.check_portal(self.pos.0, self.pos.1) {
            // Teleport!
            self.pos = dest;
            // self.energy -= 10; // Cost of teleporting?
        }
    }

    fn find_nearest_food(&self, world: &World) -> (usize, usize) {
        // Spiral search or just iterate all?
        // Optimizing: Just scan grid for nearest.
        let mut nearest_dist = f32::MAX;
        let mut target = (self.pos.0, self.pos.1); // Self if none found

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                if world.grid[x][y] == Tile::Food {
                    let d = (x as f32 - self.pos.0 as f32).powi(2)
                        + (y as f32 - self.pos.1 as f32).powi(2);
                    if d < nearest_dist {
                        nearest_dist = d;
                        target = (x, y);
                    }
                }
            }
        }
        target
    }

    fn push_input(&mut self, channel: u64, val: i64) {
        let ch = channel as i64;
        self.vm
            .ether
            .entry(ch)
            .or_default()
            .push_back(Value::Int(val));
        // Cap buffer
        if let Some(queue) = self.vm.ether.get_mut(&ch) {
            if queue.len() > 5 {
                queue.pop_front();
            }
        }
    }

    fn sum_channel(&mut self, channel: u64) -> f32 {
        let ch = channel as i64;
        let mut sum = 0.0;
        if let Some(queue) = self.vm.ether.get_mut(&ch) {
            while let Some(val) = queue.pop_front() {
                if let Value::Int(v) = val {
                    sum += v as f32;
                }
            }
        }
        sum
    }
}
