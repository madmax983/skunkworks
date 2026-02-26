#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value, MAX_STRING_LEN};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_string_bomb_safety() {
        // 👺 HAVOC: The Ouroboros String Bomb
        // EXPECTATION: The VM should prevent strings from exceeding MAX_STRING_LEN.
        // REALITY: OpCode::Add does not check bounds. This test FAILS if the bug exists.

        let mut genes = Vec::new();

        // 1. Push "A"
        genes.push(Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("A".to_string())],
        });

        // 2. Loop 20 times: Dup, Add. 2^20 = 1MB.
        for _ in 0..20 {
            genes.push(Gene {
                op: OpCode::Dup,
                args: vec![],
            });
            genes.push(Gene {
                op: OpCode::Add,
                args: vec![],
            });
        }

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute until halted or finished
        while !vm.halted && vm.ip.0 == 0 && vm.ip.1 < vm.dna.helix.strands[0].genes.len() {
            vm.step();
        }

        // Verify safety
        if let Some(Value::Str(s)) = vm.stack.last() {
            println!("Final String Length: {}", s.len());
            assert!(
                s.len() <= MAX_STRING_LEN,
                "BUFFER OVERFLOW: String length {} exceeded MAX {}",
                s.len(),
                MAX_STRING_LEN
            );
        } else {
            panic!("Stack empty or invalid type");
        }
    }
}
