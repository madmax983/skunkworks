use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_plasmid_absorption() {
    let mut vm = setup_vm();

    // Layout:
    // 5,4: 42 (Value)
    // 5,5: ! (Source) -> Emits 42 to 5,5
    // 5,6: P (Plasmid) -> Should absorb 42 from West (5,5)

    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("P".to_string());

    // Register P agent state
    vm.prologue_state.registers.insert((5, 6), Value::Int(0)); // Empty payload

    // Tick 1
    exec_prologue_tick(&mut vm);

    // ! should have emitted signal 42 to 5,5
    // P at 5,6 should check 5,5 signal and absorb it.
    // However, P also moves randomly.
    // We check registers.

    // We might need to find where P moved.
    // But P moves AFTER logic.
    // So logic runs at (5, 6).
    // It sees signal at (5, 5).
    // It updates register at (5, 6).
    // THEN it moves to (ny, nx).
    // It updates register at (ny, nx) with NEW state.

    // So we just need to find P in the grid and check its register.
    let mut found = false;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "P" {
                    if let Some(val) = vm.prologue_state.registers.get(&(y, x)) {
                        // Check if payload is 42
                        // Payload is stored as state directly.
                        if let Value::Int(v) = val {
                            if *v == 42 {
                                found = true;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(found, "Plasmid should have absorbed 42");
}

#[test]
fn test_plasmid_conjugation() {
    let mut vm = setup_vm();

    // Layout:
    // 5,5: P (Plasmid) with Payload 100
    // 5,6: @ (Seeker) with State 0

    vm.grid[5][5] = Value::Str("P".to_string());
    vm.prologue_state.registers.insert((5, 5), Value::Int(100));

    vm.grid[5][6] = Value::Str("@".to_string());
    vm.prologue_state.registers.insert((5, 6), Value::Int(0));

    // Tick 1
    exec_prologue_tick(&mut vm);

    // P should inject 100 into @ at 5,6.
    // @ moves towards signal (none here) so it stays or wanders?
    // Seeker logic:
    // If no signal, returns None (stay).

    // So @ should be at 5,6 (or moved if implementation changed to random walk on idle).
    // Let's find @
    let mut found_seeker = false;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "@" {
                    if let Some(val) = vm.prologue_state.registers.get(&(y, x)) {
                        if let Value::Int(v) = val {
                            if *v == 100 {
                                found_seeker = true;
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(found_seeker, "Seeker should have received payload 100");
}

#[test]
fn test_reverse_transcriptase() {
    let mut vm = setup_vm();

    // Layout:
    // 5,4: "push" (String Value)
    // 5,5: Ð (Eth - Reverse Transcriptase)

    // DNA: Strand 0 (Main) is empty. IP at 0,0.

    // We need to inject signal directly because source emission happens in same tick
    // but we can setup grid state.
    // Actually, simple setup:
    vm.grid[5][4] = Value::Str("push".to_string());
    vm.grid[5][5] = Value::Str("Ð".to_string());

    // We need 5,4 to be a signal for Ð to read.
    // If 5,4 is just a Value in Grid, Ð reads SIGNAL grid.
    // We need a source `!` to emit it, or manually set signal grid.
    // Since `prepare_signals` clears signal grid at start of tick,
    // we must rely on a source `!` to emit it.

    // Layout:
    // 5,3: "push"
    // 5,4: !
    // 5,5: Ð

    vm.grid[5][3] = Value::Str("push".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("Ð".to_string());

    // Add an empty strand 0
    vm.dna
        .helix
        .strands
        .push(chimera_lang::ast::Strand { genes: vec![] });
    vm.ip = (0, 0);

    // Tick 1
    exec_prologue_tick(&mut vm);

    // Check DNA
    assert_eq!(vm.dna.helix.strands[0].genes.len(), 1, "Should have 1 gene");
    assert_eq!(
        vm.dna.helix.strands[0].genes[0].op,
        chimera_lang::opcode::OpCode::Push
    );
}
