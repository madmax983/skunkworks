#![cfg(feature = "nova")]

use chimera_lang::ast::JunctionType;
use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_philosopher_agent_movement() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Define Goal: explore
    let goal_term = Value::Str("explore".to_string());

    // 2. Define Rule: action(move(east)) :- goal(explore), neighbor(east, 0).
    // Head: action(Junction(Any, ["move", "east"]))
    let head = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("action".to_string()),
            Value::Junction(
                JunctionType::Any,
                vec![
                    Value::Str("move".to_string()),
                    Value::Str("east".to_string()),
                ],
            ),
        ],
    );

    // Body 1: goal("explore")
    let body1 = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("goal".to_string()),
            Value::Str("explore".to_string()),
        ],
    );

    // Body 2: neighbor("east", 0)
    let body2 = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("neighbor".to_string()),
            Value::Str("east".to_string()),
            Value::Int(0),
        ],
    );

    let rule = Value::Junction(
        JunctionType::Any,
        vec![Value::Str("rule".to_string()), head, body1, body2],
    );

    vm.knowledge_base.push(rule);

    // 3. Setup Grid
    // (5, 5) -> Φ
    vm.grid[5][5] = Value::Str("Φ".to_string());
    vm.prologue_state.registers.insert((5, 5), goal_term);

    // (5, 6) is 0 by default.

    // 4. Run Tick
    exec_prologue_tick(&mut vm);

    // 5. Verification
    // Agent should have moved to (5, 6).
    assert_eq!(
        vm.grid[5][6],
        Value::Str("Φ".to_string()),
        "Philosopher should have moved East"
    );
    assert_eq!(vm.grid[5][5], Value::Int(0), "Old position should be empty");
}

#[test]
fn test_graduate_rune() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5, 4) -> @ (Seeker)
    // (5, 5) -> 🎓 (Graduate)
    // (4, 5) -> "find_truth" (Goal String)
    // Expected: (6, 5) -> Φ (Philosopher) with state "find_truth"

    vm.grid[5][4] = Value::Str("@".to_string());
    vm.grid[5][5] = Value::Str("🎓".to_string());
    // Use delayed_signals because prepare_signals clears signal_grid at start of tick
    vm.prologue_state.delayed_signals[4][5] = Some(Value::Str("find_truth".to_string()));

    // Run Tick
    exec_prologue_tick(&mut vm);

    // Verify Spawn
    assert_eq!(vm.grid[6][5], Value::Str("Φ".to_string()));

    // Verify State
    if let Some(state) = vm.prologue_state.registers.get(&(6, 5)) {
        assert_eq!(*state, Value::Str("find_truth".to_string()));
    } else {
        panic!("Philosopher at (6,5) has no state!");
    }
}
