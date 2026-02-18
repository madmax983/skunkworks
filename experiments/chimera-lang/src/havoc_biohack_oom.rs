#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::memetics::Meme;
    use crate::vm::{ChimeraVM, Value, MAX_STRANDS};

    #[test]
    fn test_biohack_oom() {
        let mut vm = ChimeraVM::new(Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        });

        // 1. Create a Meme manually
        let meme = Meme {
            genes: vec![Gene {
                op: OpCode::Nop,
                args: vec![],
            }],
            virulence: 100,
            fidelity: 100,
            description: "Grey Goo".to_string(),
        };
        vm.meme_pool.memes.push(meme);

        // 2. Loop BioHack
        // We expect it to stop at MAX_STRANDS (1024), but the bug allows it to grow indefinitely.
        // We'll run 1100 times to exceed the limit.
        for _ in 0..1100 {
            vm.stack.push(Value::Str("Virus".to_string()));
            vm.stack
                .push(Value::Junction(JunctionType::Any, vec![])); // Dummy grammar
            vm.stack.push(Value::Int(0)); // Meme ID 0

            crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::BioHack, &[]);
        }

        // 3. Assert failure
        // If the bug exists, this will fail because len > 1024.
        assert!(
            vm.dna.helix.strands.len() <= MAX_STRANDS,
            "Managed to create {} strands, exceeding limit of {}",
            vm.dna.helix.strands.len(),
            MAX_STRANDS
        );
    }
}
