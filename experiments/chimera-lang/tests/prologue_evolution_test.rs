use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_chaos_rune() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Place K at 5,5
    vm.grid[5][5] = Value::Str("K".to_string());

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    // Check neighbors for random values
    let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
    let mut found_signal = false;
    for (ny, nx) in neighbors {
        if let Some(Value::Int(_)) = &vm.prologue_state.signal_grid[ny][nx] {
            found_signal = true;
        }
    }
    assert!(found_signal, "Chaos rune should emit signals");
}

#[test]
fn test_register_rune() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Place R at 5,5
    vm.grid[5][5] = Value::Str("R".to_string());

    // 1. Write Phase
    // Inject signal West of R (5,4)
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(42));

    // Execute tick (Propagation will run R write logic)
    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    // Verify register has 42
    assert_eq!(
        vm.prologue_state.registers.get(&(5, 5)),
        Some(&Value::Int(42))
    );

    // 2. Read Phase
    // Clear signals manually for test
    for row in vm.prologue_state.signal_grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = None;
        }
    }

    // Inject signal North of R (4,5)
    vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(1));

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    // Check signal South (6,5)
    // Note: R propagation writes to next_signals.
    // Since we ran exec_prologue_tick, signal_grid holds the final state of propagation.
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(42)));
}

#[test]
fn test_crossover_rune() {
    // Strand 0: Push 1
    // Strand 1: Push 2
    let s0 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }],
    };
    let s1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }],
    };

    let dna = Dna {
        helix: Helix {
            strands: vec![s0, s1],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // X at 5,5
    // West (5,4): Signal 0
    // East (5,6): Signal 1

    vm.grid[5][5] = Value::Str("X".to_string());
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(0));
    vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(1));

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    // Should produce Strand 2
    assert_eq!(vm.dna.helix.strands.len(), 3);

    // Should emit index 2 to South (6,5) on GRID (not signal grid, X writes to grid)
    assert_eq!(vm.grid[6][5], Value::Int(2));
}
