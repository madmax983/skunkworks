use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_fission_reactor_chain() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // ✦ ☢ ⌘ ✇
    //
    // ✦ (Source) at (5, 5) -> Emits 50..100
    // ☢ (Reactor) at (5, 6) -> Input West (from Source), Output Half (to Self, N, E, S)
    // ⌘ (Moderator) at (5, 7) -> Input West (from Reactor), Output Half (to East)
    // ✇ (Control) at (5, 8) -> Input West (from Moderator), Absorbs

    vm.grid[5][5] = Value::Str("✦".to_string());
    vm.grid[5][6] = Value::Str("☢".to_string());
    vm.grid[5][7] = Value::Str("⌘".to_string());
    vm.grid[5][8] = Value::Str("✇".to_string());

    exec_prologue_tick(&mut vm);

    // Verify Source
    let source_sig = vm.prologue_state.signal_grid[5][5].clone();
    assert!(source_sig.is_some(), "Neutron Source should emit signal");
    let source_val = match source_sig.unwrap() {
        Value::Int(n) => n,
        _ => panic!("Source emit non-Int"),
    };
    assert!(source_val >= 50 && source_val < 100, "Source val {} out of range", source_val);

    // Verify Reactor (5, 6)
    // Reactor receives SourceVal.
    // Emits SourceVal / 2 to Self and N, E, S.
    let reactor_out = source_val / 2;
    let reactor_sig = vm.prologue_state.signal_grid[5][6].clone();
    assert_eq!(reactor_sig, Some(Value::Int(reactor_out)), "Reactor Self should be Half Input");

    // Verify Moderator (5, 7)
    // Moderator reads West (Reactor).
    // Reactor Output is `reactor_out`.
    // Moderator Self should light up with Input (`reactor_out`).
    let moderator_sig = vm.prologue_state.signal_grid[5][7].clone();
    assert_eq!(moderator_sig, Some(Value::Int(reactor_out)), "Moderator Self should be Input");

    // Verify Control Rod (5, 8)
    // Moderator Output (`reactor_out / 2`) pushed to East (5, 8).
    let moderator_out = reactor_out / 2;
    let control_sig = vm.prologue_state.signal_grid[5][8].clone();
    assert_eq!(control_sig, Some(Value::Int(moderator_out)), "Control Rod receive pushed signal");
}
