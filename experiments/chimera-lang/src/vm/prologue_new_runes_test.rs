use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_clock_rune() {
    let mut vm = setup_vm();
    // C at 6,5.
    // Modulo 10 at East (6,6).
    vm.grid[6][5] = Value::Str("C".to_string());
    vm.grid[6][6] = Value::Int(10);

    vm.tick_counter = 42;

    exec_prologue_tick(&mut vm);

    // Check signal at 6,5 is 42 % 10 = 2
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
        assert_eq!(*v, 2);
    } else {
        panic!("Clock did not output signal");
    }
}

#[test]
fn test_directional_rune() {
    let mut vm = setup_vm();
    // 42 at 8,4.
    // ! at 9,4 (Reads 8,4 -> Emits 9,4).
    // ~ at 9,5 (Reads neighbors... includes 9,4). So 9,5 gets 42.
    // N at 8,5 (Reads South 9,5 -> Emits North 7,5).

    vm.grid[8][4] = Value::Int(42);
    vm.grid[9][4] = Value::Str("!".to_string());
    vm.grid[9][5] = Value::Str("~".to_string());
    vm.grid[8][5] = Value::Str("N".to_string());

    exec_prologue_tick(&mut vm);

    // Check 9,4 (Source) has signal
    assert!(vm.prologue_state.signal_grid[9][4].is_some());
    // Check 9,5 (Wire) has signal
    assert!(vm.prologue_state.signal_grid[9][5].is_some());
    // Check 8,5 (N) has signal (Self)
    assert!(vm.prologue_state.signal_grid[8][5].is_some());
    // Check 7,5 (North of N) has signal
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[7][5] {
        assert_eq!(*v, 42);
    } else {
        panic!("N did not propagate signal to North");
    }
}

#[test]
fn test_warp_rune() {
    let mut vm = setup_vm();
    // ( at 6,5.
    // West 6,4 has signal (Source ! at 6,4 reading 5,4).
    // North 5,5 is 10.
    // South 7,5 is 20.

    vm.grid[5][4] = Value::Int(1);
    vm.grid[6][4] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Int(10);
    vm.grid[7][5] = Value::Int(20);
    vm.grid[6][5] = Value::Str("(".to_string());

    exec_prologue_tick(&mut vm);

    // Check swap
    assert_eq!(vm.grid[5][5], Value::Int(20));
    assert_eq!(vm.grid[7][5], Value::Int(10));
}

#[test]
fn test_crossover_rune() {
    let mut vm = setup_vm();
    // DNA setup: Strand 0 [Dummy], Strand 1 [1, 1], Strand 2 [2, 2]
    vm.dna.helix.strands.push(Strand { genes: vec![] }); // 0
    vm.dna.helix.strands.push(Strand { genes: vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
    ] }); // 1
    vm.dna.helix.strands.push(Strand { genes: vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
    ] }); // 2

    // X at 6,5.
    // West 6,4 Source 1 (Strand 1).
    // East 6,6 Source 2 (Strand 2).

    vm.grid[5][4] = Value::Int(1);
    vm.grid[6][4] = Value::Str("!".to_string()); // Signal 1

    vm.grid[5][6] = Value::Int(2);
    vm.grid[6][6] = Value::Str("!".to_string()); // Signal 2

    vm.grid[6][5] = Value::Str("X".to_string());

    exec_prologue_tick(&mut vm);

    // Check new strand
    // Should have 4 strands: 0, 1, 2, 3(new)
    assert_eq!(vm.dna.helix.strands.len(), 4);
    let new_strand = &vm.dna.helix.strands[3];
    assert_eq!(new_strand.genes.len(), 2);
    // Should be [1, 2] (half of A, half of B)
    if let Nucleotide::Number(n) = &new_strand.genes[0].args[0] {
        assert_eq!(*n, 1);
    }
    if let Nucleotide::Number(n) = &new_strand.genes[1].args[0] {
        assert_eq!(*n, 2);
    }

    // Check output signal at X (Self)
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
        assert_eq!(*v, 3);
    } else {
        panic!("X did not output new strand index");
    }
}
