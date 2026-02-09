#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_transposon_jump() {
        // [ Push(2) Transposon Push(99) Push(99) ]
        // Transposon pops 2.
        // Moves itself (Transposon) to current + 2.
        // current is idx 1. target is 1 + 2 = 3.
        // Strand becomes: [ Push(2) Nop Push(99) Transposon ]
        // And IP jumps to 3.
        // So next execution is whatever is AFTER execution. If at end, loop or stop.
        // Let's add a gene at the end to verify execution flow.
        // [ Push(2) Transposon Push(100) Push(200) Push(300) ]
        // Target: 1 + 2 = 3.
        // Gene at 3 is Push(200).
        // It overwrites Push(200) with Transposon?
        // Wait, logic was overwrite.
        // Original:
        // 0: Push(2)
        // 1: Transposon
        // 2: Push(100)
        // 3: Push(200)
        // 4: Push(300)

        // Execution:
        // 0: Push(2) -> Stack: [2]
        // 1: Transposon -> Pops 2. Target 1+2=3.
        //    Strand[1] = Nop
        //    Strand[3] = Transposon (copy of self)
        //    IP = 3
        // Next step execute IP 3 (Transposon again?)
        // If it executes Transposon again, it will pop stack. Stack is empty!
        // So let's push two values.

        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Transposon, args: vec![] }, // Index 2
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // Index 3
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(200)] }, // Index 4
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(300)] }, // Index 5
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 0: Push(2)
        vm.step();

        // Step 1: Push(2)
        vm.step();
        assert_eq!(vm.stack.len(), 2);

        // Step 2: Transposon
        // Pops 2. Current IP is (0, 2). Target is 2 + 2 = 4.
        // Strand[2] becomes Nop.
        // Strand[4] (Push(200)) becomes Transposon.
        // IP becomes (0, 4).
        vm.step();

        assert_eq!(vm.ip, (0, 4));
        assert_eq!(vm.dna.helix.strands[0].genes[2].op, OpCode::Nop);
        assert_eq!(vm.dna.helix.strands[0].genes[4].op, OpCode::Transposon);

        // Next step: Execute IP 4 (Transposon again)
        // Pops 2 (from first push). Target 4 + 2 = 6.
        // Strand len is 6. Index 6 is out of bounds (0..5).
        // Should error and return None (no jump, just advance).
        // IP becomes (0, 5).
        vm.step();

        // Check if error logged
        assert!(vm.output.last().unwrap().contains("out of bounds"));
        assert_eq!(vm.ip, (0, 5));

        // Execute Push(300)
        vm.step();
        assert_eq!(vm.stack.pop(), Some(Value::Int(300)));
    }
}
