#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_polyglot_antonym() {
        // Strand 0: [ Push(10) Push(0) Polyglot ]
        // Strand 1 (Target): [ Add, Mul, Mitosis ]
        // After Polyglot(0, 1), we expect a new Strand 2 with [ Sub, Div, Apoptosis ]

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Dialect 0 (Antonym)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Target Strand Index
                Gene { op: OpCode::Polyglot, args: vec![] },
            ]
        };

        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Add, args: vec![] },
                Gene { op: OpCode::Mul, args: vec![] },
                Gene { op: OpCode::Mitosis, args: vec![] },
            ]
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Execute Strand 0
        vm.step(); // Push 1
        vm.step(); // Push 0
        vm.step(); // Polyglot

        // Check if Strand 2 exists
        assert_eq!(vm.dna.helix.strands.len(), 3);

        let new_strand = &vm.dna.helix.strands[2];
        assert_eq!(new_strand.genes.len(), 3);
        assert_eq!(new_strand.genes[0].op, OpCode::Sub); // Antonym of Add
        assert_eq!(new_strand.genes[1].op, OpCode::Div); // Antonym of Mul
        assert_eq!(new_strand.genes[2].op, OpCode::Apoptosis); // Antonym of Mitosis
    }

    #[test]
    fn test_polyglot_chaotic() {
        // Dialect 1 (Chaotic)
        // Push -> Scramble, Jump -> QuantumTunnel, GRead -> Entropy

        let strand0 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Dialect 1
                Gene { op: OpCode::Polyglot, args: vec![] },
            ]
        };

        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
                Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::GRead, args: vec![] },
            ]
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.dna.helix.strands.len(), 3);
        let new_strand = &vm.dna.helix.strands[2];

        assert_eq!(new_strand.genes[0].op, OpCode::Scramble);
        assert_eq!(new_strand.genes[1].op, OpCode::QuantumTunnel);
        assert_eq!(new_strand.genes[2].op, OpCode::Entropy);
    }
}
