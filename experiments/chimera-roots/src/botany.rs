use crate::grid::{CellType, Grid};
use ::rand::Rng;
use chimera_lang::prelude::*;
use macroquad::prelude::*;

pub struct RootTip {
    pub position: (usize, usize),
    pub vm: ChimeraVM,
    pub genome: Vec<Gene>,
    pub energy: f32,
    pub path: Vec<(usize, usize)>,
    pub finished: bool,
    pub reached_water: bool,
}

impl RootTip {
    pub fn new(start: (usize, usize), genome: Vec<Gene>) -> Self {
        let strand = Strand {
            genes: genome.clone(),
        };
        let helix = Helix {
            strands: vec![strand],
        };
        let dna = Dna { helix };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000; // Starting energy

        Self {
            position: start,
            vm,
            genome,
            energy: 100.0,
            path: vec![start],
            finished: false,
            reached_water: false,
        }
    }

    pub fn random_genome(len: usize) -> Vec<Gene> {
        let mut rng = ::rand::thread_rng();
        let mut genes = Vec::new();
        // A subset of useful opcodes for navigation
        let opcodes = [
            OpCode::Add,
            OpCode::Sub,
            OpCode::Mul,
            OpCode::Div,
            OpCode::Push,
            OpCode::Drop,
            OpCode::Dup,
            OpCode::Swap,
            OpCode::GRead,  // Read from grid (sensors)
            OpCode::GWrite, // Write to grid (memory)
            OpCode::Jump,
            OpCode::Brz,
            OpCode::Photosynthesize, // Gain energy?
        ];

        for _ in 0..len {
            let op = opcodes[rng.gen_range(0..opcodes.len())].clone();
            let args = match op {
                OpCode::Push => vec![Nucleotide::Number(rng.gen_range(0..4))], // Push direction or value
                OpCode::Jump | OpCode::Brz => {
                    vec![Nucleotide::Number(rng.gen_range(0..len as i64))]
                }
                OpCode::GRead | OpCode::GWrite => vec![], // GRead takes from stack
                _ => vec![],
            };
            genes.push(Gene { op, args });
        }
        genes
    }
}

pub struct Plant {
    pub tips: Vec<RootTip>,
    pub generation: usize,
    pub best_fitness: f32,
    pub start: (usize, usize),
}

impl Plant {
    pub fn new(start: (usize, usize), pop_size: usize) -> Self {
        let mut tips = Vec::new();
        for _ in 0..pop_size {
            let genome = RootTip::random_genome(20);
            tips.push(RootTip::new(start, genome));
        }
        Self {
            tips,
            generation: 1,
            best_fitness: 0.0,
            start,
        }
    }

