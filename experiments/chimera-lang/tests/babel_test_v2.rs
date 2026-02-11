#[cfg(feature = "nova")]
#[test]
fn test_babel_grammar() {
    use chimera_lang::compiler::compile;
    use chimera_lang::vm::{ChimeraVM, Value};

    let src = r#"
    strand main {
        # Register the grammar rules
        Math exec

        # Construct a parser for 'expr'
        "expr" call_rule

        # Input string
        "x+x"

        # Execute parse
        parse
    }

    grammar Math {
        rule expr { term "+" term }
        rule term { "x" }
    }
    "#;

    let dna = compile(src, None).expect("Compilation failed");
    let mut vm = ChimeraVM::new(dna);

    // Run until halted
    while !vm.halted {
        vm.step();
    }

    println!("Output: {:?}", vm.output);
    println!("Stack: {:?}", vm.stack);

    // Check stack for result
    assert_eq!(vm.stack.len(), 1, "Stack should have 1 item");
    let result = vm.stack.pop().unwrap();
    println!("Result: {}", result);

    if let Value::Junction(chimera_lang::ast::JunctionType::All, items) = result {
        assert_eq!(items.len(), 2);
        assert_eq!(items[1], Value::Str("x".to_string()));

        if let Value::Junction(chimera_lang::ast::JunctionType::All, sub) = &items[0] {
             assert_eq!(sub[0], Value::Str("x".to_string()));
             assert_eq!(sub[1], Value::Str("+".to_string()));
        } else {
            panic!("Expected nested junction");
        }
    } else {
        panic!("Expected Junction result, got {:?}", result);
    }
}
