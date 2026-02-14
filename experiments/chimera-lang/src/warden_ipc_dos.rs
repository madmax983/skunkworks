#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use std::fs;
    use std::path::Path;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_ipc_large_payload() {
        // Use a unique channel to avoid conflicts
        let channel = 9999;
        let ether_dir = Path::new(".chimera_ether");
        let channel_dir = ether_dir.join(channel.to_string());
        if !ether_dir.exists() {
             let _ = fs::create_dir_all(&channel_dir);
        } else {
             let _ = fs::create_dir_all(&channel_dir);
        }

        // 1. Verify mechanism with small file (1KB)
        let small_size = 1024;
        let small_string = "A".repeat(small_size);
        let small_json = format!("{{\"Str\": \"{}\"}}", small_string);
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros();
        let filepath = channel_dir.join(format!("{}_1.json", timestamp));
        fs::write(&filepath, &small_json).unwrap();

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(channel)],
            },
            Gene {
                op: OpCode::Receive,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // Push
        vm.step(); // Receive

        if let Some(val) = vm.stack.pop() {
            match val {
                Value::Str(s) => {
                    assert_eq!(s.len(), 1024);
                }
                _ => panic!("Failed to receive small string. Result: {:?}", val),
            }
        } else {
            panic!("Stack empty after small receive");
        }

        // 2. Verify DoS protection with large file (2MB)
        let size = 2 * 1024 * 1024;
        let large_string = "B".repeat(size);
        let large_json = format!("{{\"Str\": \"{}\"}}", large_string);
        let timestamp2 = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() + 1000;
        let filepath_large = channel_dir.join(format!("{}_2.json", timestamp2));
        fs::write(&filepath_large, &large_json).unwrap();

        let genes2 = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(channel)],
            },
            Gene {
                op: OpCode::Receive,
                args: vec![],
            },
        ];
        let mut vm2 = ChimeraVM::new(make_dna(genes2));
        vm2.step();
        vm2.step();

        if let Some(val) = vm2.stack.pop() {
            match val {
                Value::Str(s) => {
                    // This should NOT happen if hardened
                    assert!(s.len() < 1024 * 1024, "Vulnerability: Read {} bytes (Max 1MB allowed)", s.len());
                }
                Value::Int(0) => {
                     // Expected behavior: Hardened code rejected the file
                }
                _ => panic!("Unexpected result for large file: {:?}", val),
            }
        } else {
             panic!("Stack empty after large receive");
        }

        // Assert error message exists
        let has_error = vm2.output.iter().any(|s| s.contains("Message too large"));
        assert!(has_error, "Expected error message in output, got: {:?}", vm2.output);

        // Cleanup
        let _ = fs::remove_dir_all(&ether_dir);
    }
}
