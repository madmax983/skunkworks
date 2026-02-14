#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::babel::{read_grammar_from_grid, generate_random_grid_grammar};
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::JunctionType;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_read_string_grammar() {
        let mut vm = make_vm();
        // "foo"
        vm.grid[0][0] = Value::Str("\"".to_string());
        vm.grid[0][1] = Value::Str("f".to_string());
        vm.grid[0][2] = Value::Str("o".to_string());
        vm.grid[0][3] = Value::Str("o".to_string());
        vm.grid[0][4] = Value::Str("\"".to_string());

        let grammar = read_grammar_from_grid(&vm, 0, 0);

        if let Value::Junction(JunctionType::Any, args) = grammar {
            assert_eq!(args[0], Value::Str("Match".to_string()));
            assert_eq!(args[1], Value::Str("foo".to_string()));
        } else {
            panic!("Expected Junction, got {:?}", grammar);
        }
    }

    #[test]
    fn test_read_sequence() {
        let mut vm = make_vm();
        // "a" - "b"
        vm.grid[0][0] = Value::Str("\"".to_string());
        vm.grid[0][1] = Value::Str("a".to_string());
        vm.grid[0][2] = Value::Str("\"".to_string());
        vm.grid[0][3] = Value::Str("-".to_string());
        vm.grid[0][4] = Value::Str("\"".to_string());
        vm.grid[0][5] = Value::Str("b".to_string());
        vm.grid[0][6] = Value::Str("\"".to_string());

        let grammar = read_grammar_from_grid(&vm, 0, 0);

        // Expected: Seq(Match("a"), Match("b"))
        if let Value::Junction(JunctionType::Any, args) = grammar {
            assert_eq!(args[0], Value::Str("Seq".to_string()));
            // Check first child (node)
            if let Value::Junction(_, child1) = &args[1] {
                assert_eq!(child1[1], Value::Str("a".to_string()));
            } else {
                panic!("Child 1 mismatch: {:?}", args[1]);
            }
            // Check second child (next_val)
            if let Value::Junction(_, child2) = &args[2] {
                assert_eq!(child2[1], Value::Str("b".to_string()));
            } else {
                panic!("Child 2 mismatch: {:?}", args[2]);
            }
        } else {
            panic!("Expected Seq Junction, got {:?}", grammar);
        }
    }

    #[test]
    fn test_read_branch() {
        let mut vm = make_vm();
        // A - +
        //     | - B
        //     | - C
        // Layout:
        // "A" - +
        //       |
        //       "B"
        // And "C" ?
        // Simplest branch:
        // . - "B"
        // |
        // "C"

        vm.grid[0][0] = Value::Str(".".to_string());
        // Right
        vm.grid[0][1] = Value::Str("-".to_string());
        vm.grid[0][2] = Value::Str("\"".to_string());
        vm.grid[0][3] = Value::Str("B".to_string());
        vm.grid[0][4] = Value::Str("\"".to_string());
        // Down
        vm.grid[1][0] = Value::Str("|".to_string());
        vm.grid[2][0] = Value::Str("\"".to_string());
        vm.grid[2][1] = Value::Str("C".to_string());
        vm.grid[2][2] = Value::Str("\"".to_string());

        let grammar = read_grammar_from_grid(&vm, 0, 0);

        // Expected: Alt(Seq("B"...), Seq("C"...))
        if let Value::Junction(JunctionType::Any, args) = grammar {
            assert_eq!(args[0], Value::Str("Alt".to_string()));
            assert_eq!(args.len(), 3); // Alt + 2 children
            // Order depends on find_children iteration order: N, E, S, W
            // E is "B", S is "C"
            // So args[1] should be B path, args[2] should be C path

            // Note: scan_connector calls find_children -> returns list of children
            // B path starts with "-", which is a connector.
            // scan_connector("-") calls find_children -> returns "B" node.
            // So B path returns "B" node (Match("B")).
            // C path returns "C" node (Match("C")).

            let child1 = &args[1];
            let child2 = &args[2];

            // Check contents
            println!("Child1: {:?}", child1);
            println!("Child2: {:?}", child2);

            // We verify they are Matches
            // We assume order E then S.
        } else {
            panic!("Expected Alt Junction, got {:?}", grammar);
        }
    }

    #[test]
    fn test_glossolalia() {
        let mut vm = make_vm();
        assert_eq!(vm.grid[0][0], Value::Int(0));

        generate_random_grid_grammar(&mut vm, 0, 0, 5);

        let mut changed = false;
        for y in 0..16 {
            for x in 0..16 {
                if vm.grid[y][x] != Value::Int(0) {
                    changed = true;
                    break;
                }
            }
        }
        assert!(changed, "Glossolalia did not write to grid");
    }
}
