#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    fn make_gene(op: OpCode, args: Vec<i64>) -> Gene {
        Gene {
            op,
            args: args.into_iter().map(Nucleotide::Number).collect(),
        }
    }

    #[test]
    fn test_sense_pigment() {
        let genes = vec![
            make_gene(OpCode::Push, vec![10]), // r
            make_gene(OpCode::Push, vec![20]), // g
            make_gene(OpCode::Push, vec![30]), // b
            make_gene(OpCode::Push, vec![8]),  // y
            make_gene(OpCode::Push, vec![8]),  // x
            make_gene(OpCode::Pigment, vec![]),
            make_gene(OpCode::SensePigment, vec![]),
        ];

        let mut vm = make_vm(genes);
        // 5 pushes, 1 pigment, 1 sense
        for _ in 0..7 {
            vm.step();
        }

        // Stack should be [30, 20, 10] (Top is B) because SensePigment pushes R, G, B
        assert_eq!(vm.stack.pop(), Some(Value::Int(30))); // B
        assert_eq!(vm.stack.pop(), Some(Value::Int(20))); // G
        assert_eq!(vm.stack.pop(), Some(Value::Int(10))); // R
    }

    #[test]
    fn test_sense_glyph() {
        let genes = vec![
            make_gene(OpCode::Push, vec![65]),
            make_gene(OpCode::Push, vec![8]),
            make_gene(OpCode::Push, vec![8]),
            make_gene(OpCode::Glyph, vec![]),
            make_gene(OpCode::SenseGlyph, vec![]),
        ];

        let mut vm = make_vm(genes);
        for _ in 0..5 {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Int(65)));
    }

    #[test]
    fn test_sense_nothing() {
        let genes = vec![
            make_gene(OpCode::SensePigment, vec![]),
            make_gene(OpCode::SenseGlyph, vec![]),
        ];
        let mut vm = make_vm(genes);
        vm.step(); // SensePigment -> pushes 0, 0, 0
        vm.step(); // SenseGlyph -> pushes -1

        assert_eq!(vm.stack.pop(), Some(Value::Int(-1))); // Glyph
        assert_eq!(vm.stack.pop(), Some(Value::Int(0))); // B
        assert_eq!(vm.stack.pop(), Some(Value::Int(0))); // G
        assert_eq!(vm.stack.pop(), Some(Value::Int(0))); // R
    }
}
