#![cfg(all(feature = "elektra", feature = "oracle"))]

use chimera_lang::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_oracle_gate() {
    let setup_genes = vec![
        // Wire at 0,0
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Wire at 0,2
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::GWrite, args: vec![] },

        // Assert power_on
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("power_on".to_string())] },
        Gene { op: OpCode::Assert, args: vec![] },

        // Battery at 0,0
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Battery, args: vec![] },

        // OracleGate at 0,1
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("power_on".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::OracleGate, args: vec![] },

        // Ground at 0,2
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Ground, args: vec![] },

        // Jump to Strand 1 (Idle Loop)
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
    ];

    let idle_genes = vec![
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
    ];

    let dna = Dna { helix: Helix { strands: vec![Strand { genes: setup_genes }, Strand { genes: idle_genes }] } };
    let mut vm = ChimeraVM::new(dna);

    // Give enough energy (infinite loop needs energy)
    vm.energy = 1000;

    // Run setup (enough ticks to finish strand 0 and enter strand 1)
    for _ in 0..50 { vm.step(); }

    let v_gate = vm.voltage_grid[0][1];
    println!("Voltage with power_on: {}", v_gate);
    assert!(v_gate > 10.0, "Gate should conduct (Voltage > 10.0), got {}", v_gate);

    // Retract
    // This injects into Strand 1 (current IP)
    let retract_genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("power_on".to_string())] },
        Gene { op: OpCode::Retract, args: vec![] },
    ];
    vm.inject_genes(retract_genes);

    // Step to execute injection and retraction
    for _ in 0..50 { vm.step(); }

    let v_gate_off = vm.voltage_grid[0][1];
    println!("Voltage with power_off: {}", v_gate_off);

    assert!(v_gate_off < v_gate, "Gate should block (Voltage {} < {})", v_gate_off, v_gate);
    assert!(v_gate_off < 5.0, "Gate should be effectively off");
}
