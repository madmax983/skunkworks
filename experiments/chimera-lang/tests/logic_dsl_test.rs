#[cfg(all(test, feature = "nova", feature = "oracle"))]
mod tests {
    use chimera_lang::compiler::compile;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_logic_compilation_and_execution() {
        let src = r#"
        rule ancestor(?A, ?B) :- parent(?A, ?B)

        strand main {
            # Add facts
            assert(parent("Alice", "Bob"))

            # Query the rule
            query(ancestor("Alice", "Bob"))
        }
        "#;

        let dna = compile(src, None).expect("Compilation failed");
        let mut vm = ChimeraVM::new(dna);

        // Run until halted
        while !vm.halted {
            vm.step();
        }

        // Check result
        // Stack should contain: [ 1 ] (Success)
        // Wait, did we consume energy? Yes.
        // assert pushes nothing.
        // query pushes 1 or 0.
        // Stack should have 1.

        // Stack contains [ SuccessFlag, Bindings ] on success
        if vm.stack.len() == 2 {
            // Check success flag (at index 0 or 1? Push 1, then Push Bindings. So 0=1, 1=Bindings)
            match &vm.stack[0] {
                Value::Int(1) => (), // Success
                val => {
                    println!("VM Output:\n{}", vm.output.join("\n"));
                    panic!("Expected Success(1), got {:?}", val);
                }
            }
        } else {
             println!("VM Output:\n{}", vm.output.join("\n"));
             panic!("Stack should have 2 items (Result + Bindings), got {}", vm.stack.len());
        }
    }

    #[test]
    fn test_logic_recursion() {
        let src = r#"
        rule path(?X, ?Y) :- edge(?X, ?Y)
        rule path(?X, ?Y) :- edge(?X, ?Z), path(?Z, ?Y)

        strand main {
            assert(edge(1, 2))
            assert(edge(2, 3))

            # Query path(1, 3)
            query(path(1, 3))
        }
        "#;

        let dna = compile(src, None).expect("Compilation failed");
        let mut vm = ChimeraVM::new(dna);

        // Run (limit steps to prevent infinite loop if buggy)
        let mut steps = 0;
        while !vm.halted && steps < 1000 {
            vm.step();
            steps += 1;
        }

        if vm.stack.len() == 2 {
            match &vm.stack[0] {
                Value::Int(1) => (),
                val => {
                    println!("VM Output:\n{}", vm.output.join("\n"));
                    panic!("Expected Success(1), got {:?}", val);
                }
            }
        } else {
             println!("VM Output:\n{}", vm.output.join("\n"));
             panic!("Stack should have 2 items (Result + Bindings), got {}", vm.stack.len());
        }
    }
}
