#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_orca_hgt_injection() {
        // Setup DNA with one strand containing a dummy gene
        let genes = vec![Gene {
            op: OpCode::Nop,
            args: vec![],
        }];
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.orca_mode = true;

        // Setup Injection Circuit
        // Layout:
        //   0 (Strand Index)
        //   |
        // "add" - { - (Signal)
        //   |
        //   0 (Gene Index)

        // Coordinates: Center at (5, 5)
        // West (5, 4): "add"
        // North (4, 5): 0
        // South (6, 5): 0
        // Center (5, 5): "{"
        // Signal at (5, 5): 1

        vm.grid[5][4] = Value::Str("add".to_string());
        vm.grid[4][5] = Value::Int(0);
        vm.grid[6][5] = Value::Int(0);
        vm.grid[5][5] = Value::Str("{".to_string());
        vm.signal_grid[5][5] = 1;

        // Run step
        vm.step();

        // Verify DNA Modification
        let modified_gene = &vm.dna.helix.strands[0].genes[0];
        assert_eq!(
            modified_gene.op,
            OpCode::Add,
            "Gene should be mutated to Add"
        );
    }

    #[test]
    fn test_orca_hgt_extraction() {
        // Setup DNA with one strand containing Sub
        let genes = vec![Gene {
            op: OpCode::Sub,
            args: vec![],
        }];
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.orca_mode = true;

        // Setup Extraction Circuit
        // Layout:
        //   0 (Gene Index)
        //   |
        // 0 - } - (Signal) -> Output (East)
        // (Strand Index)

        // Coordinates: Center at (10, 10)
        // West (10, 9): 0 (Strand)
        // North (9, 10): 0 (Gene)
        // Center (10, 10): "}"
        // Signal at (10, 10): 1

        vm.grid[10][9] = Value::Int(0);
        vm.grid[9][10] = Value::Int(0);
        vm.grid[10][10] = Value::Str("}".to_string());
        vm.signal_grid[10][10] = 1;

        // Run step
        vm.step();

        // Verify Grid Output at East (10, 11)
        match &vm.grid[10][11] {
            Value::Str(s) => assert_eq!(s, "sub", "Extracted OpCode should be 'sub'"),
            _ => panic!("Expected String output at (10, 11)"),
        }
    }
}
