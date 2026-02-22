use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_zeta_agent_lisp() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Grid: ζ ~ ( ~ + ~ 1 ~ 2 ~ )
    // Locations:
    // (5, 5) -> ζ
    // (5, 6) -> ~
    // (5, 7) -> (
    // (5, 8) -> ~
    // (5, 9) -> +
    // (5, 10) -> ~
    // (5, 11) -> 1
    // (5, 12) -> ~
    // (5, 13) -> 2
    // (5, 14) -> ~
    // (5, 15) -> )

    vm.grid[5][5] = Value::Str("ζ".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());
    vm.grid[5][7] = Value::Str("(".to_string());
    vm.grid[5][8] = Value::Str("~".to_string());
    vm.grid[5][9] = Value::Str("+".to_string());
    vm.grid[5][10] = Value::Str("~".to_string());
    vm.grid[5][11] = Value::Int(1);
    vm.grid[5][12] = Value::Str("~".to_string());
    vm.grid[5][13] = Value::Int(2);
    vm.grid[5][14] = Value::Str("~".to_string());
    vm.grid[5][15] = Value::Str(")".to_string());

    // Execute Tick
    // 1. scan_grid_rules finds ζ at 5,5
    // 2. process_agents calls process_zeta_agent
    // 3. process_zeta_agent scans East, finds tokens "(", "+", "1", "2", ")"
    // 4. Compiles "( + 1 2 )" -> [Push(1), Push(2), Add]
    // 5. Executes -> Stack [3]
    exec_prologue_tick(&mut vm);

    // Find the agent. It should have moved to 5,6 (onto the wire)
    // agents list is in prologue_state.agents
    let agent = vm.prologue_state.agents.iter().find(|a| {
        // It might be at 5,6 or 5,5 depending on move logic
        // If it moved, it is at 5,6
        a.y == 5 && (a.x == 5 || a.x == 6)
    }).expect("Agent not found");

    // Check stack
    assert_eq!(agent.stack.len(), 1, "Stack should have 1 item");
    if let Value::Int(v) = agent.stack[0] {
        assert_eq!(v, 3, "Expected 1+2=3");
    } else {
        panic!("Expected Int(3) on stack, got {:?}", agent.stack[0]);
    }
}

#[test]
fn test_zeta_agent_bracket_list() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Grid: ζ ~ [ ~ + ~ 5 ~ 5 ~ ]
    vm.grid[5][5] = Value::Str("ζ".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());
    vm.grid[5][7] = Value::Str("[".to_string());
    vm.grid[5][8] = Value::Str("~".to_string());
    vm.grid[5][9] = Value::Str("+".to_string());
    vm.grid[5][10] = Value::Str("~".to_string());
    vm.grid[5][11] = Value::Int(5);
    vm.grid[5][12] = Value::Str("~".to_string());
    vm.grid[5][13] = Value::Int(5);
    vm.grid[5][14] = Value::Str("~".to_string());
    vm.grid[5][15] = Value::Str("]".to_string());

    exec_prologue_tick(&mut vm);

    let agent = vm.prologue_state.agents.iter().find(|a| a.y == 5).expect("Agent not found");

    assert_eq!(agent.stack.len(), 1);
    if let Value::Int(v) = agent.stack[0] {
        assert_eq!(v, 10);
    } else {
        panic!("Expected Int(10) on stack, got {:?}", agent.stack[0]);
    }
}
