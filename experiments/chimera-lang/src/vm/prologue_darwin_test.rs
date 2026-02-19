use super::*;
use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::ChimeraVM;
use crate::vm::Value;
use crate::vm::prologue::exec_prologue_tick;

#[test]
fn test_darwin_rune() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Darwinian Circuit
    // (4,4) = 10 (Goal Value)
    // (4,5) = !  (Source for Goal) -> Signal at (4,5)
    //
    // (5,3) = 1  (Trigger Value)
    // (5,4) = !  (Source for Trigger) -> Signal at (5,4)
    //
    // (5,5) = ∞  (The Rune)
    //
    // (6,5) = 0  (Subject)
    // (6,6) = !  (Source for Subject) -> Signal at (6,6)
    // (5,6) = ~  (Wire to carry Subject to ∞) -> Reads (6,6)

    vm.grid[4][4] = Value::Int(10);
    vm.grid[4][5] = Value::Str("!".to_string());

    vm.grid[5][3] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("∞".to_string());

    vm.grid[5][6] = Value::Str("~".to_string());

    vm.grid[6][5] = Value::Int(0);
    vm.grid[6][6] = Value::Str("!".to_string());

    // Run tick
    exec_prologue_tick(&mut vm);

    // We expect mutation because 0 != 10
    // Check logs for DARWIN message
    let mutated = vm.output.iter().any(|s| s.contains("DARWIN: Mutation"));
    assert!(mutated, "Darwin rune failed to trigger mutation. Logs: {:?}", vm.output);

    // If we want to test Success case:
    // Reset output
    vm.output.clear();

    // Set Subject to 10
    vm.grid[6][5] = Value::Int(10);

    // Run tick
    exec_prologue_tick(&mut vm);

    let success = vm.output.iter().any(|s| s.contains("DARWIN: Success"));
    assert!(success, "Darwin rune failed to recognize success. Logs: {:?}", vm.output);

    // Check if ∞ emitted success signal (1) at (5,5)
    // Signal grid is transient, so we can't check it easily after tick unless we spy on it.
    // But the log confirms logic execution.
}
