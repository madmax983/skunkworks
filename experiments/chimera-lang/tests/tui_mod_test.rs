use chimera_lang::vm::{ChimeraVM, TuiEvent, Value};
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_tui_mod_glitch() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] }, // Value
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },  // Mode 0 (Glitch)
        Gene { op: OpCode::TuiMod, args: vec![] },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step(); // Push 50
    vm.step(); // Push 0
    vm.step(); // TuiMod

    assert_eq!(vm.tui_events.len(), 1);
    match vm.tui_events[0] {
        TuiEvent::Glitch(v) => assert_eq!(v, 0.5),
        _ => panic!("Expected Glitch"),
    }
}

#[test]
fn test_tui_mod_shake() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] }, // Value
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },  // Mode 1 (Shake)
        Gene { op: OpCode::TuiMod, args: vec![] },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(vm.tui_events.len(), 1);
    match vm.tui_events[0] {
        TuiEvent::Shake(v) => assert_eq!(v, 5.0),
        _ => panic!("Expected Shake"),
    }
}

#[test]
fn test_tui_mod_message() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("CHAOS".to_string())] }, // Value
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },  // Mode 2 (Message)
        Gene { op: OpCode::TuiMod, args: vec![] },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step();
    vm.step();
    vm.step();

    assert_eq!(vm.tui_events.len(), 1);
    match &vm.tui_events[0] {
        TuiEvent::Message(s) => assert_eq!(s, "CHAOS"),
        _ => panic!("Expected Message"),
    }
}
