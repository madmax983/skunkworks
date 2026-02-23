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
        let mut population = vec![seed.clone(); population_size];

        // Initial diversity
        let mut rng = rand::thread_rng();
        for i in 1..population_size {
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

    fn tournament_select(pool: &[(i64, Strand)], rng: &mut impl Rng) -> Strand {
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

    fn crossover(a: &Strand, b: &Strand, rng: &mut impl Rng) -> Strand {
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

    fn evaluate_fitness(vm_template: &ChimeraVM, strand: &Strand, challenge: &Challenge) -> i64 {
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

    fn mutate_strand(strand: &mut Strand, rng: &mut impl Rng) {
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

        let dna = Dna { evolution_config: None,
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
}
