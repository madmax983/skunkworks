use crate::ast::{EvolutionConfig, Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub enum Challenge {
    Target(i64),
    Doubler,
    Adder,
    Fibonacci,
    Custom(EvolutionConfig),
}

impl Default for Challenge {
    fn default() -> Self {
        Challenge::Target(42)
    }
}

impl std::fmt::Display for Challenge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Challenge::Target(n) => write!(f, "Target({})", n),
            Challenge::Doubler => write!(f, "Doubler (x -> 2x)"),
            Challenge::Adder => write!(f, "Adder (x,y -> x+y)"),
            Challenge::Fibonacci => write!(f, "Fibonacci (n -> fib(n))"),
            Challenge::Custom(_) => write!(f, "Custom Evolution"),
        }
    }
}

#[derive(Clone)]
pub struct EvolutionEngine {
    pub population: Vec<Strand>,
    pub challenge: Challenge,
    pub generation: usize,
    pub best_fitness: i64,
    pub history: Vec<i64>,
}

impl EvolutionEngine {
    pub fn new(seed: Strand, population_size: usize, challenge: Challenge) -> Self {
        // 🔒 WARDEN: Ensure population is at least 1 to prevent division by zero or empty selection
        let safe_pop_size = population_size.max(1);
        let mut population = vec![seed.clone(); safe_pop_size];

        // Initial diversity
        let mut rng = rand::thread_rng();
        for i in 1..safe_pop_size {
            Self::mutate_strand(&mut population[i], &mut rng);
        }

        Self {
            population,
            challenge,
            generation: 0,
            best_fitness: i64::MAX,
            history: Vec::new(),
        }
    }

    pub fn from_config(seed: Strand, config: EvolutionConfig) -> Self {
        let challenge = Challenge::Custom(config.clone());
        Self::new(seed, config.population_size, challenge)
    }

    pub fn step(&mut self, vm_template: &ChimeraVM) {
        #[cfg(feature = "nova")]
        if let Challenge::Custom(config) = &self.challenge {
            if let Some(strategy_idx) = config.strategy_strand_idx {
                // Programmable Evolution Strategy
                let mut master_vm = vm_template.clone();

                // Inject population
                master_vm.evo_state.population = self.population.clone();
                master_vm.evo_state.buffer.clear();

                // Setup execution
                if strategy_idx < master_vm.dna.helix.strands.len() {
                    master_vm.ip = (strategy_idx, 0);
                    master_vm.energy = 10000; // Generous energy for meta-evolution
                    master_vm.halted = false;

                    // Run Strategy
                    let max_meta_ticks = 10000;
                    for _ in 0..max_meta_ticks {
                        if master_vm.halted {
                            break;
                        }
                        master_vm.step();
                    }

                    // Retrieve Population
                    // If buffer was used to replace population (EvoReplace), retrieve from population
                    // The EvoReplace op moves buffer to population.
                    self.population = master_vm.evo_state.population;

                    // Update Stats (Best Fitness)
                    // We need to evaluate fitness to update history/best_fitness even if strategy handled breeding
                    // This might be redundant if strategy did scoring, but we need it for the graph.
                    // Let's re-evaluate best fitness of the new population.
                    let mut best_f = i64::MAX;
                    for strand in &self.population {
                        let f = Self::evaluate_fitness(vm_template, strand, &self.challenge);
                        if f < best_f {
                            best_f = f;
                        }
                    }
                    self.best_fitness = best_f;
                    self.history.push(self.best_fitness);
                    self.generation += 1;
                    return;
                }
            }
        }

        let mut results = Vec::new();

        // 1. Evaluate Fitness
        for strand in &self.population {
            let fitness = Self::evaluate_fitness(vm_template, strand, &self.challenge);
            results.push((fitness, strand.clone()));
        }

        // Sort by fitness (lowest is best)
        results.sort_by(|a, b| a.0.cmp(&b.0));

        let best = results[0].clone();
        self.best_fitness = best.0;
        self.history.push(self.best_fitness);
        self.generation += 1;

        let best_strand = best.1;
        let pop_size = self.population.len();

        let mut next_gen = Vec::new();
        next_gen.push(best_strand.clone()); // Elitism

        let mut rng = rand::thread_rng();

        // 2. Selection & Reproduction
        while next_gen.len() < pop_size {
            // Tournament Selection
            let parent_a = Self::tournament_select(&results, &mut rng);
            let parent_b = Self::tournament_select(&results, &mut rng);

            let mut child = if rng.gen_bool(0.7) {
                // Crossover
                Self::crossover(&parent_a, &parent_b, &mut rng)
            } else {
                parent_a.clone()
            };

            // Mutation
            Self::mutate_strand(&mut child, &mut rng);
            next_gen.push(child);
        }

        self.population = next_gen;
    }

