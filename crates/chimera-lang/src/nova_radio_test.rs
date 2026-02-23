#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_broadcast_tune() {
        // [ push(1) push(100) broadcast() push(1) tune() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Channel 1
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)], // Value
            },
            Gene {
                op: OpCode::Broadcast,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Channel 1
            },
            Gene {
                op: OpCode::Tune,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(100));
    }

    #[test]
    fn test_radio_fifo() {
        // [ push(1) push(10) broadcast()
        //   push(1) push(20) broadcast()
        //   push(1) tune()
        //   push(1) tune() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Broadcast,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Broadcast,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Tune,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Tune,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 2);
        // Stack order: First pushed is bottom? No, stack is LIFO, but instructions push sequentially.
        // Stack state:
        // After 1st tune: [10]
        // After 2nd tune: [10, 20]
        // VM stack[0] is bottom (10), stack[1] is top (20).
        assert_eq!(vm.stack[0], Value::Int(10));
        assert_eq!(vm.stack[1], Value::Int(20));
    }

    #[test]
    fn test_radio_capacity() {
        // Fill channel with 100 items. Then try 101th.
        // We simulate this with manual loop in simulation for simplicity.
        let mut genes = Vec::new();
        for i in 0..101 {
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            }); // Channel 99
            genes.push(Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i as i64)],
            });
            genes.push(Gene {
                op: OpCode::Broadcast,
                args: vec![],
            });
        }

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 10000;
        // Prevent Egregore interference (Whispers corrupt stack)
        vm.egregore.manifestation_timer = 10000;

        while !vm.halted {
            vm.step();
            // Prevent toxicity mutations by clearing waste at context location
            let (cy, cx) = vm.context_loc;
            vm.waste_grid[cy][cx] = 0;
        }

        // Check internal ether state
        if let Some(queue) = vm.ether.get(&99) {
            assert_eq!(queue.len(), 100);
            // The last one (100) should have been rejected.
            // Queue contains 0..99.
            assert_eq!(queue.front(), Some(&Value::Int(0)));
            assert_eq!(queue.back(), Some(&Value::Int(99)));
        } else {
            panic!("Queue 99 not found");
        }

        // Check output for error
        assert!(vm.output.iter().any(|s| s.contains("Channel 99 full")));
    }

    #[test]
    fn test_radio_persistence_manual() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(7)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Broadcast,
                args: vec![],
            },
            Gene {
                op: OpCode::Sporulate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Run until halted or finished
        // 3 instructions.
        vm.step(); // Push
        vm.step(); // Push
        vm.step(); // Broadcast
        vm.step(); // Sporulate

        // Verify state A
        assert_eq!(vm.ether.get(&7).unwrap().len(), 1);

        // Modify state
        vm.ether.get_mut(&7).unwrap().push_back(Value::Int(999));
        assert_eq!(vm.ether.get(&7).unwrap().len(), 2);

        // Germinate manually
        // Spore ID 0 is on stack.
        // We need to execute Germinate.

        // Let's just grab the spore and check its ether.
        let spore = &vm.spores[0];
        assert_eq!(spore.ether.get(&7).unwrap().len(), 1);
        assert_eq!(spore.ether.get(&7).unwrap()[0], Value::Int(42));
    }
}
