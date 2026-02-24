use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn create_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_library_write_read() {
    let mut vm = create_vm();

    // Setup Circuit:
    // Key: Value at (5,3), ! at (5,4) -> Emits to (5,4). Book reads West (5,4).
    vm.grid[5][3] = Value::Str("Scroll1".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    // Mode: Value at (4,4), ! at (4,5) -> Emits to (4,5). Book reads North (4,5).
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string());

    // Value: Value at (6,4), ! at (6,5) -> Emits to (6,5). Book reads South (6,5).
    vm.grid[6][4] = Value::Str("Ancient Knowledge".to_string());
    vm.grid[6][5] = Value::Str("!".to_string());

    // Book:
    vm.grid[5][5] = Value::Str("📖".to_string());

    exec_prologue_tick(&mut vm);

    // Verify Library Write
    // Note: We need to check if signals propagated correctly first.
    // If ! at (4,5) failed to read (4,4), mode would be default (Read).
    // Let's check signal grid after tick? No, cleared.
    // But library state persists.

    if let Some(val) = vm.prologue_state.library.get("Scroll1") {
        assert_eq!(val, &Value::Str("Ancient Knowledge".to_string()));
    } else {
        panic!("Library entry 'Scroll1' not found. Mode signal likely failed.");
    }

    // Now Test Read
    // Reset Grid
    vm.grid = vec![vec![Value::Int(0); crate::vm::GRID_SIZE]; crate::vm::GRID_SIZE];

    // Key: Value at (5,3), ! at (5,4).
    vm.grid[5][3] = Value::Str("Scroll1".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    // Book:
    vm.grid[5][5] = Value::Str("📖".to_string());
    // Default Mode is Read (0).

    exec_prologue_tick(&mut vm);

    // Verify Output Signal at (5,5)
    // Sinks emit to self in signal_grid during step 5.
    // Agents run step 6.
    // At end of tick, signal_grid holds sink outputs.

    assert_eq!(
        vm.prologue_state.signal_grid[5][5],
        Some(Value::Str("Ancient Knowledge".to_string()))
    );
}

#[test]
fn test_dynamic_compilation() {
    let mut vm = create_vm();

    let code = "strand main { push(999) print }";

    // Setup Circuit:
    // West: Char "X"
    vm.grid[5][3] = Value::Str("X".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    // North: Code
    // ! at (4,5) reads West (4,4).
    vm.grid[4][4] = Value::Str(code.to_string());
    vm.grid[4][5] = Value::Str("!".to_string());

    // Rune:
    vm.grid[5][5] = Value::Str("£".to_string());

    exec_prologue_tick(&mut vm);

    // Verify Strand added
    assert_eq!(vm.dna.helix.strands.len(), 1);

    // Verify Custom Rune registered
    assert_eq!(vm.prologue_state.custom_runes.get("X"), Some(&0));

    // Now execute it using "X"
    vm.grid = vec![vec![Value::Int(0); crate::vm::GRID_SIZE]; crate::vm::GRID_SIZE];
    vm.grid[5][5] = Value::Str("X".to_string());

    // Run tick
    exec_prologue_tick(&mut vm);

    // `vm.interrupt` should have been called.

    assert!(!vm.halted);
    // VM loop would run strand 0.
    // Manually step the VM to execute the compiled genes.

    // Gene 0: Push 999
    vm.step();
    // Gene 1: Print
    vm.step();

    // Wait, strand execution depends on IP.
    // interrupt sets IP to (strand_idx, 0).
    // `exec_prologue_tick` sets context_loc but doesn't run VM.
    // `vm.step()` executes ONE gene at IP.

    // If compilation produced: Push(999), Print.
    // That's 2 genes.

    // Output should contain "999"
    // Print enzyme pushes to `vm.output`.

    // Debug:
    // println!("{:?}", vm.output);

    assert!(vm.output.iter().any(|s| s.contains("999")));
}

#[test]
fn test_scholar_agent() {
    let mut vm = create_vm();

    // Place Book
    vm.grid[5][6] = Value::Str("📖".to_string());
    vm.prologue_state
        .library
        .insert("Lore".to_string(), Value::Int(100));

    // Place Scholar
    vm.grid[5][5] = Value::Str("🎓".to_string());
    // Initial state: 0 XP, Mode 0 (Wander)

    exec_prologue_tick(&mut vm);

    // Scholar is at (5,5), Book at (5,6). Dist = 1.
    // Logic: If distance <= 1, Switch to Read Mode (1).
    // Agent doesn't move this tick.

    // Check Agent State
    let agent = &vm.prologue_state.agents[0];
    // State should be [0, 1, 5, 6] (XP, Mode=Read, Ty, Tx)
    // Mode is index 1.
    if let Value::Junction(_, list) = &agent.state {
        // Checking Mode
        if let Value::Int(mode) = list[1] {
            assert_eq!(mode, 1);
        } else {
            panic!("Mode not int");
        }
    } else {
        panic!("Invalid state format");
    }

    // Next Tick: Read Mode
    exec_prologue_tick(&mut vm);

    // Check delayed signals for request
    assert!(vm.prologue_state.delayed_signals[5][5].is_some());
    if let Some(Value::Str(k)) = &vm.prologue_state.delayed_signals[5][5] {
        assert_eq!(k, "Lore");
    } else {
        panic!("Expected Key 'Lore' in delayed signals");
    }
}
