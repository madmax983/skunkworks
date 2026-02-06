#![cfg(feature = "nova")]

use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;
use crate::vm::nova_semantic;

#[test]
fn test_semantic_snapshot() {
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
    ];
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Modify state
    vm.energy = 42;
    vm.step(); // Should execute push(10)

    let snap = nova_semantic::snapshot(&vm);

    // Verify Metrics
    assert_eq!(snap.app, "chimera-vm");
    if let Some(tui_semantic::PropValue::Int(e)) = snap.metrics.get("energy") {
        assert_eq!(*e, 41); // 42 - 1 step cost
    } else {
        panic!("Energy metric missing or wrong type");
    }

    // Verify Organism Entity
    let organism = snap.entities.iter().find(|e| e.kind == "organism").expect("Organism entity missing");
    assert_eq!(organism.id.as_deref(), Some("root"));

    // Verify Stack Context
    let stack_top = snap.entities.iter().find(|e| e.kind == "stack_top");
    assert!(stack_top.is_some());
    if let Some(tui_semantic::PropValue::Text(val)) = stack_top.unwrap().props.get("value") {
        assert_eq!(val, "10");
    } else {
        panic!("Stack top value missing or wrong type");
    }
}
