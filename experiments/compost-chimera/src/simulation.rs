use crate::compost::{CompostBin, CompostFile};
use crate::decay::{apply_decay, DecayLevel};
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
use rand::Rng;
use std::fs;

pub struct Agent {
    pub vm: ChimeraVM,
    pub x: usize,
    pub y: usize,
    pub id: usize,
}

pub struct Simulation {
    pub agents: Vec<Agent>,
    pub bin: CompostBin,
    pub current_file_idx: usize,
    pub current_content: Vec<Vec<char>>, // 2D grid of characters
    pub width: usize,
    pub height: usize,
}

impl Simulation {
    pub fn new(root: &std::path::Path) -> anyhow::Result<Self> {
        let bin = CompostBin::scan(root)?;
        let mut sim = Self {
            agents: Vec::new(),
            bin,
            current_file_idx: 0,
            current_content: Vec::new(),
            width: 80,
            height: 24,
        };
        sim.load_file(0);
        sim.spawn_agents(5);
        Ok(sim)
    }

    pub fn load_file(&mut self, idx: usize) {
        if idx >= self.bin.files.len() {
            return;
        }
        self.current_file_idx = idx;
        let file = &self.bin.files[idx];

        let content = if let Ok(raw) = fs::read_to_string(&file.path) {
            apply_decay(&raw, file.decay_level)
        } else {
            "Error reading file".to_string()
        };

        // Convert to 2D grid
        self.current_content = content.lines().map(|l| l.chars().collect()).collect();
        self.height = self.current_content.len().max(10);
        self.width = self
            .current_content
            .iter()
            .map(|l| l.len())
            .max()
            .unwrap_or(0)
            .max(40);

        // Reset agents logic if needed, or just respawn them randomly
        for agent in &mut self.agents {
            let mut rng = rand::thread_rng();
            agent.y = rng.gen_range(0..self.height);
            agent.x = rng.gen_range(0..self.width);
        }
    }

    pub fn spawn_agents(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for i in 0..count {
            // Create a "Decomposer" genome
            // Logic:
            // 1. Check stack for input (we push the char under them)
            // 2. If it's a "bad" char (glitch), consume it.
            // 3. Move randomly.

            // Simplified Genome:
            // [ Consume() Push(RandDir) Print() ]
            // We rely on the VM's "consume" to gain energy.
            // But we need to define what they consume.
            // Let's rely on standard "Print" to signal movement.
            // 0: Up, 1: Down, 2: Left, 3: Right

            let genes = vec![
                Gene {
                    op: OpCode::Consume,
                    args: vec![],
                }, // Try to eat whatever is on stack
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(4)],
                }, // Modulo 4
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("?".to_string())],
                }, // Random int
                Gene {
                    op: OpCode::Print,
                    args: vec![],
                }, // Output direction
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(0)],
                }, // Loop
            ];

            let dna = Dna {
                helix: Helix {
                    strands: vec![Strand { genes }],
                },
            };
            let mut vm = ChimeraVM::new(dna);
            vm.energy = 50; // Start energy

            self.agents.push(Agent {
                vm,
                x: rng.gen_range(0..self.width.max(1)),
                y: rng.gen_range(0..self.height.max(1)),
                id: i,
            });
        }
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();

        // Remove dead agents
        self.agents.retain(|a| !a.vm.halted);

        // Spawn new if low
        if self.agents.len() < 3 {
            self.spawn_agents(1);
        }

        for agent in &mut self.agents {
            // 1. Sense Environment
            // Get char at agent pos
            let char_at_pos = if agent.y < self.current_content.len() {
                if agent.x < self.current_content[agent.y].len() {
                    Some(self.current_content[agent.y][agent.x])
                } else {
                    None
                }
            } else {
                None
            };

            // 2. Feed Input
            if let Some(c) = char_at_pos {
                // Determine nutritional value
                // Glitches are high energy? Or Code is high energy?
                // Let's say Glitches (Compost) are food.
                let is_glitch = matches!(c, '░' | '▒' | '▓' | '█' | '?' | '!' | '@');

                // Push value to stack
                // If it's a glitch, push high value.
                let val = if is_glitch { 10 } else { 0 };
                agent.vm.stack.push(Value::Int(val));

                // Also push the char code for reference
                agent.vm.stack.push(Value::Int(c as i64));
            } else {
                agent.vm.stack.push(Value::Int(0)); // Nothing
                agent.vm.stack.push(Value::Int(0));
            }

            // 3. Step VM
            agent.vm.output.clear();
            agent.vm.step();

            // 4. Process Output (Actions)
            // Did it print a number? (Direction)
            // Did it consume? (Energy check is hard externally without tracking delta)
            // But we pushed energy-value to stack. If it ran `Consume`, it popped it.

            // Parse output for movement
            for msg in &agent.vm.output {
                if let Ok(dir) = msg.parse::<usize>() {
                    // 0: Up, 1: Down, 2: Left, 3: Right
                    // But we pushed a random number char '?' which prints a number 0-9 usually?
                    // Wait, `OpCode::Unknown("?")` isn't standard.
                    // Ah, ChimeraVM has `Ribosome` specific ops like `?`.
                    // But standard VM doesn't have `Rand` opcode easily accessible without Nova features?
                    // I used `Nucleotide::String("?")` which might be interpreted as an OpCode::Unknown("?")
                    // which does nothing.

                    // Let's just use random movement in the host for now if the agent "signals" it.
                    // Or assume the agent outputs a direction.

                    match dir % 4 {
                        0 => {
                            if agent.y > 0 {
                                agent.y -= 1
                            }
                        }
                        1 => agent.y += 1,
                        2 => {
                            if agent.x > 0 {
                                agent.x -= 1
                            }
                        }
                        3 => agent.x += 1,
                        _ => {}
                    }
                }
            }

            // Random walk fallback if no output (mutated agents might stop outputting)
            if agent.vm.output.is_empty() {
                let dir = rng.gen_range(0..4);
                match dir {
                    0 => {
                        if agent.y > 0 {
                            agent.y -= 1
                        }
                    }
                    1 => agent.y += 1,
                    2 => {
                        if agent.x > 0 {
                            agent.x -= 1
                        }
                    }
                    3 => agent.x += 1,
                    _ => {}
                }
            }

            // 5. Modify Environment (Eating)
            // If they are on a glitch, and they have energy, maybe they "clean" it?
            // Or if they eat it, they remove it (turn to space).
            if let Some(c) = char_at_pos {
                let is_glitch = matches!(c, '░' | '▒' | '▓' | '█');
                if is_glitch {
                    // "Eat" the compost
                    if agent.y < self.current_content.len()
                        && agent.x < self.current_content[agent.y].len()
                    {
                        self.current_content[agent.y][agent.x] = ' '; // Consumed
                        agent.vm.energy += 2; // Reward
                    }
                }
            }
        }
    }
}
