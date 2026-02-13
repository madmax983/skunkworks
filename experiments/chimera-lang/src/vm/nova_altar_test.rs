#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;
    use crate::vm::nova_altar::perform_ritual;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_ritual_splicing() {
        let mut vm = make_vm();

        // Setup two parent strands
        let s1 = Strand {
            genes: vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }],
        };
        let s2 = Strand {
            genes: vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }],
        };

        vm.dna.helix.strands.push(s1);
        vm.dna.helix.strands.push(s2);

        // Sufficient energy
        vm.energy = 100;

        let result = perform_ritual(&mut vm, 0, 1, 10);
        assert!(result.is_ok());

        // Should have 3 strands now
        assert_eq!(vm.dna.helix.strands.len(), 3);
        let child = &vm.dna.helix.strands[2];
        assert_eq!(child.genes.len(), 1); // Max len of 1 and 1 is 1

        // Energy consumed
        assert_eq!(vm.energy, 90);
    }

    #[test]
    fn test_ritual_insufficient_energy() {
        let mut vm = make_vm();
        vm.energy = 5;
        let result = perform_ritual(&mut vm, 0, 0, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_ritual_chaos_mutation() {
        let mut vm = make_vm();
        let s1 = Strand {
            genes: vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }],
        };
        vm.dna.helix.strands.push(s1.clone());
        vm.dna.helix.strands.push(s1);

        vm.energy = 1000;

        // High sacrifice -> High Chaos
        let _ = perform_ritual(&mut vm, 0, 1, 90);

        // Check if mutation occurred (probabilistic, but likely with high chaos)
        let child = &vm.dna.helix.strands[2];
        // At 90 sacrifice, chaos is 0.9.
        // It inserts "Push(90)" and "Photosynthesize" at start due to high sacrifice bonus.
        assert!(child.genes.len() >= 3);
        assert_eq!(child.genes[0].op, OpCode::Push);
        if let Nucleotide::Number(n) = child.genes[0].args[0] {
            assert_eq!(n, 90);
        } else {
            panic!("Expected bonus gene");
        }
    }
}
