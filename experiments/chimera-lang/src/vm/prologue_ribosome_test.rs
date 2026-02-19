#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_polymerase_rune() {
        let mut vm = setup_vm();
        // Setup: List of Genes -> ! -> p
        // Genes: [ push(10), add ]
        // Representation: [ Junction(Dish, ["push", 10]), "add" ]

        let push_gene = Value::Junction(
            JunctionType::Dish,
            vec![Value::Str("push".to_string()), Value::Int(10)],
        );
        let add_gene = Value::Str("add".to_string());
        let gene_list = Value::Junction(JunctionType::Dish, vec![push_gene, add_gene]);

        // ! reads West. So place list at (5, 3).
        vm.grid[5][3] = gene_list;
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("p".to_string());

        exec_prologue_tick(&mut vm);

        // Check if p outputted 0 (new strand index)
        let output = &vm.prologue_state.signal_grid[5][5];
        assert_eq!(*output, Some(Value::Int(0)));

        // Check DNA
        assert_eq!(vm.dna.helix.strands.len(), 1);
        let strand = &vm.dna.helix.strands[0];
        assert_eq!(strand.genes.len(), 2);
        assert_eq!(strand.genes[0].op, OpCode::Push);
        assert_eq!(strand.genes[0].args[0], Nucleotide::Number(10));
        assert_eq!(strand.genes[1].op, OpCode::Add);
    }

    #[test]
    fn test_operon_rune() {
        let mut vm = setup_vm();
        // Use Index 1 to avoid '!' source suppression of 0
        // Dummy Strand 0
        vm.dna.helix.strands.push(Strand { genes: vec![] });

        // Strand 1: [ sub ]
        let genes = vec![Gene {
            op: OpCode::Sub,
            args: vec![],
        }];
        vm.dna.helix.strands.push(Strand { genes });

        // Setup: 1 (Idx) -> ! -> o
        vm.grid[5][3] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("o".to_string());

        // Operon prints to South (6, 5) then East
        exec_prologue_tick(&mut vm);

        // Check Grid
        let cell = &vm.grid[6][5];
        assert_eq!(*cell, Value::Str("sub".to_string()));
    }

    #[test]
    fn test_operon_complex_rune() {
        let mut vm = setup_vm();
        // Dummy Strand 0
        vm.dna.helix.strands.push(Strand { genes: vec![] });

        // Setup DNA: Strand 1 = [ push(42) ]
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        }];
        vm.dna.helix.strands.push(Strand { genes });

        // Setup: 1 (Idx) -> ! -> o
        vm.grid[5][3] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("o".to_string());

        exec_prologue_tick(&mut vm);

        // Check Grid at (6, 5)
        let cell = &vm.grid[6][5];
        // Expect Junction(Dish, ["push", 42])
        if let Value::Junction(_, items) = cell {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], Value::Str("push".to_string()));
            assert_eq!(items[1], Value::Int(42));
        } else {
            panic!("Expected Junction, got {:?}", cell);
        }
    }

    #[test]
    fn test_central_dogma_cycle() {
        // DNA -> Grid -> DNA
        // 1. Create Strand 1: [ add ]
        // 2. Operon prints it to grid
        // 3. Collect [ rune reads it into list
        // 4. Polymerase reads list and creates Strand 2
        // 5. Verify Strand 2 == Strand 1

        let mut vm = setup_vm();
        vm.dna.helix.strands.push(Strand { genes: vec![] }); // Dummy 0

        let genes = vec![Gene {
            op: OpCode::Add,
            args: vec![],
        }];
        vm.dna.helix.strands.push(Strand { genes });

        // Setup O: 1 -> ! -> o
        vm.grid[5][3] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("o".to_string());

        // Tick 1: o prints "add" to (6, 5)
        exec_prologue_tick(&mut vm);
        assert_eq!(vm.grid[6][5], Value::Str("add".to_string()));

        // Setup P
        // Manually plumb output of O to input of P for testing
        let grid_val = vm.grid[6][5].clone(); // "add"
        let list = Value::Junction(JunctionType::Dish, vec![grid_val]);

        // P input at (7, 6), ! at (7, 7), p at (7, 8)
        vm.grid[7][6] = list;
        vm.grid[7][7] = Value::Str("!".to_string());
        vm.grid[7][8] = Value::Str("p".to_string());

        exec_prologue_tick(&mut vm);

        // Check new strand
        assert_eq!(vm.dna.helix.strands.len(), 3);
        let s2 = &vm.dna.helix.strands[2];
        assert_eq!(s2.genes.len(), 1);
        assert_eq!(s2.genes[0].op, OpCode::Add);
    }
}
