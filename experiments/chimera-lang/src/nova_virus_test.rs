#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_infect_op() {
        let mut vm = make_vm();

        // Stack: rate, pattern, name
        // Push in order: Name (bottom), Pattern, Rate (top) -> No, stack logic in code:
        // let name_val = vm.stack.pop().unwrap();
        // let pattern_val = vm.stack.pop().unwrap();
        // let rate_val = vm.stack.pop().unwrap();
        // So Stack Top: Name, Pattern, Rate (Bottom)
        // Wait, stack.pop() returns TOP.

        // Code:
        // let name_val = vm.stack.pop().unwrap(); (Top)
        // let pattern_val = vm.stack.pop().unwrap();
        // let rate_val = vm.stack.pop().unwrap();

        // So we need to push: Payload, Rate, then Pattern, then Name.
        vm.stack.push(Value::Int(-1)); // Payload
        vm.stack.push(Value::Int(100)); // Rate
        vm.stack.push(Value::Str("FOO".to_string())); // Pattern
        vm.stack.push(Value::Str("TestVirus".to_string())); // Name (Top)

        vm.context_loc = (5, 5);

        // Execute Infect
        let op = OpCode::Infect;
        crate::vm::memetics::exec_memetics_op(&mut vm, op, &[]);

        assert_eq!(vm.virus_library.len(), 1);
        assert_eq!(vm.virus_library[0].name, "TestVirus");

        let state = vm.viral_grid[5][5].expect("Cell should be infected");
        assert_eq!(state.infection_level, 100);
        assert_eq!(state.virus_id, 0);
    }

    #[test]
    fn test_outbreak_spread() {
        let mut vm = make_vm();

        // 1. Setup Virus manually
        let virus = crate::vm::memetics::Virus {
            name: "SpreadVirus".to_string(),
            color: (255, 0, 0),
            pattern: "TARGET".to_string(),
            mutation_rate: 0,
            payload: None,
        };
        vm.virus_library.push(virus);

        // 2. Infect (5,5)
        vm.viral_grid[5][5] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        // 3. Place target text in neighbor (5,6)
        vm.grid[5][6] = Value::Str("TARGET_DATA".to_string());

        // 4. Run Outbreak
        let op = OpCode::Outbreak;
        crate::vm::memetics::exec_memetics_op(&mut vm, op, &[]);

        // 5. Check spread
        let neighbor = vm.viral_grid[5][6].expect("Neighbor should be infected");
        assert_eq!(neighbor.virus_id, 0);
        assert_eq!(neighbor.infection_level, 50); // Initial load
    }

    #[test]
    fn test_sanitize() {
        let mut vm = make_vm();
        vm.viral_grid[5][5] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        vm.stack.push(Value::Int(2)); // Radius
        vm.context_loc = (5, 5);

        crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::Sanitize, &[]);

        assert!(vm.viral_grid[5][5].is_none());
    }
}
