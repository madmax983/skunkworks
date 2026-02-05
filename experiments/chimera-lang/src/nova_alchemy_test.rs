use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[test]
fn test_alchemy_arithmetic() {
    // Strand 0: Spawn Ribosome, then jump to Strand 1.
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };
    // Strand 1: Idle loop
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        }],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    // Setup Grid
    vm.grid[8][8] = Value::Int(10);
    vm.grid[8][9] = Value::Int(20);
    vm.grid[8][10] = Value::Str("+".to_string());
    vm.grid[8][11] = Value::Int(5);
    vm.grid[8][12] = Value::Str("mul".to_string());

    // Step 1: Push 0
    vm.step();
    // Step 2: Push 4
    vm.step();
    // Step 3: Spawn. Ribosome runs (8,8)->10.
    vm.step();

    // Ribosome is at (8,9).
    vm.step(); // Tick 4: Reads 20.
    vm.step(); // Tick 5: Reads +.
    vm.step(); // Tick 6: Reads 5.
    vm.step(); // Tick 7: Reads *.

    assert!(!vm.organelles.is_empty());
    let ribosome = &vm.organelles[0];
    assert_eq!(ribosome.stack.len(), 1);
    if let Value::Int(n) = ribosome.stack[0] {
        assert_eq!(n, 150);
    } else {
        panic!("Expected Int(150)");
    }
}

#[test]
fn test_alchemy_logic() {
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        }],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    vm.grid[8][8] = Value::Int(10);
    vm.grid[8][9] = Value::Int(10);
    vm.grid[8][10] = Value::Str("=".to_string());
    vm.grid[8][11] = Value::Str("!".to_string());

    vm.step();
    vm.step();
    vm.step(); // Spawn (Tick 3). Reads 10.

    vm.step(); // Tick 4: Reads 10.
    vm.step(); // Tick 5: Reads "=". Stack [1].

    {
        let ribosome = &vm.organelles[0];
        assert_eq!(ribosome.stack.last().unwrap(), &Value::Int(1));
    }

    vm.step(); // Tick 6: Reads "!". Stack [0].

    {
        let ribosome = &vm.organelles[0];
        assert_eq!(ribosome.stack.last().unwrap(), &Value::Int(0));
    }
}

#[test]
fn test_alchemy_io() {
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(4)],
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        }],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));

    // Grid Setup
    vm.grid[8][8] = Value::Int(99);
    vm.grid[8][9] = Value::Int(1);
    vm.grid[8][10] = Value::Int(0);
    vm.grid[8][11] = Value::Str(";".to_string());

    vm.grid[8][12] = Value::Int(1);
    vm.grid[8][13] = Value::Int(-3);
    vm.grid[8][14] = Value::Str(":".to_string());

    vm.step();
    vm.step();
    vm.step(); // Spawn (Tick 3). Reads 99.

    vm.step(); // Tick 4. Reads 1.
    vm.step(); // Tick 5. Reads 0.
    vm.step(); // Tick 6. Reads ";". Writes.

    // Check grid write
    assert_eq!(vm.grid[9][11], Value::Int(99));

    vm.step(); // Tick 7. Reads 1.
    vm.step(); // Tick 8. Reads -3.
    vm.step(); // Tick 9. Reads ":". Stack [99].

    let ribosome = &vm.organelles[0];
    assert_eq!(ribosome.stack.last().unwrap(), &Value::Int(99));
}
