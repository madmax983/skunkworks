#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.metamorphism_enabled = false;
        vm
    }

    #[test]
    #[ignore]
    fn test_orca_harvest_success() {
        let mut vm = make_vm();
        // Layout:
        // . 2 .  (Offset Y=2)
        // 0 H 3  (Strand 0, Harvest, Length 3)
        // . . .
        // . a d d (Code at y+2)

        vm.grid[0][1] = Value::Str("2".to_string());
        vm.grid[1][0] = Value::Str("0".to_string());
        vm.grid[1][1] = Value::Str("H".to_string());
        vm.grid[1][2] = Value::Str("3".to_string());

        // Write "add" starting at (3, 1)
        vm.grid[3][1] = Value::Str("a".to_string());
        vm.grid[3][2] = Value::Str("d".to_string());
        vm.grid[3][3] = Value::Str("d".to_string());

        // Signal H
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check Strand 0
        let strand = &vm.dna.helix.strands[0];
        assert_eq!(strand.genes.len(), 1);
        assert_eq!(strand.genes[0].op, OpCode::Add);

        // Check Output (South of H)
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected '1' at (2,1)"),
        }
    }

    #[test]
    #[ignore]
    fn test_orca_harvest_unknown() {
        let mut vm = make_vm();
        // Layout:
        // . 1 .
        // 0 H 3
        // . f o o  (foo -> Unknown)

        vm.grid[0][1] = Value::Str("1".to_string());
        vm.grid[1][0] = Value::Str("0".to_string());
        vm.grid[1][1] = Value::Str("H".to_string());
        vm.grid[1][2] = Value::Str("3".to_string());

        vm.grid[2][1] = Value::Str("f".to_string());
        vm.grid[2][2] = Value::Str("o".to_string());
        vm.grid[2][3] = Value::Str("o".to_string());

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        let strand = &vm.dna.helix.strands[0];
        assert_eq!(strand.genes.len(), 1);
        if let OpCode::Unknown(s) = &strand.genes[0].op {
            assert_eq!(s, "foo");
        } else {
            panic!("Expected Unknown(foo), got {:?}", strand.genes[0].op);
        }

        // Output should still be success
        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "1"),
            _ => panic!("Expected '1' at (2,1)"),
        }
    }
}
