#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use std::io::Write;
    use std::fs::File;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_scavenge() {
        let path = "scavenge_test.bin";
        let mut file = File::create(path).unwrap();
        // Write some bytes.
        // We don't rely on specific byte-to-opcode mapping here, just that it produces *some* opcodes.
        file.write_all(&[0, 1, 2, 3]).unwrap();

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(path.to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            },
            Gene {
                op: OpCode::Scavenge,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        vm.step(); // Push path
        vm.step(); // Push len
        vm.step(); // Scavenge

        // Check output for errors
        for msg in &vm.output {
            println!("{}", msg);
        }

        // Check if a new strand was created.
        assert_eq!(vm.dna.helix.strands.len(), 2, "Expected a new strand to be created via Scavenge");
        assert_eq!(vm.dna.helix.strands[1].genes.len(), 4, "Expected 4 genes in the new strand");

        // Cleanup
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn test_digest() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // offset
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // len
            },
            Gene {
                op: OpCode::Digest,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        vm.step(); // Push offset
        vm.step(); // Push len
        vm.step(); // Digest

        // Check output for errors
        for msg in &vm.output {
            println!("{}", msg);
        }

        assert_eq!(vm.dna.helix.strands.len(), 2, "Expected a new strand to be created via Digest");
        assert_eq!(vm.dna.helix.strands[1].genes.len(), 10, "Expected 10 genes in the new strand");
    }
}
