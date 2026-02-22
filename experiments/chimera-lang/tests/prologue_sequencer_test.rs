use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_sequencer_read() {
    let genes = vec![Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(42)],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.prologue_state.active = true;

    // Layout:
    //     ! (4,4)
    // ! 🔍 ?
    // (5,3) (5,4) (5,5)

    // Inputs for ! at (4,4) [North Input for 🔍]
    // ! reads West (4,3). We use "0" string to avoid empty value filter.
    vm.grid[4][3] = Value::Str("0".to_string());
    vm.grid[4][4] = Value::Str("!".to_string());

    // Inputs for ! at (5,3) [West Input for 🔍]
    // ! reads West (5,2).
    vm.grid[5][2] = Value::Str("0".to_string());
    vm.grid[5][3] = Value::Str("!".to_string());

    vm.grid[5][4] = Value::Str("🔍".to_string());
    vm.grid[5][5] = Value::Str("?".to_string());

    for _ in 0..5 {
        exec_prologue_tick(&mut vm);
    }

    let output = vm.output.join("\n");
    assert!(output.contains("push(42)"), "Output should contain gene string: {}", output);
}

#[test]
fn test_sequencer_write() {
    let genes = vec![Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(42)],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.prologue_state.active = true;

    // Write "drop" to index 0 of strand 0.
    // West="drop", North=0 (Idx), East=0 (Strand)

    // North Input (Idx 0): ! at (4,4) reading (4,3)="0"
    vm.grid[4][3] = Value::Str("0".to_string());
    vm.grid[4][4] = Value::Str("!".to_string());

    // West Input ("drop"): ! at (5,3) reading (5,2)="drop"
    vm.grid[5][2] = Value::Str("drop".to_string());
    vm.grid[5][3] = Value::Str("!".to_string());

    // East Input (Strand 0): Needs signal at (5,5).
    // Wires: ! (4,6) -> ~ (5,6) -> ~ (5,5) -> ✏ (5,4)
    // ! at (4,6) reads West (4,5)="0".
    vm.grid[4][5] = Value::Str("0".to_string());
    vm.grid[4][6] = Value::Str("!".to_string());

    vm.grid[5][4] = Value::Str("✏".to_string());
    vm.grid[5][5] = Value::Str("~".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());

    for _ in 0..10 {
        exec_prologue_tick(&mut vm);
    }

    // Verify modification
    let op = &vm.dna.helix.strands[0].genes[0].op;
    assert_eq!(*op, OpCode::Drop);
}

#[test]
fn test_orca_clock() {
    let genes = vec![];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.prologue_state.active = true;
    vm.prologue_state.orca_mode = true;

    // Layout:
    //   ! (4,4) [Mod]
    // ! C ?
    // (5,3) [Rate]

    // North Input (Mod 8): ! at (4,4) reads (4,3)=8
    vm.grid[4][3] = Value::Int(8);
    vm.grid[4][4] = Value::Str("!".to_string());

    // West Input (Rate 1): ! at (5,3) reads (5,2)=1
    vm.grid[5][2] = Value::Int(1);
    vm.grid[5][3] = Value::Str("!".to_string());

    vm.grid[5][4] = Value::Str("C".to_string());
    vm.grid[5][5] = Value::Str("?".to_string());

    // Need enough ticks
    for _ in 0..10 {
        exec_prologue_tick(&mut vm);
        vm.tick_counter += 1; // exec_prologue_tick doesn't increment tick!
    }

    let output = vm.output.join("\n");
    assert!(output.contains("received Int(1)"));
    assert!(output.contains("received Int(2)"));
}
