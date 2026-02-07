#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_relativity_toggle() {
        let genes = vec![Gene {
            op: OpCode::Relativity,
            args: vec![],
        }];
        let mut vm = make_vm(genes);
        assert!(!vm.relativity_mode);
        vm.step();
        assert!(vm.relativity_mode);
        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("RELATIVITY: Physics engine ON")));
    }

    #[test]
    fn test_gravity_accretion() {
        // Turn on relativity, emit graviton, check horizon
        let genes = vec![
            Gene {
                op: OpCode::Relativity,
                args: vec![],
            },
            Gene {
                op: OpCode::Graviton,
                args: vec![],
            },
            Gene {
                op: OpCode::EventHorizon,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        let (cy, cx) = vm.context_loc;

        vm.step(); // Relativity ON
        assert!(vm.relativity_mode);

        vm.step(); // Graviton
                   // Expect mass at context_loc
                   // Base accretion is 5 per tick. Graviton adds 50.
                   // Tick 1: +5.
                   // Tick 2: +5 + 50 = 55 (plus previous diffusions/decay).
                   // Let's check grid directly or via EventHorizon.
        let g = vm.gravity_grid[cy][cx];
        assert!(g >= 50, "Gravity should increase significantly");

        vm.step(); // EventHorizon -> push g
        let val = vm.stack.pop().unwrap();
        if let crate::vm::Value::Int(measured_g) = val {
            assert!(measured_g >= 50);
        } else {
            panic!("Expected Int");
        }
    }

    #[test]
    fn test_time_dilation() {
        // 1. Turn on relativity
        // 2. Emit Graviton to create massive gravity (> 100 for stasis)
        // 3. Emit Graviton again to ensure > 100
        // 4. Check time_grid

        let genes = vec![
            Gene {
                op: OpCode::Relativity,
                args: vec![],
            },
            Gene {
                op: OpCode::Graviton,
                args: vec![],
            },
            Gene {
                op: OpCode::Graviton,
                args: vec![],
            },
            Gene {
                op: OpCode::Graviton,
                args: vec![],
            },
            Gene {
                op: OpCode::Graviton,
                args: vec![],
            }, // More mass to overcome decay
        ];
        let mut vm = make_vm(genes);
        let (cy, cx) = vm.context_loc;

        // Run steps
        for _ in 0..5 {
            vm.step();
        }

        let g = vm.gravity_grid[cy][cx];
        assert!(g > 100, "Gravity should be > 100, got {}", g);

        let t = vm.time_grid[cy][cx];
        assert_eq!(t, 0, "Time should be frozen (0) due to high gravity");
    }
}
