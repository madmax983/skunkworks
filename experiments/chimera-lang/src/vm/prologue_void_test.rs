use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_vacuum_rune() {
    let mut vm = make_vm();

    // µ (Vacuum) at (5,5)
    // West (5,4) is Empty (0)
    // Expected: µ outputs 1

    vm.grid[5][5] = Value::Str("µ".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(1)));

    // Now put a signal at West
    // Use a source ! at (5,4) and value 1 at (4,4)
    // ! at (5,4) reads (4,4).
    vm.grid[4][4] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm);

    // ! emits to (5,4). µ sees signal at West. Should output None.
    assert!(vm.prologue_state.signal_grid[5][5].is_none());
}

#[test]
fn test_void_anchor() {
    let mut vm = make_vm();

    // Ø (Void Anchor) at (5,5)
    // All neighbors empty.
    vm.grid[5][5] = Value::Str("Ø".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(1)));

    // Add neighbor signal at West (5,4) using !
    vm.grid[4][4] = Value::Int(1);
    vm.grid[5][4] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm);

    // ! emits to (5,4). Ø sees signal. Should be None.
    assert!(vm.prologue_state.signal_grid[5][5].is_none());
}

#[test]
fn test_singularity_attraction() {
    let mut vm = make_vm();

    // § at (5,5)
    vm.grid[5][5] = Value::Str("§".to_string());

    // Agent @ at (5,8) (Distance 3)
    vm.grid[5][8] = Value::Str("@".to_string());

    exec_prologue_tick(&mut vm);

    // Agent should move towards (5,5). So (5,7).
    assert_eq!(vm.grid[5][8], Value::Int(0));
    assert_eq!(vm.grid[5][7], Value::Str("@".to_string()));

    // Check internal state
    let agent = &vm.prologue_state.agents[0];
    assert_eq!(agent.x, 7);
    assert_eq!(agent.y, 5);
}

#[test]
fn test_singularity_consumption() {
    let mut vm = make_vm();

    // § at (5,5)
    vm.grid[5][5] = Value::Str("§".to_string());

    // Signal chain: 99 -> ! -> ~ -> §
    // ! at (5,3) reads (4,3)
    vm.grid[4][3] = Value::Int(99);
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("~".to_string());

    exec_prologue_tick(&mut vm);

    // ! emits to (5,3).
    // ~ at (5,4) reads (5,3), lights up (5,4), propagates to (5,5).
    // § at (5,5) consumes neighbor signals.
    // It should clear (5,4) because (5,4) is a neighbor.

    let output = vm.output.join("\n");

    // Verify consumption
    assert!(vm.prologue_state.signal_grid[5][4].is_none());
    assert!(output.contains("PROLOGUE: Singularity consumed signal"));
}
