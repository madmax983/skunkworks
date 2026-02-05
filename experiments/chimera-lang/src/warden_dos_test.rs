#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::vm::{MAX_CALL_STACK_DEPTH, MAX_ORGANELLES, MAX_SPORES};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_dos_spore_bomb() {
        let mut genes = Vec::new();
        genes.push(Gene {
            op: OpCode::Sporulate,
            args: vec![],
        });
        for _ in 0..10 {
            genes.push(Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            });
        }
        genes.push(Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        });

        let mut vm = make_vm(genes);

        // Try to push unlimited spores
        for _ in 0..1000 {
            vm.energy = 1000;
            vm.execute_gene_inner(OpCode::Sporulate, &[]);
        }

        assert!(vm.spores.len() <= MAX_SPORES, "Spores should be capped");
        assert_eq!(vm.spores.len(), MAX_SPORES, "Should reach max spores");

        // Ensure one more doesn't add
        vm.energy = 1000;
        vm.execute_gene_inner(OpCode::Sporulate, &[]);
        assert_eq!(vm.spores.len(), MAX_SPORES);
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_dos_organelle_swarm() {
        // Spawn costs 20.
        let mut genes = Vec::new();
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // strand
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }); // type
        genes.push(Gene {
            op: OpCode::Spawn,
            args: vec![],
        });

        let mut vm = make_vm(genes);

        for _ in 0..1000 {
            vm.energy = 1000;
            vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(0)]);
            vm.execute_gene_inner(OpCode::Push, &[Nucleotide::Number(0)]);
            vm.execute_gene_inner(OpCode::Spawn, &[]);
        }

        assert!(
            vm.organelles.len() <= MAX_ORGANELLES,
            "Organelles should be capped"
        );
        assert_eq!(
            vm.organelles.len(),
            MAX_ORGANELLES,
            "Should reach max organelles"
        );
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_dos_reflex_storm() {
        // Strand 0: [ reflex(1, 0) irradiate(1, 10) ]
        // irradiate -> triggers mutation -> triggers reflex(1) -> calls strand 0
        // Infinite recursion of call stack.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // strand 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // event 1 (mutation)
            Gene {
                op: OpCode::Reflex,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // amount
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // radius
            Gene {
                op: OpCode::Irradiate,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);

        // Force mutation rate high implicitly via irradiate

        // Setup reflex
        vm.step(); // push 0
        vm.step(); // push 1
        vm.step(); // reflex

        // Check reflex registered
        assert_eq!(vm.reflexes.get(&1), Some(&0));

        // Now trigger the storm
        // Irradiate adds mutagen. Mutation happens in `process_environment` OR if chaos mode.
        // But `OpCode::Irradiate` itself doesn't trigger mutation directly.
        // `process_environment` does checks.

        // Let's manually inject mutagen to trigger it
        vm.mutagen_grid[8][8] = 100;

        // We need to execute step() which calls process_environment
        // AND randomness needs to trigger.
        // This is flaky to test deterministically because of RNG.

        // Alternative: call trigger_reflex recursively in a loop manually?
        // No, we want to test the protection.

        // Let's create a scenario where we manually call trigger_reflex in a loop
        // simulating a recursive chain.

        for _ in 0..MAX_CALL_STACK_DEPTH + 10 {
            vm.trigger_reflex(1);
        }

        assert!(
            vm.call_stack.len() <= MAX_CALL_STACK_DEPTH,
            "Call stack should be capped"
        );
        assert_eq!(
            vm.call_stack.len(),
            MAX_CALL_STACK_DEPTH,
            "Should reach max call stack"
        );
    }
}
