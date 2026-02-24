use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_green_spore_spawn() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1 -> ! -> * (Green)
    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("*".to_string());

    // Paint * Green
    vm.chroma_grid[5][6].fg = Some((0, 255, 0));

    exec_prologue_tick(&mut vm);

    // Assert Propagation East: (5, 7) should have signal 1
    assert_eq!(vm.prologue_state.signal_grid[5][7], Some(Value::Int(1)));

    // Assert Spawn South: (6, 6) should have an Agent
    // We expect this to FAIL currently as it is not implemented
    if let Value::Str(s) = &vm.grid[6][6] {
        assert_eq!(s, "@", "Expected Agent @ spawned at (6,6)");
    } else {
        panic!("Expected Agent @ at (6,6), found {:?}", vm.grid[6][6]);
    }
}