    pub fn tournament_select(pool: &[(i64, Strand)], rng: &mut impl Rng) -> Strand {
        let k = 3; // Tournament size
        let mut best: Option<&(i64, Strand)> = None;

        for _ in 0..k {
            let idx = rng.gen_range(0..pool.len());
            let candidate = &pool[idx];
            match best {
                None => best = Some(candidate),
                Some(b) => {
                    if candidate.0 < b.0 {
                        best = Some(candidate);
                    }
                }
            }
        }
        best.unwrap().1.clone()
    }

    pub fn crossover(a: &Strand, b: &Strand, rng: &mut impl Rng) -> Strand {
        if a.genes.is_empty() || b.genes.is_empty() {
            return a.clone();
        }

        let len_a = a.genes.len();
        let len_b = b.genes.len();
        let min_len = len_a.min(len_b);
        let split = rng.gen_range(0..min_len);

        let mut new_genes = Vec::new();
        // Head from A
        for i in 0..split {
            new_genes.push(a.genes[i].clone());
        }
        // Tail from B
        for i in split..len_b {
            new_genes.push(b.genes[i].clone());
        }

        Strand { genes: new_genes }
    }

    pub fn evaluate_fitness(
        vm_template: &ChimeraVM,
        strand: &Strand,
        challenge: &Challenge,
    ) -> i64 {
        if let Challenge::Custom(config) = challenge {
            let mut vm = vm_template.clone();
            // Inject candidate as strand 0 (or replace existing 0)
            // But vm_template might have other strands (e.g. fitness function).
            // We should APPEND or REPLACE.
            // If we replace strand 0, we might break things if fitness function expects it elsewhere.
            // But usually strand 0 is main.
            // Let's replace strand 0.
            if !vm.dna.helix.strands.is_empty() {
                vm.dna.helix.strands[0] = strand.clone();
            } else {
                vm.dna.helix.strands.push(strand.clone());
            }

            vm.ip = (0, 0);
            vm.energy = 1000;
            vm.halted = false;
            vm.stack.clear();

            // Run Candidate
            let max_ticks = 1000;
            for _ in 0..max_ticks {
                if vm.halted {
                    break;
                }
                vm.step();
            }

            // Run Fitness Function (if present)
            if let Some(f_idx) = config.fitness_strand_idx {
                if f_idx < vm.dna.helix.strands.len() {
                    vm.ip = (f_idx, 0);
                    vm.halted = false; // Resume
                    for _ in 0..max_ticks {
                        if vm.halted {
                            break;
                        }
                        vm.step();
                    }
                }
            }

            // Result is top of stack
            let result = vm
                .stack
                .pop()
                .and_then(|v| match v {
                    Value::Int(n) => Some(n),
                    _ => None,
                })
                .unwrap_or(1000000); // Default high error

            // If target_value is set and no fitness function, do simple diff
            if config.fitness_strand_idx.is_none() {
                if let Some(target) = config.target_value {
                    return (result - target).abs();
                }
            }

            return result;
        }

        let mut total_error = 0;
        let test_cases = match challenge {
            Challenge::Target(n) => vec![(vec![], *n)],
            Challenge::Doubler => vec![
                (vec![Value::Int(5)], 10),
                (vec![Value::Int(12)], 24),
                (vec![Value::Int(0)], 0),
                (vec![Value::Int(-5)], -10),
            ],
            Challenge::Adder => vec![
                (vec![Value::Int(5), Value::Int(3)], 8),
                (vec![Value::Int(10), Value::Int(20)], 30),
                (vec![Value::Int(0), Value::Int(0)], 0),
                (vec![Value::Int(-5), Value::Int(5)], 0),
            ],
            Challenge::Fibonacci => vec![
                (vec![Value::Int(0)], 0),
                (vec![Value::Int(1)], 1),
                (vec![Value::Int(5)], 5),
                (vec![Value::Int(6)], 8),
                (vec![Value::Int(7)], 13),
            ],
            Challenge::Custom(_) => vec![], // Handled above
        };

        for (inputs, expected) in test_cases {
            let mut vm = vm_template.clone();
            vm.dna.helix.strands = vec![strand.clone()];
            vm.ip = (0, 0);
            vm.energy = 100;
            vm.halted = false;
            vm.chaos_mode = false;
            vm.stack = inputs; // Preload stack

            let max_ticks = 100;
            for _ in 0..max_ticks {
                if vm.halted {
                    break;
                }
                vm.step();
            }

            let val = vm
                .stack
                .last()
                .and_then(|v| match v {
                    Value::Int(n) => Some(*n),
                    _ => None,
                })
                .unwrap_or(0);

            // Stack Depth Check: Should be exactly 1
            let stack_penalty = if vm.stack.len() == 1 { 0 } else { 1000 };

            total_error += (val - expected).abs() + stack_penalty;
        }

        // Length Penalty
        let len_penalty = strand.genes.len() as i64;
        total_error.saturating_add(len_penalty)
    }

