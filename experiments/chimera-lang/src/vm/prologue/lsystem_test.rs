use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::lsystem::{LSystemState, process_lsystem_agent};
use crate::vm::prologue::{PrologueAgent, normalize_coords};
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use std::collections::HashMap;
use std::str::FromStr;

#[test]
fn test_lsystem_state_parsing() {
    // Basic Parsing
    let state_str = "🌲:F:F=F+F:2";
    let state = LSystemState::from_str(state_str).unwrap();
    assert_eq!(state.axiom, "F");
    assert_eq!(state.rules.get(&'F'), Some(&"F+F".to_string()));
    assert_eq!(state.iterations, 2);
    assert_eq!(state.current_string, "F");
    assert_eq!(state.current_iteration, 0);

    // Parsing with execution state
    let state_str_full = "🌲:F:F=FF:3:5:1:FFF";
    let state_full = LSystemState::from_str(state_str_full).unwrap();
    assert_eq!(state_full.pc, 5);
    assert_eq!(state_full.dir, 1);
    assert_eq!(state_full.current_string, "FFF");
    // Iteration inference logic check
    assert_eq!(state_full.current_iteration, 3);
    assert!(state_full.stack.is_empty());

    // Parsing with execution state AND stack
    let state_str_stack = "🌲:F:F=FF:3:5:1:FFF:10,10,2|15,15,0";
    let state_stack = LSystemState::from_str(state_str_stack).unwrap();
    assert_eq!(state_stack.stack.len(), 2);
    assert_eq!(state_stack.stack[0], (10, 10, 2));
    assert_eq!(state_stack.stack[1], (15, 15, 0));
}

#[test]
fn test_lsystem_growth() {
    let mut rules = HashMap::new();
    rules.insert('A', "AB".to_string());
    rules.insert('B', "A".to_string());

    let mut state = LSystemState {
        axiom: "A".to_string(),
        rules,
        iterations: 2,
        current_string: "A".to_string(),
        current_iteration: 0,
        ..Default::default()
    };

    let agent = PrologueAgent {
        x: 0,
        y: 0,
        state: state.to_value(),
        stack: Vec::new(),
    };

    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    let grid_snapshot = vm.grid.clone();

    // Tick 1: Growth A -> AB
    let (updated_agent, _) = process_lsystem_agent(&mut vm, &agent, &grid_snapshot).unwrap();
    let updated_state = if let Value::Str(s) = &updated_agent.state {
        LSystemState::from_str(s).unwrap()
    } else {
        panic!("Invalid state");
    };
    assert_eq!(updated_state.current_string, "AB");
    assert_eq!(updated_state.current_iteration, 1);

    // Tick 2: Growth AB -> ABA
    let (updated_agent_2, _) = process_lsystem_agent(&mut vm, &updated_agent, &grid_snapshot).unwrap();
    let updated_state_2 = if let Value::Str(s) = &updated_agent_2.state {
        LSystemState::from_str(s).unwrap()
    } else {
        panic!("Invalid state");
    };
    assert_eq!(updated_state_2.current_string, "ABA");
    assert_eq!(updated_state_2.current_iteration, 2);

    // Tick 3: Execution (Should not grow anymore)
    let (updated_agent_3, _) = process_lsystem_agent(&mut vm, &updated_agent_2, &grid_snapshot).unwrap();
    let updated_state_3 = if let Value::Str(s) = &updated_agent_3.state {
        LSystemState::from_str(s).unwrap()
    } else {
        panic!("Invalid state");
    };
    assert_eq!(updated_state_3.current_string, "ABA"); // No change
    assert_eq!(updated_state_3.current_iteration, 2);
}

#[test]
fn test_lsystem_execution() {
    // "F+F" -> Move Forward, Turn Right, Move Forward
    // Start at 5,5 Facing North (0)
    // F: (4, 5)
    // +: Facing East (1)
    // F: (4, 6)

    let mut state = LSystemState {
        axiom: "F+F".to_string(),
        rules: HashMap::new(),
        iterations: 0,
        current_string: "F+F".to_string(),
        current_iteration: 0,
        pc: 0,
        dir: 0, // North
        ..Default::default()
    };

    let agent = PrologueAgent {
        x: 5,
        y: 5,
        state: state.to_value(),
        stack: Vec::new(),
    };

    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    let mut grid_snapshot = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE]; // Empty grid

    // Tick 1: Execute 'F' (Move North)
    let (updated_agent, move_target) = process_lsystem_agent(&mut vm, &agent, &grid_snapshot).unwrap();

    // Check movement
    assert_eq!(move_target, Some((4, 5)));
    // Check drawing trail at old pos
    assert_eq!(vm.grid[5][5], Value::Str("*".to_string()));

    // Check state update
    let updated_state = if let Value::Str(s) = &updated_agent.state {
        LSystemState::from_str(s).unwrap()
    } else {
        panic!("Invalid state");
    };
    assert_eq!(updated_state.pc, 1); // Advanced past 'F'

    // Tick 2: Execute '+' (Turn) and 'F' (Move East)
    // Note: The loop consumes non-move commands. So it consumes '+' then sees 'F' and moves.
    // Start from NEW position (4, 5)
    let agent_2 = PrologueAgent {
        x: 5, // Grid logic would update this
        y: 4,
        state: updated_agent.state,
        stack: Vec::new(),
    };

    let (updated_agent_2, move_target_2) = process_lsystem_agent(&mut vm, &agent_2, &grid_snapshot).unwrap();

    // Check movement: Was North (0), Turned Right (+), Now East (1).
    // From (4, 5) East -> (4, 6)
    assert_eq!(move_target_2, Some((4, 6)));

    // Check trail at (4, 5)
    assert_eq!(vm.grid[4][5], Value::Str("*".to_string()));

    let updated_state_2 = if let Value::Str(s) = &updated_agent_2.state {
        LSystemState::from_str(s).unwrap()
    } else {
        panic!("Invalid state");
    };
    // PC should have advanced past '+' (1) and 'F' (2) -> 3
    assert_eq!(updated_state_2.pc, 3);
    assert_eq!(updated_state_2.dir, 1);
}

