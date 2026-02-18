#[cfg(test)]
#[cfg(feature = "nova")]
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
        vm.energy = 1000;
        vm
    }

    #[test]
    fn test_simulation_depth_protection() {
        // Strand: [ push(0) push(10) simulate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Strand 0
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // 10 ticks
            },
            Gene {
                op: OpCode::Simulate,
                args: vec![],
            },
        ];

        // Case 1: Recursion Depth 11 (Greater than limit 10)
        // Should be blocked.
        let mut vm = make_vm(genes.clone());
        vm.recursion_depth = 11;

        // Execute 3 steps to hit Simulate
        vm.step(); // Push 0
        vm.step(); // Push 10
        vm.step(); // Simulate

        let last_msg = vm.output.last().expect("Output should not be empty");
        println!("Output: {}", last_msg);
        assert!(
            last_msg.contains("Simulation depth limit exceeded"),
            "Should block simulation at depth 11"
        );

        // Case 2: Recursion Depth 5 (Safe)
        // Should execute.
        let mut vm2 = make_vm(genes.clone());
        vm2.recursion_depth = 5;

        vm2.step(); // Push 0
        vm2.step(); // Push 10
        vm2.step(); // Simulate -> Should run

        let last_msg_2 = vm2.output.last().expect("Output should not be empty");
        println!("Output 2: {}", last_msg_2);
        assert!(
            last_msg_2.contains("SIMULATE: Ran strand 0"),
            "Should allow simulation at depth 5"
        );
    }
}
