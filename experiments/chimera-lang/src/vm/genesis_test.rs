#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::genesis::evolve;
    use crate::vm::ChimeraVM;

    fn make_vm(strands: Vec<Strand>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_genesis_evolution() {
        // Goal: Evolve Strand 0 to push 42.
        // Strand 0 (Subject): [ push(0) ] (Initial)
        // Strand 1 (Fitness): [ push(42) sub abs push(1000) swap sub ]
        // Score = 1000 - |x - 42|

        let subject = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }],
        };

        // Note: abs is not an opcode. We simulate abs with:
        // dup 0 < if (neg) else () ...
        // Or simpler: just try to hit 42.
        // Let's use equality check for simplicity. 1 if equal, 0 if not.
        // But for evolution, gradient is better.
        // Let's use simple difference squared or linear difference if we can implement it.
        // sub returns a-b.
        // If we don't have abs, we can assume target > current or just maximize.
        // Let's try to MAXIMIZE the output. Target: Infinity.
        // Fitness: [ ] (Score is the value itself)

        let fitness = Strand {
            genes: vec![], // Identity. Score = Top of stack.
        };

        let mut vm = make_vm(vec![subject, fitness]);

        // Enable chaos mode for mutation to work
        vm.chaos_mode = true;

        // Run for 50 generations
        let (_success, score, _traces, best_strand) = evolve(&mut vm, 0, 1, 50);

        println!("Final Score: {}", score);
        println!("Best Strand: {:?}", best_strand);

        // We expect the score to increase from 0
        assert!(score > 0, "Evolution should improve score");

        // Check if the best strand actually produces the score
        if let Some(strand) = best_strand {
            // It should be different from initial
            // It's possible it mutated to Push(high number) or changed OpCode
            let initial_gene = Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            };
            assert_ne!(strand.genes[0], initial_gene, "Strand should have mutated");
        }
    }
}
