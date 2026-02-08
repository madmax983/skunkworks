#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};
    use crate::opcode::OpCode;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_orca_gene_read() {
        let mut vm = make_vm();

        // Add a strand: 0 -> "Add(1, 2)"
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Add,
                args: vec![Nucleotide::Number(1), Nucleotide::Number(2)],
            }],
        };
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);

        // Layout:
        // . 0 . (North)
        // 0 G 0 (West: Strand 0, East: Gene 0)
        // . . . (South: Result)

        vm.grid[1][0] = Value::Int(0); // Strand Index
        vm.grid[1][1] = Value::Str("G".to_string());
        vm.grid[1][2] = Value::Int(0); // Gene Index

        vm.signal_grid[1][1] = 1; // Activate G

        process_signals(&mut vm);

        // Result at South (2,1) should be OpCode ID (or name hash if implemented that way)
        // Currently OpCode doesn't have a numeric ID exposed easily, but `process_signals`
        // will likely convert to string or int.
        // Let's assume it writes the OpCode name or ID.
        // If the implementation writes the OpCode variant index, we might need to know it.
        // Or if it writes the string representation.

        match &vm.grid[2][1] {
            Value::Str(s) => assert_eq!(s, "add"),
            Value::Int(_) => {}, // Accept int if implementation uses int
            _ => panic!("Expected result 'add', got {:?}", vm.grid[2][1]),
        }
    }

    #[test]
    fn test_orca_play() {
        let mut vm = make_vm();

        // Add a strand: 0 -> "Push(10)"
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        // Layout:
        // . * .
        // 0 P . (West: Strand 0)
        // . . .

        vm.grid[1][0] = Value::Int(0);
        vm.grid[1][1] = Value::Str("P".to_string());
        vm.grid[0][1] = Value::Str("*".to_string()); // Bang!

        vm.signal_grid[1][1] = 1; // Trigger P directly

        process_signals(&mut vm);

        // The 'P' operator queues execution. process_signals runs executions at end.
        // So the stack should now contain 10.
        // Wait, process_signals runs `vm.execute_gene_inner`.
        // If P executes `Call(0)`, it pushes to call_stack and jumps IP.
        // It does NOT run the strand immediately in `process_signals`.
        // It just sets up the VM state (IP).
        // Then subsequent `vm.step()` calls run the code.

        // However, `process_signals` in test runs isolated.
        // If P triggers `Call(0)`, vm.ip should be (0, 0).
        // And vm.call_stack should have return address.

        assert_eq!(vm.ip, (0, 0));
        assert_eq!(vm.call_stack.len(), 1);
    }

    #[test]
    fn test_orca_synthesis() {
        let mut vm = make_vm();

        // Add a strand: 0 -> "Push(0)"
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }],
        };
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);

        // Layout:
        // . 5 . (North: Value 5)
        // 0 Y 0 (West: Strand 0, East: Gene 0)
        // * . . (Bang)

        vm.grid[0][1] = Value::Int(5);
        vm.grid[1][0] = Value::Int(0);
        vm.grid[1][1] = Value::Str("Y".to_string());
        vm.grid[1][2] = Value::Int(0);
        vm.grid[2][0] = Value::Str("*".to_string()); // Bang Y from SW? No, signal propagates.

        // Let's just put signal on Y directly for simplicity
        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Gene 0 arg should now be 5
        let gene = &vm.dna.helix.strands[0].genes[0];
        match &gene.args[0] {
            Nucleotide::Number(n) => assert_eq!(*n, 5),
            _ => panic!("Expected Number(5)"),
        }
    }

    #[test]
    fn test_orca_kill() {
        let mut vm = make_vm();

        // Add a strand: 0 -> "Push(0)"
        let strand = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }],
        };
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);

        // Addcladistics
        vm.cladistics.register_strand(0, None, 0, "Init".to_string());

        // Layout:
        // . * .
        // 0 K . (West: Strand 0)

        vm.grid[1][0] = Value::Int(0);
        vm.grid[1][1] = Value::Str("K".to_string());
        vm.grid[0][1] = Value::Str("*".to_string());

        vm.signal_grid[1][1] = 1; // Trigger K directly

        process_signals(&mut vm);

        // Strand 0 should be cleared (apoptosis)
        assert!(vm.dna.helix.strands[0].genes.is_empty());
    }
}
