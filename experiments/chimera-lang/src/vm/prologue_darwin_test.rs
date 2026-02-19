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
    // Goal: 10
    // (4,4) = 10
    // (4,5) = ! (Reads West 4,4) -> Signal at 4,5 (North of ∞)

    vm.grid[4][4] = Value::Int(10);
    vm.grid[4][5] = Value::Str("!".to_string());

    // Trigger: 1
    // (5,3) = 1
    // (5,4) = ! (Reads West 5,3) -> Signal at 5,4 (West of ∞)

    vm.grid[5][3] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    // Rune
    vm.grid[5][5] = Value::Str("∞".to_string());

    // Actual (Subject): 0
    // We need signal at East (5,6).
    // Use Wire at 5,6. Reads South (6,6).
    // ! at 6,6. Reads West (6,5).
    // (6,5) = 0.

    vm.grid[5][6] = Value::Str("~".to_string());
    vm.grid[6][6] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Int(0);

    // Run tick
    // 0 != 10. Expect Mutation.
    exec_prologue_tick(&mut vm);

    let mutated = vm.output.iter().any(|s| s.contains("DARWIN: Mutation"));
    assert!(mutated, "Darwin rune failed to trigger mutation. Logs: {:?}", vm.output);

    // Reset output
    vm.output.clear();

    // Set Subject to 10
    vm.grid[6][5] = Value::Int(10);

    // Run tick
    // 10 == 10. Expect Success.
    exec_prologue_tick(&mut vm);

    let success = vm.output.iter().any(|s| s.contains("DARWIN: Success"));
    assert!(success, "Darwin rune failed to recognize success. Logs: {:?}", vm.output);
}
