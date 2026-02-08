#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_chronos_splice() {
        // Strand 0: [ Push(100) Sporulate() Push(0) Apoptosis() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Sporulate, args: vec![] }, // Creates Spore 0
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Apoptosis, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 200;

        // Run until halted
        while !vm.halted {
            vm.step();
        }

        // Verify Strand 0 is empty
        assert!(vm.dna.helix.strands[0].genes.is_empty());
        assert!(!vm.spores.is_empty());

        // Resurrect via ChronosSplice
        // Reset state to allow execution
        vm.halted = false;
        vm.ip = (0, 0);
        vm.inject_genes(vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Spore ID
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Strand ID in Spore
            Gene { op: OpCode::ChronosSplice, args: vec![] },
        ]);

        // Run miracle
        for _ in 0..5 {
            vm.step();
        }

        // Check if we have a new strand (Index 1)
        assert_eq!(vm.dna.helix.strands.len(), 2);

        // The new strand should be a copy of the original Strand 0
        let restored_strand = &vm.dna.helix.strands[1];
        assert_eq!(restored_strand.genes.len(), 4);
        assert_eq!(restored_strand.genes[0].op, OpCode::Push);
        if let Nucleotide::Number(n) = restored_strand.genes[0].args[0] {
            assert_eq!(n, 100);
        } else {
            panic!("Wrong arg");
        }

        // Verify stack has the new index
        // Note: inject_genes put 3 ops. Push, Push, ChronosSplice.
        // Stack should have result of ChronosSplice (new_idx)
        // Previous stack might have garbage?
        // Sporulate pushes spore_id (0).
        // Apoptosis doesn't push.
        // So stack might have [100, 0].
        // Then we inject.
        // Push(0), Push(0), ChronosSplice -> Pops 2, Pushes 1 (new index).
        // Result stack: [100, 0, 1].

        println!("DEBUG: Stack: {:?}", vm.stack);
        println!("DEBUG: Output: {:?}", vm.output);

        // Stack contains [..., new_strand_idx, 100] because the VM continued to execute the restored strand!
        let top = vm.stack.pop().unwrap();
        assert_eq!(top, Value::Int(100)); // Executed first instruction of restored strand

        let new_idx = vm.stack.pop().unwrap();
        assert_eq!(new_idx, Value::Int(1)); // Result of ChronosSplice
    }
}
