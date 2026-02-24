#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::memetics::{Meme, MemePool};
    use chimera_lang::vm::{ChimeraVM, Value, MAX_GENES_PER_STRAND};

    fn make_dna() -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        }
    }

    #[test]
    fn test_propagate_unbounded_growth() {
        let mut vm = ChimeraVM::new(make_dna());

        // Add a strand (Target)
        vm.dna.helix.strands.push(Strand { genes: vec![] }); // Strand 1

        // Create a Meme with many genes
        let mut genes = Vec::new();
        for _ in 0..100 {
            genes.push(Gene {
                op: OpCode::Nop,
                args: vec![],
            });
        }

        let meme = Meme {
            genes,
            virulence: 100, // Always infect
            fidelity: 100,  // No mutation
            description: "Bloat".to_string(),
        };
        vm.meme_pool.memes.push(meme);

        // Try to propagate repeatedly until it exceeds limit (if limit existed)
        // We simulate the loop here

        for _ in 0..200 {
            // 200 * 100 = 20000 genes > 4096 limit
            vm.stack.push(Value::Int(0)); // Meme ID 0
            vm.stack.push(Value::Int(1)); // Target Strand 1

            // vm.execute_gene handles tracking and calling inner
            // But we need to call with OpCode::Propagate

            // We can use execute_gene directly on the vm
            // But since we are in integration test, execute_gene is private?
            // No, execute_gene is private in vm/mod.rs.
            // But we can construct a gene and run step()?

            // Better: vm.execute_gene_inner is private.
            // But we can put the gene in the strand and run step().

            let propagate_gene = Gene {
                op: OpCode::Propagate,
                args: vec![],
            };

            // Inject gene into strand 0
            vm.dna.helix.strands[0].genes = vec![propagate_gene];
            vm.ip = (0, 0);

            vm.step();
        }

        // Check size
        let len = vm.dna.helix.strands[1].genes.len();
        println!("Strand length: {}", len);

        assert!(
            len <= MAX_GENES_PER_STRAND,
            "Strand grew beyond MAX_GENES_PER_STRAND"
        );
    }
}
