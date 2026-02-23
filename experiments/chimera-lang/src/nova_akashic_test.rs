#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::fs;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_akashic_record() {
        // 1. Write to Record
        // [ push("test_key") push(42) akashic_write() ]
        let write_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("test_key".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::AkashicWrite,
                args: vec![],
            },
        ];

        let mut vm1 = ChimeraVM::new(make_dna(write_genes));
        let path = vm1.akashic.file_path.clone();

        while !vm1.halted && vm1.ip.0 < 1 {
            vm1.step();
        }

        // 2. Read from Record (New VM)
        // [ push("test_key") akashic_read() ]
        let read_genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("test_key".to_string())],
            },
            Gene {
                op: OpCode::AkashicRead,
                args: vec![],
            },
        ];

        let mut vm2 = ChimeraVM::new(make_dna(read_genes));
        // Force vm2 to load from vm1's unique path
        vm2.akashic = crate::vm::akashic::AkashicRecords::load_from(&path).unwrap();

        while !vm2.halted && vm2.ip.0 < 1 {
            vm2.step();
        }

        assert_eq!(vm2.stack.len(), 1);
        assert_eq!(vm2.stack[0], Value::Int(42));

        // Cleanup
        let _ = fs::remove_file(path);
    }
}
