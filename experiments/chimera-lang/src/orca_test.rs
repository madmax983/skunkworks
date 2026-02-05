use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[test]
fn test_orca_arithmetic() {
    // Strand 0: Spawn Ribosome then Jump to Strand 1
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // Strand Index
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)], // Type = Ribosome
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)], // Jump to idle strand
            },
        ],
    };

    // Strand 1: Idle
    let strand1 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    // Setup Grid Program at (8,8)
    // 5, 3, +, 2, *
    vm.grid[8][8] = Value::Int(5);
    vm.grid[8][9] = Value::Int(3);
    vm.grid[8][10] = Value::Str("+".to_string());
    vm.grid[8][11] = Value::Int(2);
    vm.grid[8][12] = Value::Str("*".to_string());

    // Execution:
    vm.step(); // push 0
    vm.step(); // push 4
    vm.step(); // Spawn

    // Organelle created at 8,8 with default dir (0,1) East.
    // Main VM -> Jump(1).

    // Organelle Step 1: Reads 5. Stack [5]. Pos -> 8,9.
    vm.step();

    // Organelle Step 2: Reads 3. Stack [5, 3]. Pos -> 8,10.
    vm.step();

    // Organelle Step 3: Reads "+". Stack [8]. Pos -> 8,11.
    vm.step();

    // Organelle Step 4: Reads 2. Stack [8, 2]. Pos -> 8,12.
    vm.step();

    // Organelle Step 5: Reads "*". Stack [16]. Pos -> 8,13.
    // This step executes implicitly because Spawn also consumed a tick.

    assert!(!vm.organelles.is_empty());
    assert_eq!(vm.organelles[0].stack.len(), 1);
    assert_eq!(vm.organelles[0].stack[0], Value::Int(16));
}

#[test]
fn test_relative_io() {
    // Test g_read_r and g_write_r
    let strand0 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(4)] },
            Gene { op: OpCode::Spawn, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
        ],
    };

    let strand1 = Strand {
        genes: vec![
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] },
        ],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    // Grid Setup
    // (8,8): 1 (dy)
    // (8,9): 0 (dx)
    // (8,10): "g_read_r"
    // (9,10): 42 (Target)

    vm.grid[8][8] = Value::Int(1);
    vm.grid[8][9] = Value::Int(0);
    vm.grid[8][10] = Value::Str("g_read_r".to_string());
    vm.grid[9][10] = Value::Int(42);

    // Run
    vm.step(); vm.step(); vm.step(); // Spawn

    vm.step(); // Read 1. Stack [1]. Pos 8,9.
    vm.step(); // Read 0. Stack [1, 0]. Pos 8,10.
    // Step 3 (g_read_r) executes here. Stack [42]. Pos 8,11.

    assert_eq!(vm.organelles[0].stack.last().unwrap(), &Value::Int(42));

    // Test g_write_r
    // (8,11): 99 (Value)
    // (8,12): -1 (dy = North)
    // (8,13): 0 (dx)
    // (8,14): "g_write_r"
    // Write 99 to (8-1, 14+0) = (7,14).

    vm.grid[8][11] = Value::Int(99);
    vm.grid[8][12] = Value::Int(-1);
    vm.grid[8][13] = Value::Int(0);
    vm.grid[8][14] = Value::Str("g_write_r".to_string());

    vm.step(); // Read 99. Stack [42, 99]. Pos 8,12.
    vm.step(); // Read -1. Stack [42, 99, -1]. Pos 8,13.
    vm.step(); // Read 0. Stack [42, 99, -1, 0]. Pos 8,14.
    vm.step(); // Exec g_write_r. Pop 0(dx), -1(dy), 99(val). Write 99 to 7,14.

    assert_eq!(vm.grid[7][14], Value::Int(99));
}
