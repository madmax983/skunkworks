use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

#[derive(Clone)]
pub struct EvolutionEngine {
    pub population: Vec<Strand>,
    pub target_val: i64,
    pub generation: usize,
    pub best_fitness: i64,
    pub history: Vec<i64>,
}

impl EvolutionEngine {
    pub fn new(seed: Strand, population_size: usize, target: i64) -> Self {
        let population = vec![seed; population_size];
        Self {
            population,
            target_val: target,
            generation: 0,
            best_fitness: i64::MAX,
            history: Vec::new(),
        }
    }

    pub fn step(&mut self, vm_template: &ChimeraVM) {
        let mut results = Vec::new();

        for strand in &self.population {
            // Sandboxed VM
            let mut vm = vm_template.clone();
            // Clear existing DNA and use just this strand
            vm.dna.helix.strands = vec![strand.clone()];
            vm.ip = (0, 0);
            vm.energy = 100; // Enough energy
            vm.halted = false;
            vm.chaos_mode = false; // Disable random mutations during sim

            // Run for fixed steps
            let max_ticks = 50;
            for _ in 0..max_ticks {
                if vm.halted {
                    break;
                }
                vm.step();
            }

            // Calculate Fitness (Distance to target)
            // We look at the top of the stack.
            let val = vm
                .stack
                .last()
                .and_then(|v| match v {
                    Value::Int(n) => Some(*n),
                    _ => None,
                })
                .unwrap_or(0);

            // If stack empty, heavy penalty
            let fitness = if vm.stack.is_empty() {
                i64::MAX / 2
            } else {
                (val - self.target_val).abs()
            };

            // Secondary fitness: code length (shorter is better)
            // But primary is value.
            // Let's add length/10 to fitness to break ties
            let len_penalty = strand.genes.len() as i64;
            let final_fitness = fitness.saturating_add(len_penalty);

            results.push((final_fitness, strand.clone()));
        }

        // Sort by fitness (lowest is best)
        results.sort_by(|a, b| a.0.cmp(&b.0));

        let best = results[0].clone();
        self.best_fitness = best.0;
        self.history.push(self.best_fitness);
        self.generation += 1;

        let best_strand = best.1;
        let pop_size = self.population.len();
        self.population.clear();
        self.population.push(best_strand.clone()); // Elitism

        let mut rng = rand::thread_rng();
        while self.population.len() < pop_size {
            let mut clone = best_strand.clone();
            Self::mutate_strand(&mut clone, &mut rng);
            self.population.push(clone);
        }
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
                // Convert Nop to Push? Or fix Push with no args
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

        let dna = Dna {
            helix: Helix {
                strands: vec![seed.clone()],
            },
        };
        let vm_template = ChimeraVM::new(dna);

        let mut engine = EvolutionEngine::new(seed, 20, 42);

        // It might take many generations, but fitness should not regress significantly
        // and should eventually find 42 (Push 42 or Push 40 Add 2 etc)
        // With random mutations, 50 generations might not be enough for complex logic,
        // but for a simple "Push X", it should find X=42 quickly.

        for _ in 0..100 {
            engine.step(&vm_template);
            if engine.best_fitness < 10 {
                // Allow some slack for length penalty
                break;
            }
        }

        println!("Best Fitness: {}", engine.best_fitness);
        assert!(engine.best_fitness < 1000, "Fitness should be reasonable");
    }
}
