#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_babel_compile_simple() {
        let mut vm = make_vm();

        // CST: Junction(All, [ "push", "10" ])
        let cst = Value::Junction(
            JunctionType::All,
            vec![Value::Str("push".to_string()), Value::Str("10".to_string())],
        );

        vm.stack.push(cst);
        vm.stack.push(Value::Int(0)); // Handler index (mock)

        // Call OpCode::BabelCompile
        // Stack: [ cst, handler_idx ] -> [ new_strand_idx ]
        // But since we don't have easy access to private `exec_babel_op`, we simulate via VM execution
        // Or we just test `compile_cst` public function if available?
        // `compile_cst` is in `babel.rs` but not re-exported easily?
        // It is `pub` in `babel.rs`. Let's use `exec_babel_op` via `execute_gene`.

        let res = vm.execute_gene_inner(OpCode::BabelCompile, &[]);

        assert!(res.is_none());
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(idx) = vm.stack[0] {
            assert!(idx >= 0);
            assert!(idx < vm.dna.helix.strands.len() as i64);
            let strand = &vm.dna.helix.strands[idx as usize];
            // push "push", push "10" ? No, `compile_cst_recursive` logic:
            // "push" -> Push("push")
            // "10" -> Push("10")
            // Junction(All) -> Push("All"), Push(2), Call(handler)
            // So:
            // 1. Push("push")
            // 2. Push("10")
            // 3. Push("All")
            // 4. Push(2)
            // 5. Call(0)
            assert_eq!(strand.genes.len(), 5);
        } else {
            panic!("Expected strand index");
        }
    }

    #[test]
    fn test_recursive_grammar() {
        let mut vm = make_vm();

        // 1. Define Rule "S" -> "a" S "b" | ""
        // We construct this in parts.

        // Part 1: "a" S "b" (Sequence)
        // [ "Match", "a" ] grammar
        let match_a = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("a".to_string())],
        );

        // [ "Ref", "S" ] grammar
        let ref_s = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Ref".to_string()), Value::Str("S".to_string())],
        );

        // [ "Match", "b" ] grammar
        let match_b = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("b".to_string())],
        );

        // Combine into Seq
        let seq_part = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Seq".to_string()), match_a, ref_s, match_b],
        );

        // Part 2: "" (Empty Match)
        let match_empty = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("".to_string())],
        );

        // Combine into Alt
        let final_rule = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Alt".to_string()), seq_part, match_empty],
        );

        // Define Rule "S"
        vm.stack.push(final_rule);
        vm.stack.push(Value::Str("S".to_string()));
        vm.execute_gene_inner(OpCode::DefineRule, &[]);

        // 2. Parse "aaabbb"
        vm.stack.push(Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Ref".to_string()), Value::Str("S".to_string())],
        )); // Parser (Ref S)
        vm.stack.push(Value::Str("aaabbb".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);

        // Check result
        // Should be successful parse
        let result = vm.stack.pop().unwrap();
        // Failed parse pushes 0 (Int), Success pushes Junction (AST)
        if matches!(result, Value::Int(0)) {
            // Debug failure
            println!("Parse failed. Output: {:?}", vm.output);
        }
        assert!(matches!(result, Value::Junction(_, _)));

        // 3. Parse "aabbb" (Unbalanced)
        // "aabbb" -> "aa" S "bb" b -> "aa" "" "bb" b -> "aabb" match, remainder "b".
        // Exec OpCode::Parse checks if consumed == input.len().
        // So this should fail (return 0).

        vm.stack.push(Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Ref".to_string()), Value::Str("S".to_string())],
        ));
        vm.stack.push(Value::Str("aabbb".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);

        let fail_result = vm.stack.pop().unwrap();
        assert_eq!(fail_result, Value::Int(0));
    }

    #[test]
    fn test_babel_parser_match() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("foo".to_string()));
        vm.execute_gene_inner(OpCode::ParserMatch, &[]);
        let parser = vm.stack.pop().unwrap();

        vm.stack.push(parser);
        vm.stack.push(Value::Str("foobar".to_string()));

        // Use Parse directly
        vm.execute_gene_inner(OpCode::Parse, &[]);
        let _res = vm.stack.pop().unwrap();
        // Consumed 3, but Parse checks consumed == input.len() -> False -> 0
        // Wait, "foobar" vs "foo" -> Partial match.
        // OpCode::Parse implementation: if consumed == len -> Success, else -> 0 (Fail).
        // So "foobar" with "Match foo" should fail unless we slice input.
        // Let's test with exact match "foo"

        vm.stack.push(Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Match".to_string()),
                Value::Str("foo".to_string()),
            ],
        ));
        vm.stack.push(Value::Str("foo".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);
        let res2 = vm.stack.pop().unwrap();
        assert!(matches!(res2, Value::Str(_)));
    }

    #[test]
    fn test_babel_parser_seq() {
        let mut vm = make_vm();
        // Seq(Match("a"), Match("b"))

        let p_a = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("a".to_string())],
        );
        let p_b = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("b".to_string())],
        );

        vm.stack.push(p_a);
        vm.stack.push(p_b);
        vm.execute_gene_inner(OpCode::ParserSeq, &[]);
        let parser = vm.stack.pop().unwrap();

        vm.stack.push(parser);
        vm.stack.push(Value::Str("ab".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);
        let res = vm.stack.pop().unwrap();
        assert!(matches!(res, Value::Junction(JunctionType::All, _)));
    }

    #[test]
    fn test_babel_parser_alt() {
        let mut vm = make_vm();
        // Alt(Match("a"), Match("b"))

        let p_a = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("a".to_string())],
        );
        let p_b = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("Match".to_string()), Value::Str("b".to_string())],
        );

        vm.stack.push(p_a);
        vm.stack.push(p_b);
        vm.execute_gene_inner(OpCode::ParserAlt, &[]);
        let parser = vm.stack.pop().unwrap();

        // Match "b"
        vm.stack.push(parser);
        vm.stack.push(Value::Str("b".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);
        let res = vm.stack.pop().unwrap();
        match res {
            Value::Str(s) => assert_eq!(s, "b"),
            _ => panic!("Expected string 'b'"),
        }
    }

    #[test]
    fn test_babel_parser_regex() {
        let mut vm = make_vm();
        vm.stack.push(Value::Str("\\d+".to_string()));
        vm.execute_gene_inner(OpCode::ParserRegex, &[]);
        let parser = vm.stack.pop().unwrap();

        vm.stack.push(parser);
        vm.stack.push(Value::Str("12345".to_string()));
        vm.execute_gene_inner(OpCode::Parse, &[]);
        let res = vm.stack.pop().unwrap();
        match res {
            Value::Str(s) => assert_eq!(s, "12345"),
            _ => panic!("Expected string '12345'"),
        }
    }
}
