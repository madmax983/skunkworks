#!/bin/bash
cat << 'EOF2' > experiments/chimera-lang/src/nova_akashic_test.rs
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
        let path = ".chimera_akashic_test.json".to_string();
        vm1.akashic.file_path = path.clone();
        vm1.akashic.corrupted = false;

        for _ in 0..10 {
            vm1.step();
            if vm1.halted {
                break;
            }
        }
        println!("{:?}", vm1.output);

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
        vm2.akashic = crate::vm::akashic::AkashicRecords::load_from(&path).unwrap();

        for _ in 0..10 {
            vm2.step();
            if vm2.halted {
                break;
            }
        }

        assert_eq!(vm2.stack.len(), 1);
        assert_eq!(vm2.stack[0], Value::Int(42));

        let _ = fs::remove_file(path);
    }
}
EOF2
