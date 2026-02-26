use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_automaton_agent() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Automaton at (5, 5)
    // Program: "+W" (Inc Memory -> 1, Write Forward)
    vm.grid[5][5] = Value::Str("🤖".to_string());
    vm.prologue_state
        .registers
        .insert((5, 5), Value::Str("A:0:1:0:+W".to_string()));

    exec_prologue_tick(&mut vm);
    exec_prologue_tick(&mut vm);

    // Verify Grid
    if let Value::Int(v) = &vm.grid[5][6] {
        assert_eq!(*v, 1, "Expected 1 at (5, 6)");
    } else {
        panic!("Expected Int(1) at (5, 6), found {:?}", vm.grid[5][6]);
    }
}

#[test]
fn test_automaton_loop() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Automaton at (10, 10)
    // Program: "+++[-]" (Set 3, Decrement until 0)
    vm.grid[10][10] = Value::Str("🤖".to_string());
    vm.prologue_state
        .registers
        .insert((10, 10), Value::Str("A:0:1:0:+++[-]".to_string()));

    // Run EXACTLY 12 ticks
    for i in 0..12 {
        exec_prologue_tick(&mut vm);

        let grid_val = &vm.grid[10][10];
        let reg_val = vm.prologue_state.registers.get(&(10, 10));

        println!("Tick {}: Grid={:?} Reg={:?}", i + 1, grid_val, reg_val);

        if !matches!(grid_val, Value::Str(s) if s == "🤖") {
            panic!("Agent disappeared from grid at tick {}!", i + 1);
        }
        if reg_val.is_none() {
            panic!("Registers lost at tick {}!", i + 1);
        }
    }

    // Verify Memory is 0
    if let Some(state_val) = vm.prologue_state.registers.get(&(10, 10)) {
        if let Value::Str(s) = state_val {
            println!("Loop State Final: {}", s);
            let parts: Vec<&str> = s.split(':').collect();
            let mem = parts[3].parse::<i64>().unwrap();
            assert_eq!(mem, 0, "Memory should be 0 after loop, got {}", mem);
        }
    }
}
