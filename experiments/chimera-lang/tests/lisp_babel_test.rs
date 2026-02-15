#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::lisp;
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::vm::Value;

    #[test]
    fn test_lisp_grammar() {
        // We compile a fragment to get genes, then wrap in DNA
        // But lisp::compile does that for us.
        let code = r#"
        (parse (seq (match "Hello") (match "World")) "HelloWorld")
        "#;

        let dna = lisp::compile(code).expect("Failed to compile");
        let mut vm = ChimeraVM::new(dna);

        // Run until completion
        while !vm.halted {
            vm.step();
            if vm.ip.0 >= vm.dna.helix.strands.len() {
                break;
            }
        }

        let ast = vm.stack.pop().unwrap();
        println!("AST: {}", ast);

        // AST for Seq matches is Junction(All, [matches...])
        if let Value::Junction(_, args) = ast {
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Value::Str("Hello".to_string()));
            assert_eq!(args[1], Value::Str("World".to_string()));
        } else {
            panic!("Expected AST Junction, got {:?}", ast);
        }
    }
}
