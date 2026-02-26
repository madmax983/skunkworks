#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::{ChimeraVM, MAX_GENES_PER_STRAND, MAX_STRANDS};

    #[test]
    fn test_chain_dos() {
        // Create a VM
        // We need a strand with > 2048 genes to chain with itself > 4096.
        let mut genes = Vec::new();
        for _ in 0..2500 {
            genes.push(Gene { op: OpCode::Nop, args: vec![] });
        }

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Push strand indices (0, 0)
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(0));

        // Execute Chain
        // Inject OpCode
        vm.dna.helix.strands.push(Strand {
            genes: vec![Gene { op: OpCode::Chain, args: vec![] }]
        });
        // Jump to execution strand
        vm.ip = (1, 0);

        vm.step();

        // Check result
        // If vulnerable, a new strand exists with 5000 genes.
        // If secure, no new strand or error.

        let strand_count = vm.dna.helix.strands.len();
        if strand_count > 2 {
            let new_strand = &vm.dna.helix.strands[strand_count - 1];
            assert!(
                new_strand.genes.len() <= MAX_GENES_PER_STRAND,
                "SECURITY FAILURE: Strand length {} exceeds limit {}",
                new_strand.genes.len(),
                MAX_GENES_PER_STRAND
            );
        }
    }
}