    pub fn update(&mut self, grid: &mut Grid) {
        for tip in &mut self.tips {
            if tip.finished {
                continue;
            }

            // Input: Write neighbors to VM memory (Grid[0][0..4])
            // We need a consistent ordering for inputs: Up, Right, Down, Left
            let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
            for (i, (dx, dy)) in dirs.iter().enumerate() {
                let nx = tip.position.0 as i32 + dx;
                let ny = tip.position.1 as i32 + dy;
                let val = if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32
                {
                    match grid.get(nx as usize, ny as usize).unwrap().cell_type {
                        CellType::Soil => 1,
                        CellType::HardSoil => 2,
                        CellType::Rock => 100,  // Obstacle
                        CellType::Water => 200, // Goal
                        CellType::Seed => 0,
                    }
                } else {
                    100 // Wall
                };

                // Write to VM grid: row 0, col i
                tip.vm.grid[0][i] = Value::Int(val);
            }

            // Execute VM
            for _ in 0..10 {
                tip.vm.step();
                if tip.vm.energy <= 0 {
                    tip.finished = true;
                    break;
                }
            }

            if tip.finished {
                continue;
            }

            // Output: Pop from stack -> Direction
            // 0: Up, 1: Right, 2: Down, 3: Left
            if let Some(val) = tip.vm.stack.pop() {
                if let Value::Int(dir) = val {
                    let dir = (dir.abs() % 4) as usize;
                    let (dx, dy) = dirs[dir];
                    let nx = tip.position.0 as i32 + dx;
                    let ny = tip.position.1 as i32 + dy;

                    if nx >= 0 && nx < grid.width as i32 && ny >= 0 && ny < grid.height as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        let cell_type = grid.get(nx, ny).unwrap().cell_type;

                        if cell_type == CellType::Rock {
                            // Hit rock, die
                            tip.finished = true;
                        } else if cell_type == CellType::Water {
                            // Found water!
                            tip.reached_water = true;
                            tip.finished = true;
                            tip.path.push((nx, ny));
                            tip.position = (nx, ny);
                        } else {
                            // Move
                            // Check if already in path (don't backtrack immediately)
                            if !tip.path.contains(&(nx, ny)) {
                                tip.path.push((nx, ny));
                                tip.position = (nx, ny);
                                tip.energy -= 1.0; // Movement cost
                                if tip.energy <= 0.0 {
                                    tip.finished = true;
                                }
                            }
                        }
                    } else {
                        // Hit wall
                        tip.finished = true;
                    }
                }
            } else {
                // No output, just wait
            }
        }
    }

    pub fn next_generation(&mut self, grid: &Grid) {
        let mut rng = ::rand::thread_rng();

        // Calculate fitness
        // Fitness = 1.0 / distance_to_water
        // If reached water, fitness = 100.0 + remaining_energy
        let mut fitnesses: Vec<(usize, f32)> = self
            .tips
            .iter()
            .enumerate()
            .map(|(i, tip)| {
                let score = if tip.reached_water {
                    100.0 + tip.energy
                } else {
                    if let Some(goal) = grid.goal {
                        let dist = ((tip.position.0 as f32 - goal.0 as f32).powi(2)
                            + (tip.position.1 as f32 - goal.1 as f32).powi(2))
                        .sqrt();
                        100.0 / (dist + 1.0)
                    } else {
                        0.0
                    }
                };
                (i, score)
            })
            .collect();

        fitnesses.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.best_fitness = fitnesses[0].1;

        // Select top 20%
        let elite_count = (self.tips.len() as f32 * 0.2).ceil() as usize;
        let elite_indices: Vec<usize> = fitnesses
            .iter()
            .take(elite_count)
            .map(|(i, _)| *i)
            .collect();

        let mut new_tips = Vec::new();

        // Elitism: keep best
        new_tips.push(RootTip::new(
            self.start,
            self.tips[elite_indices[0]].genome.clone(),
        ));

        // Breeding / Mutation
        while new_tips.len() < self.tips.len() {
            let parent_idx = elite_indices[rng.gen_range(0..elite_indices.len())];
            let mut genome = self.tips[parent_idx].genome.clone();

            // Mutate
            if rng.gen_bool(0.3) {
                let mutation_type = rng.gen_range(0..3);
                match mutation_type {
                    0 => {
                        // Change a gene
                        let idx = rng.gen_range(0..genome.len());
                        genome[idx] = RootTip::random_genome(1)[0].clone();
                    }
                    1 => {
                        // Insert a gene
                        let idx = rng.gen_range(0..genome.len());
                        genome.insert(idx, RootTip::random_genome(1)[0].clone());
                    }
                    2 => {
                        // Delete a gene
                        if genome.len() > 1 {
                            let idx = rng.gen_range(0..genome.len());
                            genome.remove(idx);
                        }
                    }
                    _ => {}
                }
            }

            // Crossover (maybe later)

            new_tips.push(RootTip::new(self.start, genome));
        }

        self.tips = new_tips;
        self.generation += 1;
    }

    pub fn draw(&self, cell_size: f32, offset_x: f32, offset_y: f32) {
        for tip in &self.tips {
            // Draw path
            if tip.path.len() > 1 {
                for i in 0..tip.path.len() - 1 {
                    let p1 = tip.path[i];
                    let p2 = tip.path[i + 1];
                    draw_line(
                        p1.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                        p1.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                        p2.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                        p2.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                        2.0,
                        if tip.reached_water {
                            BLUE
                        } else {
                            Color::new(0.6, 0.4, 0.2, 0.3)
                        }, // Faint brown for trails
                    );
                }
            }

            // Draw tip
            if !tip.finished {
                draw_circle(
                    tip.position.0 as f32 * cell_size + cell_size / 2.0 + offset_x,
                    tip.position.1 as f32 * cell_size + cell_size / 2.0 + offset_y,
                    cell_size / 4.0,
                    Color::new(0.2, 0.8, 0.2, 0.8),
                );
            }
        }
    }
}
