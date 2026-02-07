#![cfg(feature = "nova")]

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_egregore_tithe() {
    // [ push(10) tithe() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::EgregoreTithe,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.energy = 50;

    vm.step(); // Push
    vm.step(); // Tithe

    assert_eq!(vm.egregore.faith, 10);
    // 50 - 1(step1) - 1(step2) - 10(tithe) = 38
    assert_eq!(vm.energy, 38);
}

#[test]
fn test_egregore_channel() {
    // [ push("comms") push(42) channel() push("comms") query() ]
    // stack ops:
    // push("comms") -> ["comms"]
    // push(42) -> ["comms", 42]
    // channel() -> pops 42, pops "comms". sends 42 to "comms". stack: []
    // push("comms") -> ["comms"]
    // query() -> pops "comms", pushes 42. stack: [42]

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("comms".to_string())],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::EgregoreChannel,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("comms".to_string())],
        },
        Gene {
            op: OpCode::EgregoreQuery,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    // Execute 5 instructions
    for _ in 0..5 {
        vm.step();
    }

    assert_eq!(vm.stack.last(), Some(&Value::Int(42)));
}

#[test]
fn test_egregore_summon_fail() {
    // [ push("Rain") summon() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Rain".to_string())],
        },
        Gene {
            op: OpCode::EgregoreSummon,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);

    vm.step(); // Push
    vm.step(); // Summon

    // Should fail (0 faith)
    assert!(vm.output.iter().any(|s| s.contains("Insufficient faith")));
}

#[test]
fn test_egregore_summon_success() {
    // [ push("Rain") summon() ] but with faith hacked in
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::String("Rain".to_string())],
        },
        Gene {
            op: OpCode::EgregoreSummon,
            args: vec![],
        },
    ];
    let mut vm = make_vm(genes);
    vm.egregore.faith = 200;

    vm.step(); // Push
    vm.step(); // Summon

    assert!(vm.output.iter().any(|s| s.contains("Summoned RAIN")));
    // Check moisture grid increased
    // Assuming Rain adds 50 moisture to all cells
    let moisture = vm.moisture_grid[0][0];
    assert!(moisture >= 50);
}