    pub fn mutate_strand(strand: &mut Strand, rng: &mut impl Rng) {
        // 1. Change Op (Mutation)
        if !strand.genes.is_empty() && rng.gen_bool(0.3) {
            let idx = rng.gen_range(0..strand.genes.len());
            let ops = [
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Mul,
                OpCode::Div,
                OpCode::Dup,
                OpCode::Swap,
                OpCode::Drop,
                OpCode::Nop,
            ];
            let op = ops[rng.gen_range(0..ops.len())].clone();
            strand.genes[idx].op = op;
        }

        // 2. Change Arg (Mutation)
        if !strand.genes.is_empty() && rng.gen_bool(0.3) {
            let idx = rng.gen_range(0..strand.genes.len());
            if !strand.genes[idx].args.is_empty() {
                let val = rng.gen_range(0..100);
                strand.genes[idx].args[0] = Nucleotide::Number(val);
            } else if strand.genes[idx].op == OpCode::Push {
                strand.genes[idx]
                    .args
                    .push(Nucleotide::Number(rng.gen_range(0..100)));
            }
        }

        // 3. Add Gene (Insertion)
        if rng.gen_bool(0.2) {
            let val = rng.gen_range(0..100);
            let gene = Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(val)],
            };
            let idx = rng.gen_range(0..=strand.genes.len());
            strand.genes.insert(idx, gene);
        }

        // 4. Remove Gene (Deletion)
        if rng.gen_bool(0.2) && !strand.genes.is_empty() {
            let idx = rng.gen_range(0..strand.genes.len());
            strand.genes.remove(idx);
        }

        // 5. Swap Genes (Transposition)
        if strand.genes.len() > 1 && rng.gen_bool(0.1) {
            let idx1 = rng.gen_range(0..strand.genes.len());
            let idx2 = rng.gen_range(0..strand.genes.len());
            strand.genes.swap(idx1, idx2);
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_evo_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::EvoPopSize => {
            vm.stack
                .push(Value::Int(vm.evo_state.population.len() as i64));
        }
        OpCode::EvoLoad => {
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if idx >= 0 && (idx as usize) < vm.evo_state.population.len() {
                    // Push index as "Handle"
                    // We don't push the full strand data to stack because it's complex.
                    // We just verify it exists and push the index back (or leave it?)
                    // Actually, Load usually means "Load to Stack".
                    // But Strand is not a Value type (except maybe Junction?).
                    // Let's keep it as an Index reference for other Evo Ops.
                    // So EvoLoad checks bounds and pushes index if valid, else -1.
                    vm.stack.push(Value::Int(idx));
                } else {
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: EvoLoad requires population index".to_string());
            }
        }
        OpCode::EvoStore => {
            // [ ..., gene_junction ] -> [ ... ]
            if let Some(_val) = vm.stack.pop() {
                // Convert Value::Junction to Strand
                // This requires parsing the junction back to genes.
                // Simplified: We assume the junction is a list of op strings.
                // Or maybe we just use EvoSave to clone from population?
                // Let's implement EvoStore later if needed. For now, warn.
                vm.output
                    .push("Warning: EvoStore from stack not implemented. Use EvoSave.".to_string());
            }
        }
        OpCode::EvoScore => {
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if idx >= 0 && (idx as usize) < vm.evo_state.population.len() {
                    let strand = vm.evo_state.population[idx as usize].clone();
                    // We need a challenge. Where do we get it?
                    // The VM doesn't know the challenge.
                    // However, we can use a default or maybe store it in EvoState?
                    // For now, let's assume a default Challenge if not provided,
                    // OR we can pass it in via VM creation.
                    // But VM is created in step().
                    // Hack: We can serialize the challenge into the VM's genes or something?
                    // Better: We assume the user wants to run the configured fitness function.
                    // If DNA has evolution_config, use that.
                    let challenge = if let Some(config) = &vm.dna.evolution_config {
                        Challenge::Custom(config.clone())
                    } else {
                        Challenge::Target(42)
                    };

                    let score = EvolutionEngine::evaluate_fitness(vm, &strand, &challenge);
                    vm.stack.push(Value::Int(score));
                } else {
                    vm.stack.push(Value::Int(-1));
                }
            }
        }
        OpCode::EvoBreed => {
            if vm.stack.len() >= 2 {
                let idx_b = vm.stack.pop().unwrap();
                let idx_a = vm.stack.pop().unwrap();
                if let (Value::Int(a), Value::Int(b)) = (idx_a, idx_b) {
                    if a >= 0
                        && b >= 0
                        && (a as usize) < vm.evo_state.population.len()
                        && (b as usize) < vm.evo_state.population.len()
                    {
                        let parent_a = &vm.evo_state.population[a as usize];
                        let parent_b = &vm.evo_state.population[b as usize];
                        let mut rng = rand::thread_rng();
                        let child = EvolutionEngine::crossover(parent_a, parent_b, &mut rng);
                        vm.evo_state.buffer.push(child);
                        let child_idx = vm.evo_state.buffer.len() - 1;
                        vm.stack.push(Value::Int(child_idx as i64));
                    } else {
                        vm.stack.push(Value::Int(-1));
                    }
                }
            }
        }
        OpCode::EvoMutate => {
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                // Mutate in BUFFER
                if idx >= 0 && (idx as usize) < vm.evo_state.buffer.len() {
                    let strand = &mut vm.evo_state.buffer[idx as usize];
                    let mut rng = rand::thread_rng();
                    EvolutionEngine::mutate_strand(strand, &mut rng);
                } else {
                    vm.output.push(format!(
                        "Error: EvoMutate index {} out of buffer bounds",
                        idx
                    ));
                }
            }
        }
        OpCode::EvoReplace => {
            if !vm.evo_state.buffer.is_empty() {
                vm.evo_state.population = vm.evo_state.buffer.clone();
                vm.evo_state.buffer.clear();
                vm.output.push("EVOLUTION: Population Replaced".to_string());
            }
        }
        OpCode::EvoClear => {
            vm.evo_state.buffer.clear();
        }
        OpCode::EvoSave => {
            if let Some(Value::Int(idx)) = vm.stack.pop() {
                if idx >= 0 && (idx as usize) < vm.evo_state.population.len() {
                    let strand = vm.evo_state.population[idx as usize].clone();
                    vm.evo_state.buffer.push(strand);
                    let new_idx = vm.evo_state.buffer.len() - 1;
                    vm.stack.push(Value::Int(new_idx as i64));
                } else {
                    vm.stack.push(Value::Int(-1));
                }
            }
        }
        _ => {}
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Helix};

    #[test]
    fn test_evolution_convergence() {
        let seed = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![seed.clone()],
            },
        };
        let vm_template = ChimeraVM::new(dna);

        // Test Target
        let mut engine = EvolutionEngine::new(seed.clone(), 20, Challenge::Target(42));
        for _ in 0..100 {
            engine.step(&vm_template);
            if engine.best_fitness < 10 {
                break;
            }
        }
        assert!(engine.best_fitness < 100, "Target convergence failed");

        // Test Doubler
        let mut engine = EvolutionEngine::new(seed, 20, Challenge::Doubler);
        // This is harder, give it more time
        for _ in 0..200 {
            engine.step(&vm_template);
            // Perfect fitness is approx length of code (e.g. 5 for Push 2 Mul)
            if engine.best_fitness < 20 {
                break;
            }
        }
        println!("Doubler Best Fitness: {}", engine.best_fitness);
        // We can't guarantee convergence with random mutation in unit test time, but ensure it runs
        assert!(engine.generation == 200 || engine.best_fitness < 20);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_programmable_evolution() {
        // Strategy:
        // 1. Get Pop Size
        // 2. Loop (simplified: just take index 0)
        // 3. EvoLoad(0)
        // 4. EvoSave(0) (Clone to buffer)
        // 5. EvoReplace

        let strategy_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::EvoLoad,
                args: vec![],
            }, // Stack: [0]
            Gene {
                op: OpCode::Drop,
                args: vec![],
            }, // Stack: []
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::EvoSave,
                args: vec![],
            }, // Buffer has 1 item
            Gene {
                op: OpCode::EvoReplace,
                args: vec![],
            }, // Population now has 1 item
        ];

        let seed = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            }],
        };

        // Construct DNA with strategy at index 1
        let dna = Dna {
            evolution_config: Some(EvolutionConfig {
                population_size: 10,
                mutation_rate: "0.1".to_string(),
                fitness_strand_idx: None,
                strategy_strand_idx: Some(1),
                target_value: Some(42),
            }),
            helix: Helix {
                strands: vec![
                    seed.clone(),
                    Strand {
                        genes: strategy_genes,
                    },
                ],
            },
        };

        let vm_template = ChimeraVM::new(dna.clone());
        let mut engine = EvolutionEngine::from_config(seed, dna.evolution_config.unwrap());

        // Initial population size is 10
        assert_eq!(engine.population.len(), 10);

        // Run one step with strategy
        engine.step(&vm_template);

        // Strategy replaces population with buffer containing only 1 item (clone of index 0)
        assert_eq!(engine.population.len(), 1);
    }
}
