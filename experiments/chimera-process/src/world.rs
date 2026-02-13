use std::collections::HashMap;
use sysinfo::{Pid, System};
use crate::agent::ProcessAgent;
use chimera_lang::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct World {
    pub system: System,
    pub agents: HashMap<Pid, ProcessAgent>,
}

impl World {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self {
            system,
            agents: HashMap::new(),
        }
    }

    pub fn update(&mut self) {
        self.system.refresh_all();

        let current_pids: Vec<Pid> = self.system.processes().keys().cloned().collect();

        // Remove dead agents
        self.agents.retain(|pid, _| current_pids.contains(pid));

        // Update or Add agents
        for (pid, process) in self.system.processes() {
            if !self.agents.contains_key(pid) {
                // Spawn new agent
                let dna = generate_dna(process.name());
                let agent = ProcessAgent::new(*pid, process.name().to_string(), dna);
                self.agents.insert(*pid, agent);
            }

            if let Some(agent) = self.agents.get_mut(pid) {
                agent.update(process.cpu_usage(), process.memory());
            }
        }
    }
}

fn generate_dna(name: &str) -> Dna {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let seed = hasher.finish();
    let mut rng = StdRng::seed_from_u64(seed);

    let ops = vec![
        OpCode::Photosynthesize,
        OpCode::Consume,
        OpCode::GRead,
        OpCode::GWrite,
        OpCode::Radiate,
        OpCode::Siphon,
        OpCode::Jump,
        OpCode::Brz,
        OpCode::Push,
        OpCode::Add,
        OpCode::Sub,
        OpCode::Dup,
        OpCode::Swap,
    ];

    let mut genes = Vec::new();
    for _ in 0..32 {
        let op = ops[rng.gen_range(0..ops.len())].clone();
        let args = if matches!(op, OpCode::Push | OpCode::Jump | OpCode::Brz) {
             vec![Nucleotide::Number(rng.gen_range(0..16))]
        } else {
             vec![]
        };
        genes.push(Gene { op, args });
    }

    // Ensure it loops
    genes.push(Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] });

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}
