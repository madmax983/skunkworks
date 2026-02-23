#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_unbounded_stack_growth() {
        println!("Testing Unbounded Stack Growth...");

        // [ push(1) jump(0) ]
        // This program infinitely pushes 1 onto the stack.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };

        let mut vm = ChimeraVM::new(dna);
        vm.energy = 100_000;

        // Fix: Increase telomeres so it doesn't die of old age
        if !vm.telomeres.is_empty() {
            vm.telomeres[0] = 100_000;
        }

        // Run for a bit and see if stack grows
        for _ in 0..10_000 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        println!("Final Stack size = {}", vm.stack.len());

        // Assert that stack size is capped.
        // We allow a small margin over 4096 for side-channel pushes (Egregore, etc.)
        // But it should be significantly less than the 5000 we saw before.
        assert!(vm.stack.len() < 4200, "Stack grew beyond safe limits!");
    }
}
