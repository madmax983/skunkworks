#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.metamorphism_enabled = false;
        vm.context_loc = (8, 8); // Ensure center
        vm
    }

    #[test]
    fn test_emit_scent() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Rose".to_string())],
            },
            Gene {
                op: OpCode::Emit,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // push 100
        vm.step(); // push Rose
        vm.step(); // emit

        assert_eq!(vm.pheromones.len(), 1);
        assert_eq!(vm.pheromones[0].signature, "Rose");
        assert_eq!(vm.pheromones[0].intensity, 100.0);
    }

    #[test]
    fn test_smell_scent() {
        let mut vm = make_vm(vec![]);
        // Manually add scent close to center (8,8) -> (8.5, 8.5) center of cell
        // Place scent at (8.5, 9.5) -> South
        vm.pheromones.push(crate::vm::nova_scent::Scent {
            signature: "Musk".to_string(),
            x: 8.5,
            y: 9.5,
            intensity: 200.0,
            age: 0,
        });

        // Inject smell instruction
        let genes = vec![Gene {
            op: OpCode::Smell,
            args: vec![],
        }];
        vm.dna.helix.strands[0].genes = genes;

        vm.step();

        // Stack: signature, intensity, dx, dy (top)
        let dy = vm.stack.pop().unwrap();
        let dx = vm.stack.pop().unwrap();
        let int = vm.stack.pop().unwrap();
        let sig = vm.stack.pop().unwrap();

        assert_eq!(dy, Value::Int(1)); // South
        assert_eq!(dx, Value::Int(0));
        if let Value::Int(i) = int {
            assert!(i > 0);
        } else {
            panic!("Expected int intensity");
        }
        assert_eq!(sig, Value::Str("Musk".to_string()));
    }

    #[test]
    fn test_track_scent() {
        let mut vm = make_vm(vec![]);
        // Place scent at (10.5, 8.5) -> East (dx=2)
        vm.pheromones.push(crate::vm::nova_scent::Scent {
            signature: "Food".to_string(),
            x: 10.5,
            y: 8.5,
            intensity: 100.0,
            age: 0,
        });

        // Inject track instruction
        // [ push("Food") track() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Food".to_string())],
            },
            Gene {
                op: OpCode::Track,
                args: vec![],
            },
        ];
        vm.dna.helix.strands[0].genes = genes;

        vm.step(); // push
        vm.step(); // track

        // Stack: dx, dy (top)
        let dy = vm.stack.pop().unwrap();
        let dx = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x)) = (dy, dx) {
            assert!(x > 0, "Should point East");
            // Due to Brownian motion, y might slightly drift, but X should be dominant
            assert!(x.abs() > y.abs(), "X should be dominant direction");
        } else {
            panic!("Expected int vector");
        }
    }

    #[test]
    fn test_scent_decay() {
        let mut vm = make_vm(vec![]);
        vm.pheromones.push(crate::vm::nova_scent::Scent {
            signature: "Fades".to_string(),
            x: 8.5,
            y: 8.5,
            intensity: 100.0,
            age: 0,
        });

        // Run environment step manually (exposed via step() but also check logic)
        // Since step() consumes energy and runs genes, we'll just check if process_scents works.
        // It's called in step().

        vm.step(); // 1 tick

        assert_eq!(vm.pheromones.len(), 1);
        assert!(vm.pheromones[0].intensity < 100.0);
        assert_eq!(vm.pheromones[0].age, 1);

        // Decay until gone (threshold 1.0)
        // 100 * 0.95^n < 1.0 -> n approx 90
        for _ in 0..100 {
            crate::vm::nova_scent::process_scents(&mut vm);
        }

        assert!(vm.pheromones.is_empty());
    }
}
