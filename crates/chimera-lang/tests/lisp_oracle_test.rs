#[cfg(feature = "oracle")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::JunctionType;
    use chimera_lang::lisp;
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::vm::Value;

    #[test]
    fn test_lisp_rule_query() {
        let code = r#"
        (rule (ancestor ?x ?y) (parent ?x ?z) (ancestor ?z ?y))
        (rule (ancestor ?x ?y) (parent ?x ?y))
        (assert (parent "alice" "bob"))
        (assert (parent "bob" "charlie"))
        (query (ancestor "alice" ?who))
        "#;

        let dna = lisp::compile(code).expect("Failed to compile");
        let mut vm = ChimeraVM::new(dna);

        while !vm.halted {
            vm.step();
            if vm.ip.0 >= vm.dna.helix.strands.len() {
                break;
            }
        }

        // Query pushes: Int(1) on success, then Junction(All, bindings)
        let bindings = vm.stack.pop().unwrap();
        let success = vm.stack.pop().unwrap();

        assert_eq!(success, Value::Int(1));

        if let Value::Junction(JunctionType::All, list) = bindings {
            println!("Bindings: {:?}", list);
            assert!(!list.is_empty());
        } else {
            panic!("Expected bindings list");
        }
    }
}
