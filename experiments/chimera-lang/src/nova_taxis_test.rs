use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_chemotaxis_gradient() {
    // Setup:
    // [ push(0) chemotaxis() ]
    // Grid: Center (8,8).
    // Hormone Grid: (8,9) = 100 (East), (8,7) = 0.
    // Expected: dy=0, dx=1.

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Chemotaxis,
            args: vec![],
        },
    ];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Set context to 8,8
    vm.context_loc = (8, 8);

    // Set hormone gradient
    // Channel 0
    // (8,8) is center.
    // (8,9) is East.
    vm.hormone_grid[8][9][0] = 100;
    vm.hormone_grid[8][8][0] = 50;
    vm.hormone_grid[8][7][0] = 10;

    // Step
    vm.step(); // push(0)
    vm.step(); // chemotaxis

    // Stack should have dy, dx
    assert_eq!(vm.stack.len(), 2);
    let dx = vm.stack.pop().unwrap();
    let dy = vm.stack.pop().unwrap();

    assert_eq!(dx, Value::Int(1)); // East
    assert_eq!(dy, Value::Int(0));
}

#[test]
fn test_chemotaxis_none() {
    // Flat gradient
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Chemotaxis,
            args: vec![],
        },
    ];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.context_loc = (8, 8);

    vm.step(); // push
    vm.step(); // chemotaxis

    assert_eq!(vm.stack.len(), 2);
    let dx = vm.stack.pop().unwrap();
    let dy = vm.stack.pop().unwrap();

    // All zero, so max is 0.
    // The implementation scans loops -1..1. The first one with max intensity is picked if ties.
    // Order: dy=-1 (dx=-1,0,1), dy=0 (dx=-1,0,1), dy=1...
    // With all zeros, max is -1 initially. First update sets max to 0.
    // It will pick the first neighbor it checks.
    // Implementation:
    // for dy in -1..=1 { for dx in -1..=1 { ... } }
    // First iteration: dy=-1, dx=-1. Intensity 0 > -1. Picks (-1, -1).
    // So expected is -1, -1.

    assert_eq!(dx, Value::Int(-1));
    assert_eq!(dy, Value::Int(-1));
}
