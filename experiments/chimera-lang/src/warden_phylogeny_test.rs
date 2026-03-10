#[cfg(all(test, feature = "phylogeny"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::fs;


    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_exploit_path_traversal_read() {
        // Setup: Create a secret file outside the "sandbox" (simulated)
        let root = std::env::temp_dir().join("chimera_test_exploit_read");
        let safe_dir = root.join("safe");
        let secret_file = root.join("secret.txt");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&safe_dir).unwrap();
        fs::write(&secret_file, "TOP SECRET DATA").unwrap();

        // Attack: Try to read "../secret.txt"
        // [ push("../secret.txt") sequencing() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("../secret.txt".to_string())],
            },
            Gene {
                op: OpCode::Sequencing,
                args: vec![],
            },
        ];

        let mut vm = make_vm();
        // Override sandbox root for testing
        vm.sandbox_root = safe_dir.clone();
        vm.inject_genes(genes);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            vm.step(); // push
            vm.step(); // sequencing
        }));

        let _ = fs::remove_dir_all(&root);

        assert!(result.is_ok());

        if let Some(Value::Str(content)) = vm.stack.pop() {
            if content == "TOP SECRET DATA" {
                panic!("VULNERABILITY CONFIRMED: Managed to read ../secret.txt");
            } else {
                println!("Read content: '{}'", content);
                println!("VM Output: {:?}", vm.output);
                // If content is empty, maybe read failed?
            }
        } else {
            println!("Stack empty or not string");
            println!("VM Output: {:?}", vm.output);
        }
    }

    #[test]
    fn test_exploit_path_traversal_write() {
        let root = std::env::temp_dir().join("chimera_test_exploit_write");
        let safe_dir = root.join("safe");
        let target_file = root.join("pwned.txt");

        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&safe_dir).unwrap();

        // Attack: Try to write "../pwned.txt"
        // [ push("../pwned.txt") push("hacked") synthesize() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("../pwned.txt".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("hacked".to_string())],
            },
            Gene {
                op: OpCode::Synthesize,
                args: vec![],
            },
        ];

        let mut vm = make_vm();
        // Override sandbox root for testing
        vm.sandbox_root = safe_dir.clone();
        vm.inject_genes(genes);

        vm.step(); // push path
        vm.step(); // push content
        vm.step(); // synthesize

        let exists = target_file.exists();
        let _ = fs::remove_dir_all(&root);

        if exists {
            panic!("VULNERABILITY CONFIRMED: Managed to write ../pwned.txt");
        } else {
            println!("VM Output: {:?}", vm.output);
        }
    }
}
