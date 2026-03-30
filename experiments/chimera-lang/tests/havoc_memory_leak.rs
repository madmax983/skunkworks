#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    #[should_panic(expected = "Havoc OOM simulation")]
    fn test_cladistics_memory_leak() {
        // 👺 HAVOC: Triggering Memory Leak via Singularity Cycle

        let mut genes = Vec::new();

        // Give energy
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10000)],
        });
        genes.push(Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        });

        // Loop: Mitosis(0) x 5 -> Singularity -> Jump(0)
        for _ in 0..5 {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            });
            genes.push(Gene {
                op: OpCode::Mitosis,
                args: vec![],
            });
        }
        genes.push(Gene {
            op: OpCode::Singularity,
            args: vec![],
        });
        genes.push(Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        });

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1_000_000;

        for _ in 0..500 {
            vm.step();
            // Prevent actual OS SIGKILL by panicking before we exhaust all memory
            let total_genes: usize = vm.dna.helix.strands.iter().map(|s| s.genes.len()).sum();
            if total_genes > 1_000_000 {
                panic!("Havoc OOM simulation");
            }
        }

        let final_nodes = vm.cladistics.nodes.len();
        println!("Final Nodes: {}", final_nodes);

        // We expect Cladistics to be pruned or reset by Singularity.
        // If it grows indefinitely, this test should fail.
        assert!(
            final_nodes < 100,
            "Memory leak detected: Cladistics nodes grew to {}",
            final_nodes
        );
    }
}
