#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_project_geometry() {
        let mut vm = make_vm();

        // Create a strand with some genes
        // [ push(10) add ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Add, args: vec![] },
        ];

        vm.dna.helix.strands.push(Strand { genes });
        let strand_idx = vm.dna.helix.strands.len() - 1;

        // Push args: strand_idx, y=8, x=8
        vm.stack.push(Value::Int(strand_idx as i64));
        vm.stack.push(Value::Int(8));
        vm.stack.push(Value::Int(8));

        // Execute ProjectGeometry via gene injection
        vm.dna.helix.strands[0].genes.push(Gene {
            op: OpCode::ProjectGeometry,
            args: vec![],
        });

        // Run step
        vm.step();

        // Check grid
        // Spiral: (8,8), (8,9)
        // (8,8) should have Push(10) -> Int(10)
        // (8,9) should have Add -> Str("add")

        assert_eq!(vm.grid[8][8], Value::Int(10));
        assert_eq!(vm.grid[8][9], Value::Str("add".to_string()));
    }

    #[test]
    fn test_absorb_geometry() {
        let mut vm = make_vm();

        // Setup grid manually
        // Spiral: (8,8) -> 42, (8,9) -> "sub"
        vm.grid[8][8] = Value::Int(42);
        vm.grid[8][9] = Value::Str("sub".to_string());

        // Push args: radius=1, y=8, x=8
        vm.stack.push(Value::Int(1)); // Radius
        vm.stack.push(Value::Int(8)); // y
        vm.stack.push(Value::Int(8)); // x

        // Execute AbsorbGeometry via gene
        vm.dna.helix.strands[0].genes.push(Gene {
            op: OpCode::AbsorbGeometry,
            args: vec![],
        });

        vm.step();

        // Result should be new strand idx on stack
        assert_eq!(vm.stack.len(), 1);
        let new_idx = match vm.stack.pop().unwrap() {
            Value::Int(i) => i as usize,
            _ => panic!("Expected Int index"),
        };

        assert!(new_idx < vm.dna.helix.strands.len());
        let new_strand = &vm.dna.helix.strands[new_idx];

        // Check genes
        // 0: Push(42)
        // 1: Sub
        // Note: Spiral reads (2*r+1)^2 cells. Radius 1 = 9 cells.
        // We only set 2. The rest are 0 (Int(0)).
        // Int(0) -> Push(0).

        assert_eq!(new_strand.genes[0].op, OpCode::Push);
        match &new_strand.genes[0].args[0] {
            Nucleotide::Number(n) => assert_eq!(*n, 42),
            _ => panic!("Expected Number 42"),
        }

        assert_eq!(new_strand.genes[1].op, OpCode::Sub);

        // Check rest are Push(0)
        for i in 2..9 {
             assert_eq!(new_strand.genes[i].op, OpCode::Push);
             match &new_strand.genes[i].args[0] {
                Nucleotide::Number(n) => assert_eq!(*n, 0),
                _ => panic!("Expected Number 0"),
            }
        }
    }
}
