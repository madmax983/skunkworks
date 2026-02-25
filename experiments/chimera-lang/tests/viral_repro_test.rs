#![cfg(feature = "nova")]

use chimera_lang::prelude::*;
use std::path::PathBuf;

#[test]
fn test_self_replicate() {
    // Setup: Create a temporary directory for the test
    let temp_dir = std::env::temp_dir().join("chimera_viral_test");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }
    std::fs::create_dir_all(&temp_dir).unwrap();

    // Create DNA with SelfReplicate
    // [ push(1000) consume() self_replicate() ]
    // Consume energy to afford replication cost (100)
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1000)],
        },
        Gene {
            op: OpCode::Consume,
            args: vec![],
        },
        Gene {
            op: OpCode::SelfReplicate,
            args: vec![],
        },
    ];

    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    // Override sandbox root for safety/test isolation
    vm.sandbox_root = temp_dir.clone();

    // Run
    // 1. Push
    vm.step();
    // 2. Consume
    vm.step();
    // 3. SelfReplicate
    vm.step();

    // Check Output
    for line in &vm.output {
        println!("{}", line);
    }

    // Assert Success on Stack (1)
    if let Some(val) = vm.stack.pop() {
        assert_eq!(val, Value::Int(1), "SelfReplicate failed (returned 0)");
    } else {
        panic!("Stack empty after SelfReplicate");
    }

    // Verify File Creation
    let viral_lab = temp_dir.join("viral_lab");
    assert!(viral_lab.exists());

    let entries: Vec<PathBuf> = std::fs::read_dir(&viral_lab)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();

    assert_eq!(entries.len(), 1, "Expected 1 file created");
    let file_path = &entries[0];

    // Verify Content
    let content = std::fs::read_to_string(file_path).unwrap();
    println!("Replica Content:\n{}", content);

    assert!(content.contains("strand strand_0 {"));
    assert!(content.contains("push(1000)"));
    assert!(content.contains("self_replicate"));

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
}