#[test]
fn test_lsystem_branching_persistence() {
    // "F[+F]F"
    // 1. Move F (4,5)
    // 2. Push [ -> Stack should save (4,5, North)
    // 3. Turn + -> East
    // 4. Move F -> (4,6)
    // 5. Pop ] -> Restore to (4,5, North)
    // 6. Move F -> (3,5)

    let mut state = LSystemState {
        axiom: "F[+F]F".to_string(),
        rules: HashMap::new(),
        iterations: 0,
        current_string: "F[+F]F".to_string(),
        current_iteration: 0,
        pc: 0,
        dir: 0, // North
        ..Default::default()
    };

    let dna = Dna { evolution_config: None, helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    let mut grid_snapshot = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];

    // Tick 1: F -> Move to (4,5)
    let agent = PrologueAgent { x: 5, y: 5, state: state.to_value(), stack: Vec::new() };
    let (updated_agent, move_target) = process_lsystem_agent(&mut vm, &agent, &grid_snapshot).unwrap();
    assert_eq!(move_target, Some((4, 5)));

    // Tick 2: [ + F -> Push, Turn, Move to (4,6)
    // Agent is now at (4,5)
    let agent_2 = PrologueAgent { x: 5, y: 4, state: updated_agent.state, stack: Vec::new() };
    let (updated_agent_2, move_target_2) = process_lsystem_agent(&mut vm, &agent_2, &grid_snapshot).unwrap();

    // Should have moved East to (4,6)
    assert_eq!(move_target_2, Some((4, 6)));

    // Verify stack persistence in state string
    if let Value::Str(s) = &updated_agent_2.state {
        // We pushed at (4,5,0) before turning
        // Stack should contain (5, 4, 0) -- wait, agent x=5, y=4.
        // Stack saves current_pos which is (4,5) in (y,x) coords?
        // My test setup agent_2 x=5, y=4.
        // Logic: current_pos = (agent.y, agent.x) = (4, 5).
        // Push (x, y, dir) = (5, 4, 0).
        assert!(s.contains("5,4,0"));
    } else {
        panic!("Invalid state format");
    }

    // Tick 3: ] F -> Pop, Restore (4,5,N), Move F -> (3,5)
    // Agent is currently at (4,6)
    let agent_3 = PrologueAgent { x: 6, y: 4, state: updated_agent_2.state, stack: Vec::new() };

    // We expect it to restore position to (4,5) immediately on ] command.
    // The `break` in execution loop on `]` ensures we return the teleport target.
    let (updated_agent_3, move_target_3) = process_lsystem_agent(&mut vm, &agent_3, &grid_snapshot).unwrap();

    // Should teleport back to (4,5)
    assert_eq!(move_target_3, Some((4, 5)));

    // Verify stack is empty (popped)
    let final_state = LSystemState::from_str(updated_agent_3.state.as_str().unwrap()).unwrap();
    assert!(final_state.stack.is_empty());
    // Dir should be restored to 0 (North)
    assert_eq!(final_state.dir, 0);
    // PC should be at the last F
    // String: F [ + F ] F
    // Indices:0 1 2 3 4 5
    // Tick 1: Exec 0 (F). PC -> 1.
    // Tick 2: Exec 1 ([), 2 (+), 3 (F). PC -> 4.
    // Tick 3: Exec 4 (]). PC -> 5. Returns teleport.
    assert_eq!(final_state.pc, 5);

    // Tick 4: F -> Move North to (3,5)
    // Agent back at (4,5)
    let agent_4 = PrologueAgent { x: 5, y: 4, state: updated_agent_3.state, stack: Vec::new() };
    let (_, move_target_4) = process_lsystem_agent(&mut vm, &agent_4, &grid_snapshot).unwrap();
    assert_eq!(move_target_4, Some((3, 5)));
}
