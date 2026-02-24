#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_self_rewrite_and_perceive() {
        let mut vm = make_empty_vm();

        // 1. Construct Grammar: Match("TEST")
        // Junction(Any, [Str("Match"), Str("TEST")])
        let grammar = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Match".to_string()),
                Value::Str("TEST".to_string()),
            ],
        );

        // 2. Execute SelfRewrite
        vm.stack.push(grammar);
        crate::vm::babel::exec_babel_op(&mut vm, OpCode::SelfRewrite, &[]);

        // Verify active grammar updated
        if let Value::Junction(_, args) = &vm.active_grammar {
            assert_eq!(args.len(), 2);
            assert_eq!(args[1], Value::Str("TEST".to_string()));
        } else {
            panic!("Active grammar not updated correctly");
        }

        // 3. Write "TEST" to grid at (0,0)
        vm.grid[0][0] = Value::Str("T".to_string());
        vm.grid[0][1] = Value::Str("E".to_string());
        vm.grid[0][2] = Value::Str("S".to_string());
        vm.grid[0][3] = Value::Str("T".to_string());

        // 4. Set Context Loc to (0,0)
        vm.context_loc = (0, 0);

        // 5. Execute Perceive(4)
        // Set IP to (0,0) so we have a return address
        vm.ip = (0, 0);
        vm.stack.push(Value::Int(4));
        let res = crate::vm::babel::exec_babel_op(&mut vm, OpCode::Perceive, &[]);

        // 6. Assertions
        assert!(res.is_some(), "Perceive should return a jump target");
        let (new_strand, new_gene) = res.unwrap();
        assert!(new_strand > 0, "Should jump to new strand");
        assert_eq!(new_gene, 0);

        assert_eq!(
            vm.stack.pop(),
            Some(Value::Int(1)),
            "Stack should have Success(1)"
        );
        assert_eq!(vm.call_stack.len(), 1, "Should have pushed return address");
        assert_eq!(vm.call_stack[0], (0, 1));

        // Check compiled strand content
        // Match("TEST") should compile to Push("TEST") (from compile_cst default behavior for Str)
        // Wait, compile_cst compiles the AST. The AST for Match("TEST") is Str("TEST").
        // compile_cst_recursive for Str("TEST") generates Push("TEST").

        let compiled_strand = &vm.dna.helix.strands[new_strand];
        assert_eq!(compiled_strand.genes.len(), 1);
        assert_eq!(compiled_strand.genes[0].op, OpCode::Push);
        if let Nucleotide::String(s) = &compiled_strand.genes[0].args[0] {
            assert_eq!(s, "TEST");
        } else {
            panic!("Expected String argument");
        }
    }
}
