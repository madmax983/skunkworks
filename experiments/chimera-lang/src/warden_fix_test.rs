#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::vm::{MAX_CALL_STACK_DEPTH, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_input_interrupt_stack_overflow() {
        // Strand 0: [ bind('a') ... ]
        // Bind expects: stack: [char, strand] (top)
        // Push char ('a' = 97)
        // Push strand (0)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(97)],
            }, // 'a'
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // strand 0
            Gene {
                op: OpCode::Bind,
                args: vec![],
            },
            // Infinite loop to keep VM running
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];

        let mut vm = make_vm(genes);

        // Bind 'a' to strand 0
        vm.step();
        vm.step();
        vm.step();

        // Give infinite energy
        vm.energy = 10000;

        // Flood input buffer with 'a'
        for _ in 0..(MAX_CALL_STACK_DEPTH + 10) {
            vm.handle_input('a');
        }

        // Process interrupts
        // handle_input_interrupts is called inside step()
        for _ in 0..(MAX_CALL_STACK_DEPTH + 20) {
            vm.step();
        }

        // With the bug, call_stack grows without bound
        assert!(
            vm.call_stack.len() <= MAX_CALL_STACK_DEPTH,
            "Call stack exceeded limit! Current: {}, Max: {}",
            vm.call_stack.len(),
            MAX_CALL_STACK_DEPTH
        );
    }

    #[test]
    fn test_incubate_dos_string_explosion() {
        // Setup a grid full of large strings
        // Each cell = 1MB string
        // Incubate 10 cells -> 10MB strand (acceptable test size)
        // If we tried 1GB it would crash the test runner, so we test logic.

        let mut genes = vec![];
        // Just empty strand, we manipulate grid directly
        let mut vm = make_vm(genes);

        let large_string = "A".repeat(1024 * 1024); // 1MB
        for x in 0..10 {
            vm.grid[0][x] = Value::Str(large_string.clone());
        }

        // Push args for Incubate: [len=10, y=0, x=0]
        vm.stack.push(Value::Int(10)); // len
        vm.stack.push(Value::Int(0));  // y
        vm.stack.push(Value::Int(0));  // x

        // Execute Incubate manually
        // Since we are mocking the VM state, we can call the inner execution or opcode directly.
        // Or we can just use vm.execute_gene_inner if exposed.
        // nova::exec_nova_op is private, so we go through execute_gene.

        // We need to enable nova feature, which is done by cfg.
        // We use execute_gene

        vm.energy = 1000;
        vm.execute_gene_inner(OpCode::Incubate, &[]);

        // Verify failure
        // 1. Check output for abort message
        let output = vm.output.join("\n");
        assert!(output.contains("INCUBATE: Aborted"), "Should have aborted due to size limit. Output: {}", output);

        // 2. Check strand count (should still be 1, the empty initial strand)
        assert_eq!(vm.dna.helix.strands.len(), 1, "Should not have created a new strand");
    }
}
