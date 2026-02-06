use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::vm::nova::perform_alchemy;

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_alchemy_transmutation() {
    let mut vm = ChimeraVM::new(make_dna(vec![]));

    // Setup Grid
    // (8,8) center
    // (8,9) fire (Right)
    // (9,8) water (Down)
    vm.grid[8][9] = Value::Str("fire".to_string());
    vm.grid[9][8] = Value::Str("water".to_string());

    // Perform Alchemy at (8,8)
    let success = perform_alchemy(&mut vm, 8, 8);

    assert!(success);

    // Center should be "steam"
    if let Value::Str(s) = &vm.grid[8][8] {
        assert_eq!(s, "steam");
    } else {
        panic!("Expected steam");
    }

    // Neighbors should be consumed (0)
    assert_eq!(vm.grid[8][9], Value::Int(0));
    assert_eq!(vm.grid[9][8], Value::Int(0));
}

#[test]
#[cfg(feature = "nova")]
fn test_alchemist_organelle() {
    // [ spawn(6, 0) ] -> Spawn Alchemist (6) to execute strand 0
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // strand 0
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)], // type Alchemist
            },
            Gene {
                op: OpCode::Spawn,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump, // Loop
                args: vec![Nucleotide::Number(0)],
            },
        ],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0]));
    // Set start location
    vm.context_loc = (8, 8);

    // Setup ingredients around (8,8)
    vm.grid[8][7] = Value::Str("earth".to_string()); // Left
    vm.grid[7][8] = Value::Str("fire".to_string()); // Up

    // Step 1: Push 0
    vm.step();
    // Step 2: Push 6
    vm.step();
    // Step 3: Spawn Alchemist at (8,8)
    vm.step();

    assert_eq!(vm.organelles.len(), 1);

    // Alchemist is at (8,8).
    // Tick 4: Alchemist ticks. Should perform alchemy immediately.
    vm.step();

    // Check Grid
    if let Value::Str(s) = &vm.grid[8][8] {
        assert_eq!(s, "lava");
    } else {
        panic!("Expected lava, got {:?}", vm.grid[8][8]);
    }
}
