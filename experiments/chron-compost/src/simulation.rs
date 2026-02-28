use crate::blame::LineInfo;
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};
use rand::Rng;

pub struct Agent {
    pub vm: ChimeraVM,
    pub x: usize,
    pub y: usize,
    pub id: usize,
}

pub struct Simulation {
    pub agents: Vec<Agent>,
    pub width: usize,
    pub height: usize,
    pub blame_info: Vec<LineInfo>,
}

impl Simulation {
    pub fn new(
        blame_info: Vec<LineInfo>,
        content_lines: usize,
        max_line_len: usize,
    ) -> anyhow::Result<Self> {
        let mut sim = Self {
            agents: Vec::new(),
            width: max_line_len.max(40),
            height: content_lines.max(10),
            blame_info,
        };
        sim.spawn_agents(5);
        Ok(sim)
    }

    pub fn spawn_agents(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for i in 0..count {
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
                evolution_config: None,
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
            let mut age_score = 1.0;
            if agent.y < self.blame_info.len() {
                age_score = self.blame_info[agent.y].age_score;
            }

            // 2. Feed Input
            // Cold code (old) is food. age_score near 0.0 is very old.
            let is_old = age_score < 0.2;
            let val = if is_old { 10 } else { 0 };
            agent.vm.stack.push(Value::Int(val));
            agent.vm.stack.push(Value::Int(if is_old { 1 } else { 0 }));

            // 3. Step VM
            agent.vm.output.clear();
            agent.vm.step();

            // 4. Process Output (Actions)
            for msg in &agent.vm.output {
                if let Ok(dir) = msg.parse::<usize>() {
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

            // Random walk fallback if no output
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
            if is_old {
                if agent.y < self.blame_info.len() {
                    // Refactor! Reset the age score to 1.0 (hot/new)
                    self.blame_info[agent.y].age_score = 1.0;
                    agent.vm.energy += 5; // Reward
                }
            }
        }
    }
}
