use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::exec_prologue_tick;

#[test]
fn test_mycelium_network() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Injection Site
    // 42 -> ! -> 📥 -> 🍄
    // Grid:
    // (5,2): 42
    // (5,3): ! (Source emits 42 to East)
    // (5,4): 📥 (Injects 42)
    // (5,5): 🍄

    vm.grid[5][2] = Value::Int(42);
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("📥".to_string());
    vm.grid[5][5] = Value::Str("🍄".to_string());

    // 2. Setup Extraction Site (Disjoint)
    // 🍄 -> 📤 -> ? (South)
    // Grid:
    // (10,10): 🍄
    // (10,11): 📤

    vm.grid[10][10] = Value::Str("🍄".to_string());
    vm.grid[10][11] = Value::Str("📤".to_string());

    // 3. Tick 1: Scan & Signal
    // Scan registers 🍄 in network.
    // 42 signals to 📥.
    // 📥 pushes to buffer.

    exec_prologue_tick(&mut vm);

    // Verify Network Registration
    assert!(vm.prologue_state.mycelium_network.contains(&(5, 5)));
    assert!(vm.prologue_state.mycelium_network.contains(&(10, 10)));

    // Verify Buffer Injection
    // Sink logic runs after Propagation. So after Tick 1, buffer should have 42.
    assert_eq!(vm.prologue_state.mycelium_buffer.len(), 1);
    assert_eq!(vm.prologue_state.mycelium_buffer[0], Value::Int(42));

    // Disable Injection Site to prevent duplicate injection
    vm.grid[5][3] = Value::Int(0);

    // 4. Tick 2: Extraction
    // - Propagation: 📤 reads buffer.
    // - Sink: 📤 triggers and pops buffer to South.

    // We need to trigger 📤 with a signal.
    // Set up a signal source for 📤
    // (10, 9): 1
    // (10, 10): ! (Source emits 1 to East)
    // (10, 11): 📤 (Extract)

    vm.grid[10][9] = Value::Int(1);
    vm.grid[10][10] = Value::Str("!".to_string());
    vm.grid[10][11] = Value::Str("📤".to_string());

    // Tick 2
    exec_prologue_tick(&mut vm);

    // Check Result
    // Buffer should be empty
    assert_eq!(vm.prologue_state.mycelium_buffer.len(), 0);

    // South of 📤 is (11, 11).
    let extracted = &vm.grid[11][11];
    assert_eq!(*extracted, Value::Int(42));
}
