use chimera_lang::ast::{Dna, Helix, JunctionType};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_logic_agent_gate_mode() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Define Rule in Knowledge Base: gate_and(A, B, Res)
    // We can use OpCode::Rule to add it, or manipulate KB directly.
    // Rule: gate_and(1, 1, 1).
    let rule_head = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("gate_and".to_string()),
            Value::Int(1),
            Value::Int(1),
            Value::Int(1),
        ],
    );
    // Add as a fact (rule with no body)
    vm.knowledge_base.push(rule_head);

    // 2. Setup Grid
    // (5,4) = 1 (West Input)
    // (5,6) = 1 (East Input)
    // (5,5) = ∃ (Logic Agent)
    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][6] = Value::Int(1);
    vm.grid[5][5] = Value::Str("∃".to_string());

    // 3. Configure Agent
    // Goal: gate_and(?W, ?E, ?S)
    let goal = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("gate_and".to_string()),
            Value::Str("?W".to_string()), // West
            Value::Str("?E".to_string()), // East
            Value::Str("?S".to_string()), // South (Output)
        ],
    );

    // State: [Goal, Dy=0, Dx=0]
    let state = Value::Junction(
        JunctionType::All,
        vec![
            goal,
            Value::Int(0),
            Value::Int(0),
        ],
    );

    // We need to inject the agent into the state manually or let scan_grid_rules do it.
    // But scan_grid_rules initializes with default state.
    // We need to overwrite the register at (5,5) so scan_grid_rules picks up our custom state.

    // Pack agent data: [State, Stack]
    let packed = Value::Junction(
        JunctionType::All,
        vec![
            state,
            Value::Junction(JunctionType::All, vec![]), // Empty stack
        ],
    );
    vm.prologue_state.registers.insert((5, 5), packed);

    // 4. Run Tick
    exec_prologue_tick(&mut vm);

    // 5. Assert Output
    // South of (5,5) is (6,5). Should be 1.
    let output = &vm.grid[6][5];
    println!("Output at (6,5): {:?}", output);

    if let Value::Int(val) = output {
        assert_eq!(*val, 1, "Logic Gate should output 1");
    } else {
        panic!("Logic Gate output not found or incorrect type: {:?}", output);
    }
}

#[test]
fn test_logic_agent_math_gate() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // No KB needed, using dynamic math_add

    // Setup Grid
    vm.grid[5][4] = Value::Int(5); // West
    vm.grid[5][6] = Value::Int(3); // East
    vm.grid[5][5] = Value::Str("∃".to_string());

    // Configure Agent
    // Goal: math_add(?W, ?E, ?S)
    let goal = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("math_add".to_string()),
            Value::Str("?W".to_string()),
            Value::Str("?E".to_string()),
            Value::Str("?S".to_string()),
        ],
    );

    let state = Value::Junction(
        JunctionType::All,
        vec![
            goal,
            Value::Int(0),
            Value::Int(0),
        ],
    );

    let packed = Value::Junction(
        JunctionType::All,
        vec![
            state,
            Value::Junction(JunctionType::All, vec![]),
        ],
    );
    vm.prologue_state.registers.insert((5, 5), packed);

    // Run Tick
    exec_prologue_tick(&mut vm);

    // Assert Output
    let output = &vm.grid[6][5];
    println!("Output at (6,5): {:?}", output);

    if let Value::Int(val) = output {
        assert_eq!(*val, 8, "Math Gate should output 8 (5+3)");
    } else {
        panic!("Math Gate output not found: {:?}", output);
    }
}
