#[cfg(feature = "nova")]
#[test]
fn test_paradox_loop() {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    let strand_main = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // ID for TimeLoop
            Gene {
                op: OpCode::TimeLoop,
                args: vec![],
            },
            // Check Stack
            Gene {
                op: OpCode::SLen,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Sub,
                args: vec![],
            },
            // If 0 (Empty), Jump to Paradox Setup (Strand 1)
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(1)],
            },
            // Else (Success), verify value is 999
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            },
            Gene {
                op: OpCode::Sub,
                args: vec![],
            },
            // Should be 0
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(2)],
            }, // Jump to End
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(666)],
            }, // Fail marker
            Gene {
                op: OpCode::Print,
                args: vec![],
            },
            Gene {
                op: OpCode::Apoptosis,
                args: vec![Nucleotide::Number(0)],
            },
        ],
    };

    let strand_paradox = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // ID
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            }, // Value to send back
            Gene {
                op: OpCode::Paradox,
                args: vec![],
            },
        ],
    };

    let strand_end = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("SUCCESS".to_string())],
            },
            Gene {
                op: OpCode::Print,
                args: vec![],
            },
            Gene {
                op: OpCode::Apoptosis,
                args: vec![Nucleotide::Number(2)],
            },
        ],
    };

    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![strand_main, strand_paradox, strand_end],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000; // Give enough energy for time travel costs

    let max_ticks = 100;
    let mut ticks = 0;

    while !vm.halted && ticks < max_ticks {
        vm.step();
        ticks += 1;
    }

    // Verify Output contains SUCCESS
    let success = vm.output.iter().any(|s| s.contains("SUCCESS"));
    assert!(
        success,
        "Paradox loop did not succeed. Output: {:?}",
        vm.output
    );

    // Verify Integrity Damage
    assert!(
        vm.chronos_integrity < 100.0,
        "Chronos Integrity did not decrease: {}. Output: {:?}",
        vm.chronos_integrity,
        vm.output
    );

    // Verify we actually looped (Paradox log in output)
    let looped = vm
        .output
        .iter()
        .any(|s| s.contains("PARADOX: Timeline Rewound"));
    assert!(looped, "Paradox did not trigger");
}
