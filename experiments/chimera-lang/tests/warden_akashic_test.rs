#[cfg(feature = "nova")]
#[test]
fn test_akashic_overwrite_vulnerability() {
    use chimera_lang::vm::{ChimeraVM, Value};
    use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use chimera_lang::opcode::OpCode;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    let filename = ".chimera_akashic.json";
    let path = Path::new(filename);

    // 1. Create a large dummy file (> 10MB)
    {
        let file = File::create(path).expect("Failed to create dummy file");
        file.set_len(11 * 1024 * 1024).expect("Failed to set file length");
    }

    let initial_size = std::fs::metadata(path).unwrap().len();
    assert!(initial_size > 10 * 1024 * 1024, "File should be > 10MB");

    // 2. Setup VM with AkashicWrite("key", "value")
    // stack: "value", "key" -> AkashicWrite
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("key".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("value".to_string())],
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
    while !vm.halted && vm.ip.0 < 1 {
        vm.step();
    }

    // 4. Check file size
    let final_size = std::fs::metadata(path).unwrap().len();

    // Cleanup
    let _ = std::fs::remove_file(path);

    // Assert that the file was NOT wiped (size preserved)
    // Currently, this should FAIL because it IS wiped.
    assert!(
        final_size > 10 * 1024 * 1024,
        "VULNERABILITY CONFIRMED: File was wiped! Size changed from {} to {}",
        initial_size,
        final_size
    );
}
