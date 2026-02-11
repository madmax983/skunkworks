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
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_ley_sense() {
        let mut vm = make_vm(vec![Gene {
            op: OpCode::LeySense,
            args: vec![],
        }]);

        // Manually configure network
        vm.ley_network.nodes.clear();
        vm.ley_network.connections.clear();
        vm.ley_network.add_node(8, 8, 100);
        vm.context_loc = (8, 9); // 1 step east (x=9) to x=8 -> dx = -1

        vm.step();

        // Stack: [ ..., dy, dx, dist, power ]
        assert_eq!(vm.stack.pop(), Some(Value::Int(100))); // Power
        assert_eq!(vm.stack.pop(), Some(Value::Int(1))); // Dist
        assert_eq!(vm.stack.pop(), Some(Value::Int(-1))); // dx
        assert_eq!(vm.stack.pop(), Some(Value::Int(0))); // dy
    }

    #[test]
    fn test_ley_tap() {
        let mut vm = make_vm(vec![Gene {
            op: OpCode::LeyTap,
            args: vec![],
        }]);

        vm.ley_network.nodes.clear();
        vm.ley_network.connections.clear();
        vm.ley_network.add_node(5, 5, 10); // Low power to avoid overload
        vm.context_loc = (5, 5); // On top of node

        let initial_energy = vm.energy;
        vm.step();

        // Energy should increase (gain 10 - cost 1 step)
        // Note: step() decreases energy by 1 before execution.
        // So expected = initial - 1 + 10 = initial + 9.
        // However, there is a 5% chance of overload (loss of gain/2 = 5).
        // Overload: initial - 1 - 5 = initial - 6.

        let success = vm.energy == initial_energy + 9;
        let overload = vm.energy == initial_energy - 6;

        assert!(
            success || overload,
            "Energy was {}, expected {} (success) or {} (overload)",
            vm.energy,
            initial_energy + 9,
            initial_energy - 6
        );

        if success {
            assert_eq!(vm.stack.pop(), Some(Value::Int(10)));
            // Node power should drain
            assert_eq!(vm.ley_network.nodes[0].power, 9);
        } else {
            // Overload pushes negative damage
            assert_eq!(vm.stack.pop(), Some(Value::Int(-5)));
            // Node power halved
            assert_eq!(vm.ley_network.nodes[0].power, 5);
        }
    }

    #[test]
    fn test_ley_warp() {
        let mut vm = make_vm(vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Target Node 1
            Gene {
                op: OpCode::LeyWarp,
                args: vec![],
            },
        ]);

        vm.ley_network.nodes.clear();
        vm.ley_network.connections.clear();

        // Node 0 at (0,0)
        vm.ley_network.add_node(0, 0, 100);
        // Node 1 at (10, 10)
        vm.ley_network.add_node(10, 10, 100);

        // Connect 0 <-> 1
        vm.ley_network.connect(0, 1);

        vm.context_loc = (0, 0); // At Node 0

        vm.step(); // Push
        vm.step(); // Warp

        assert_eq!(vm.context_loc, (10, 10));
    }

    #[test]
    fn test_ley_shift() {
        let mut vm = make_vm(vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // dx
            Gene {
                op: OpCode::LeyShift,
                args: vec![],
            },
        ]);

        vm.ley_network.nodes.clear();
        vm.ley_network.connections.clear();
        vm.ley_network.add_node(5, 5, 100);
        vm.context_loc = (5, 5);

        vm.step(); // Push 1
        vm.step(); // Push 2
        vm.step(); // Shift

        // Node should move to (5+1, 5+2) -> (6, 7)
        assert_eq!(vm.ley_network.nodes[0].y, 6);
        assert_eq!(vm.ley_network.nodes[0].x, 7);
    }
}
