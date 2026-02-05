#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::nova::OrganelleType;

    fn make_gene(op: OpCode, val: Option<i64>) -> Gene {
        Gene {
            op,
            args: if let Some(v) = val {
                vec![Nucleotide::Number(v)]
            } else {
                vec![]
            },
        }
    }

    #[test]
    fn test_differentiation() {
        // Strand 0: Spawn(Type=0/Worker, Strand=1) -> Infinite Loop
        let strand0 = Strand {
            genes: vec![
                make_gene(OpCode::Push, Some(1)), // Strand 1
                make_gene(OpCode::Push, Some(0)), // Type 0 (Worker)
                make_gene(OpCode::Spawn, None),
                make_gene(OpCode::Jump, Some(0)), // Keep VM alive
            ],
        };

        // Strand 1: Check Identity(0) -> If OK, Jump(2). Else Error.
        let strand1 = Strand {
            genes: vec![
                make_gene(OpCode::Identity, None),
                make_gene(OpCode::Push, Some(0)),
                make_gene(OpCode::Sub, None),
                make_gene(OpCode::Brz, Some(2)),
                make_gene(OpCode::Push, Some(999)), // Error
                make_gene(OpCode::Jump, Some(99)),
            ],
        };

        // Strand 2: Differentiate(Type=1/Chloroplast) -> Jump(3)
        let strand2 = Strand {
            genes: vec![
                make_gene(OpCode::Push, Some(1)), // Type 1 (Chloroplast)
                make_gene(OpCode::Differentiate, None),
                make_gene(OpCode::Jump, Some(3)),
            ],
        };

        // Strand 3: Check Identity(1) -> If OK, Jump(4). Else Error.
        let strand3 = Strand {
            genes: vec![
                make_gene(OpCode::Identity, None),
                make_gene(OpCode::Push, Some(1)),
                make_gene(OpCode::Sub, None),
                make_gene(OpCode::Brz, Some(4)),
                make_gene(OpCode::Push, Some(888)), // Error
                make_gene(OpCode::Jump, Some(99)),
            ],
        };

        // Strand 4: Success -> Loop
        let strand4 = Strand {
            genes: vec![
                make_gene(OpCode::Push, Some(777)),
                make_gene(OpCode::Jump, Some(4)), // Keep Organelle alive
            ],
        };

        // Strand 99: Error Sink (Empty)
        let strand99 = Strand { genes: vec![make_gene(OpCode::Jump, Some(99))] };

        // Fill gaps with empty strands
        let mut strands = vec![strand0, strand1, strand2, strand3, strand4];
        while strands.len() < 99 {
            strands.push(Strand { genes: vec![] });
        }
        strands.push(strand99);

        let dna = Dna {
            helix: Helix { strands },
        };
        let mut vm = ChimeraVM::new(dna);

        // Energy for differentiation
        vm.energy = 1000;

        // Run
        for _ in 0..100 {
            vm.step();
        }

        // Check
        assert!(!vm.organelles.is_empty(), "Organelle should exist");
        let organelle = &vm.organelles[0];

        // Check final type
        assert_eq!(organelle.kind, OrganelleType::Chloroplast, "Should have differentiated to Chloroplast");

        // Check stack for success marker
        assert!(organelle.stack.iter().any(|v| matches!(v, Value::Int(777))), "Stack should contain 777. Stack: {:?}", organelle.stack);
    }
}
