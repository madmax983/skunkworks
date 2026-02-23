use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_logic_agent_unification() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Grid
    // Agent at (5, 5) moving East (0, 1)
    vm.grid[5][5] = Value::Str("∃".to_string());

    // Fact at (5, 7): ( foo bar )
    // (5, 6) is empty space for movement
    // (5, 7) = '('
    // (5, 8) = 'foo'
    // (5, 9) = 'bar'
    // (5, 10) = ')'
    vm.grid[5][7] = Value::Str("(".to_string());
    vm.grid[5][8] = Value::Str("foo".to_string());
    vm.grid[5][9] = Value::Str("bar".to_string());
    vm.grid[5][10] = Value::Str(")".to_string());

    // 2. Initialize Agent State manually
    // Goal Structure: Value::Junction(All, [foo, ?X])
    let goal = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Str("foo".to_string()), Value::Str("?X".to_string())],
    );

    // Register the agent state
    vm.prologue_state.registers.insert(
        (5, 5),
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![goal, Value::Int(0), Value::Int(1)], // Goal, Dy, Dx
        ),
    );

    // 3. Run Ticks

    // Tick 1: Agent moves to (5, 6).
    exec_prologue_tick(&mut vm);

    // Verify move
    assert_eq!(vm.grid[5][6], Value::Str("∃".to_string()));
    assert_eq!(vm.grid[5][5], Value::Int(0));

    // Tick 2: Agent is at (5, 6). Target is (5, 7) which is '('.
    // Logic scans '(', parses (foo bar), unifies with (foo ?X).
    // Success -> Logs, Emits Signal.
    // Agent moves to (5, 7).
    exec_prologue_tick(&mut vm);

    assert_eq!(vm.grid[5][7], Value::Str("∃".to_string()));

    // Check Output
    let output = vm.output.join("\n");
    println!("{}", output);
    assert!(output.contains("∃ Logic: Unified"));
    assert!(output.contains("?X = Str(\"bar\")"));
}

#[test]
fn test_logic_agent_fail_unification() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Agent at (5, 5)
    vm.grid[5][5] = Value::Str("∃".to_string());

    // Fact at (5, 6): ( foo baz )
    vm.grid[5][6] = Value::Str("(".to_string());
    vm.grid[5][7] = Value::Str("foo".to_string());
    vm.grid[5][8] = Value::Str("baz".to_string());
    vm.grid[5][9] = Value::Str(")".to_string());

    // Goal: ( foo bar ) -- Mismatch 'baz' != 'bar'
    let goal = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Str("foo".to_string()), Value::Str("bar".to_string())],
    );

    vm.prologue_state.registers.insert(
        (5, 5),
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![goal, Value::Int(0), Value::Int(1)],
        ),
    );

    // Tick 1: At (5, 5), scans (5, 6). Unifies. Fails.
    exec_prologue_tick(&mut vm);

    let output = vm.output.join("\n");
    assert!(output.contains("∃ Logic: Failed to unify"));

    // Should still move?
    assert_eq!(vm.grid[5][6], Value::Str("∃".to_string()));
}
