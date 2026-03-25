use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::ChimeraVM;
use crate::vm::Value;

#[test]
fn test_darwin_rune() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.metamorphism_enabled = false;
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
    // The mutation cone of ∞ is y + 1..=5, x - 2..=2.
    // This is y=6..=10, x=3..=7.
    // We construct a wire path outside the cone:
    // ! at (4,8) reads from (4,7). (4,7) = 0.
    // ~ at (5,8) connects to (4,8).
    // ~ at (5,7) connects to (5,8).
    // ~ at (5,6) connects to (5,7).
    // ∞ at (5,5) reads actual from (5,6).
    // None of these cells are in the y=6..10 range.

    vm.grid[4][7] = Value::Int(0);
    vm.grid[4][8] = Value::Str("!".to_string());
    vm.grid[5][8] = Value::Str("~".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());

    // Run tick
    // 0 != 10. Expect Mutation.
    exec_prologue_tick(&mut vm);

    let mutated = vm.output.iter().any(|s| s.contains("DARWIN: Mutation"));
    assert!(
        mutated,
        "Darwin rune failed to trigger mutation. Logs: {:?}",
        vm.output
    );

    // Reset output
    vm.output.clear();

    // Set Subject to 10
    vm.grid[4][7] = Value::Int(10);

    // Run tick
    // 10 == 10. Expect Success.
    exec_prologue_tick(&mut vm);

    let success = vm.output.iter().any(|s| s.contains("DARWIN: Success"));
    assert!(
        success,
        "Darwin rune failed to recognize success. Logs: {:?}",
        vm.output
    );
}
