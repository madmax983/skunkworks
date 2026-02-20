#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, MAX_BRAINFUCK_OUTPUT, MAX_MEMES, MAX_VIRUSES};
    use crate::ast::{Dna, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::Value;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_meme_limit() {
        let mut vm = make_vm();
        // Fill meme pool
        for _ in 0..MAX_MEMES {
            vm.meme_pool.memes.push(crate::vm::memetics::Meme {
                genes: vec![],
                virulence: 0,
                fidelity: 0,
                description: "Filler".to_string(),
            });
        }

        // Try to conceive one more
        // Stack: len, virulence, fidelity
        vm.stack.push(Value::Int(10));
        vm.stack.push(Value::Int(50));
        vm.stack.push(Value::Int(50));

        // Execute Conceive
        // Note: Conceive usually needs genes in the strand to conceptualize.
        // We need to add some genes to strand 0.
        vm.dna.helix.strands[0].genes.push(crate::ast::Gene { op: OpCode::Nop, args: vec![] });
        vm.dna.helix.strands[0].genes.push(crate::ast::Gene { op: OpCode::Nop, args: vec![] });

        vm.execute_gene_inner(OpCode::Conceive, &[]);

        // Should return -1 (failure) on stack
        assert_eq!(vm.stack.last(), Some(&Value::Int(-1)));
        assert_eq!(vm.meme_pool.memes.len(), MAX_MEMES);
    }

    #[test]
    fn test_virus_limit() {
        let mut vm = make_vm();
        // Fill virus library
        for _ in 0..MAX_VIRUSES {
            vm.virus_library.push(crate::vm::memetics::Virus {
                name: "Filler".to_string(),
                color: (0,0,0),
                pattern: "".to_string(),
                mutation_rate: 0,
                payload: None,
                grammar: None,
                quorum_threshold: 0,
                quorum_action: None,
                mode: crate::vm::memetics::VirusMode::Overwrite,
            });
        }

        // Infect stack order: payload, rate, pattern, name, mode (popped in reverse)
        // Code pops: mode, name, pattern, rate, payload

        vm.stack.push(Value::Int(0)); // payload
        vm.stack.push(Value::Int(50)); // rate
        vm.stack.push(Value::Str("test".to_string())); // pattern
        vm.stack.push(Value::Str("Virus".to_string())); // name
        vm.stack.push(Value::Int(0)); // mode

        vm.execute_gene_inner(OpCode::Infect, &[]);

        assert_eq!(vm.virus_library.len(), MAX_VIRUSES);
        assert!(vm.output.last().unwrap().contains("Virus library full"));
    }

    #[test]
    fn test_brainfuck_output_limit() {
        let mut vm = make_vm();

        // Create BF code that outputs '.' forever (until cycle limit)
        // +[.+.]
        let bf_code = "+[.+.]".to_string();
        let input = "".to_string();

        vm.stack.push(Value::Str(bf_code));
        vm.stack.push(Value::Str(input));

        vm.execute_gene_inner(OpCode::Brainfuck, &[]);

        if let Some(Value::Str(output)) = vm.stack.pop() {
            assert!(output.len() <= MAX_BRAINFUCK_OUTPUT);
        } else {
            panic!("Brainfuck did not return string");
        }
    }
}
