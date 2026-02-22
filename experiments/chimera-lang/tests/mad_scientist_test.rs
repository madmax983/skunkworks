#[cfg(feature = "oracle")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_metabolism_query() {
        let mut vm = make_vm();
        vm.energy = 42;
        vm.experience = 100;

        // Query: metabolism(?E ?X) (Space separated, no comma)
        vm.stack.push(Value::Str("metabolism(?E ?X)".to_string()));

        // Execute PrologCall
        chimera_lang::vm::oracle::exec_oracle_op(&mut vm, OpCode::PrologCall, &[]);

        // Check output for errors
        println!("VM Output: {:?}", vm.output);

        // Result should be on stack
        let result = vm.stack.pop().expect("Stack should have result");
        println!("Result: {:?}", result);

        if let Value::Junction(chimera_lang::ast::JunctionType::All, bindings) = result {
            // Check for ?E = 42
            let has_energy = bindings.iter().any(|b| {
                if let Value::Junction(_, pair) = b {
                    if pair.len() == 2 {
                        if let (Value::Str(k), Value::Int(v)) = (&pair[0], &pair[1]) {
                            return k == "?E" && *v == 42;
                        }
                    }
                }
                false
            });
            assert!(has_energy, "Did not find ?E = 42 binding");
        } else {
            panic!("Expected Junction result, got {:?}", result);
        }
    }

    #[test]
    fn test_synthesis_manifestation() {
        let mut vm = make_vm();

        // Manifest(Query, Transform)
        use chimera_lang::ast::JunctionType;

        // metabolism(?E ?X)
        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("metabolism".to_string()),
                Value::Str("?E".to_string()),
                Value::Str("?X".to_string()),
            ],
        );

        // synthesize("energy" 0)
        let transform = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("synthesize".to_string()),
                Value::Str("energy".to_string()),
                Value::Int(0),
            ],
        );

        vm.stack.push(query);
        vm.stack.push(transform);

        // Execute Manifest
        chimera_lang::vm::oracle::exec_oracle_op(&mut vm, OpCode::Manifest, &[]);

        println!("VM Output: {:?}", vm.output);

        // Check if strand 0 exists and has photosynthesize
        assert_eq!(vm.dna.helix.strands.len(), 1);
        assert_eq!(vm.dna.helix.strands[0].genes[0].op, OpCode::Photosynthesize);
    }
}
