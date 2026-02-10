use chimera_lang::vm::ChimeraVM;
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;
use std::fs::File;
use std::io::{Write, Read};

#[test]
fn test_akashic_corruption_prevention() {
    // 1. Create a corrupted Akashic Record file
    let filename = ".chimera_akashic.json";
    let corrupted_content = "INVALID JSON CONTENT";
    {
        let mut file = File::create(filename).expect("Failed to create test file");
        file.write_all(corrupted_content.as_bytes()).expect("Failed to write test file");
    }

    // 2. Create DNA program: [ Push("value"), Push("key"), AkashicWrite ]
    // Stack order for AkashicWrite: [ ..., key, value ] (pops value, then pops key)
    // Wait, let's check stack order in akashic.rs:
    // let value = vm.stack.pop().unwrap();
    // let key_val = vm.stack.pop().unwrap();
    // So stack should be [ key, value ] (top is value)

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("test_key".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("test_value".to_string())],
        },
        Gene {
            op: OpCode::AkashicWrite,
            args: vec![],
        },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // 3. Run VM
    // It should try to read the file, fail (because it's invalid JSON),
    // and ideally NOT overwrite it.
    // In the BROKEN version, it returns empty map, adds the key, and OVERWRITES the file.
    // In the FIXED version, it should return Err and abort.

    // We run until halted or for enough steps.
    for _ in 0..10 {
        vm.step();
        if vm.halted {
            break;
        }
    }

    // 4. Verify file content
    let mut file = File::open(filename).expect("Failed to open test file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read test file");

    // Clean up
    let _ = std::fs::remove_file(filename);

    // Assert that content is UNCHANGED (still corrupted)
    // If it was overwritten, it would be {"test_key":"test_value"}
    assert_eq!(content, corrupted_content, "Akashic Record was overwritten! Data Loss detected!");

    // Assert that VM reported an error (optional but good)
    // The exact error message depends on implementation, but should contain "Error" or "fail"
    let error_found = vm.output.iter().any(|s| s.to_lowercase().contains("error") || s.contains("fail"));
    assert!(error_found, "VM did not report an error for corrupted file. Output: {:?}", vm.output);
}
