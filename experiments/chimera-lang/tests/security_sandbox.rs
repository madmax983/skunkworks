#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_scavenge_sandbox() {
        // --- Setup ---
        // Create a secret file outside the sandbox (in the CWD)
        let secret_path = PathBuf::from("secret_outside.txt");
        let mut f = File::create(&secret_path).unwrap();
        f.write_all(b"SECRET_DATA").unwrap();

        // Setup VM with a sandbox subdirectory
        let sandbox_dir = PathBuf::from("sandbox_test");
        std::fs::create_dir_all(&sandbox_dir).unwrap();

        // Create a safe file INSIDE the sandbox
        let safe_path = sandbox_dir.join("safe.txt");
        let mut f = File::create(&safe_path).unwrap();
        f.write_all(b"SAFE_DATA").unwrap();

        // --- Case 1: Try to read outside file (Should Fail) ---
        let genes_exploit = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("../secret_outside.txt".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Scavenge,
                args: vec![],
            },
        ];

        let dna_exploit = Dna {
            helix: Helix {
                strands: vec![Strand {
                    genes: genes_exploit,
                }],
            },
        };

        let mut vm = ChimeraVM::new(dna_exploit);
        vm.sandbox_root = sandbox_dir.canonicalize().unwrap(); // Set sandbox

        vm.step(); // push path
        vm.step(); // push len
        vm.step(); // scavenge

        // Expect failure (-1)
        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert_eq!(
                idx, -1,
                "SECURITY BREACH: Scavenge read a file outside the sandbox!"
            );
        } else {
            panic!("Stack underflow on exploit attempt");
        }

        // --- Case 2: Try to read inside file (Should Succeed) ---
        let genes_safe = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("safe.txt".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Scavenge,
                args: vec![],
            },
        ];

        let dna_safe = Dna {
            helix: Helix {
                strands: vec![Strand { genes: genes_safe }],
            },
        };

        let mut vm = ChimeraVM::new(dna_safe);
        vm.sandbox_root = sandbox_dir.canonicalize().unwrap();

        vm.step(); // push path
        vm.step(); // push len
        vm.step(); // scavenge

        // Expect success (idx >= 0)
        if let Some(Value::Int(idx)) = vm.stack.pop() {
            assert!(
                idx >= 0,
                "Failed to read valid file inside sandbox. Code: {}",
                idx
            );
        } else {
            panic!("Stack underflow on safe attempt");
        }

        // --- Cleanup ---
        let _ = std::fs::remove_file(secret_path);
        let _ = std::fs::remove_dir_all(sandbox_dir);
    }
}
