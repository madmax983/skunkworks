use chimera_lang::ast::{Dna, Helix};
use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::Value;

#[test]
fn test_memetic_source() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid: "Meme" -> ι
    vm.grid[5][4] = Value::Str("Meme".to_string());
    vm.grid[5][5] = Value::Str("ι".to_string());

    exec_prologue_tick(&mut vm);

    // Should emit "Meme" to self (signal)
    assert_eq!(
        vm.prologue_state.signal_grid[5][5],
        Some(Value::Str("Meme".to_string()))
    );
}

#[test]
fn test_memetic_evolve() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid: "ABC" -> ι -> ε
    vm.grid[5][4] = Value::Str("ABC".to_string());
    vm.grid[5][5] = Value::Str("ι".to_string());
    vm.grid[5][6] = Value::Str("ε".to_string());

    exec_prologue_tick(&mut vm);

    // ε should receive "ABC" from ι (propagation) and emit mutated version to self
    let result = vm.prologue_state.signal_grid[5][6].clone();
    assert!(result.is_some());
    if let Some(Value::Str(s)) = result {
        assert_ne!(s, "ABC");
        println!("Mutated: {}", s);
    } else {
        panic!("Expected string signal, got {:?}", result);
    }
}

#[test]
fn test_memetic_censor() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid:
    // "BadWord" -> ι -> φ -> ? (Sink)
    // "Bad" -> ι -> (North of φ)

    // Row 4: "Bad" -> ι
    vm.grid[4][5] = Value::Str("Bad".to_string());
    vm.grid[4][6] = Value::Str("ι".to_string());

    // Row 5: "BadWord" -> ι -> φ
    vm.grid[5][4] = Value::Str("BadWord".to_string());
    vm.grid[5][5] = Value::Str("ι".to_string());
    vm.grid[5][6] = Value::Str("φ".to_string());

    exec_prologue_tick(&mut vm);

    // φ should BLOCK because "BadWord" contains "Bad"
    let result = vm.prologue_state.signal_grid[5][6].clone();
    assert!(
        result.is_none(),
        "Expected censorship to block signal, got {:?}",
        result
    );

    // Test Passing Case
    // Change Filter to "Good"
    vm.grid[4][5] = Value::Str("Good".to_string());
    exec_prologue_tick(&mut vm);

    let result_pass = vm.prologue_state.signal_grid[5][6].clone();
    assert_eq!(result_pass, Some(Value::Str("BadWord".to_string())));
}

#[test]
fn test_memetic_spread() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid: "Virus" -> ι -> σ
    vm.grid[5][4] = Value::Str("Virus".to_string());
    vm.grid[5][5] = Value::Str("ι".to_string());
    vm.grid[5][6] = Value::Str("σ".to_string());

    exec_prologue_tick(&mut vm);

    // σ should write "Virus" to grid neighbors (radius 1)
    // Neighbors of (5,6) are (4,5), (4,6), (4,7), (5,5), (5,7), (6,5), (6,6), (6,7)

    // Check (4,6) (North)
    assert_eq!(vm.grid[4][6], Value::Str("Virus".to_string()));
    // Check (6,6) (South)
    assert_eq!(vm.grid[6][6], Value::Str("Virus".to_string()));
    // Check (5,7) (East)
    assert_eq!(vm.grid[5][7], Value::Str("Virus".to_string()));
}

#[test]
fn test_memetic_imitate() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid:
    // "Short"
    //    κ    "Looooooooong"

    // (5,5) is κ
    vm.grid[5][5] = Value::Str("κ".to_string());
    vm.grid[4][5] = Value::Str("Short".to_string());
    vm.grid[5][6] = Value::Str("Looooooooong".to_string());

    exec_prologue_tick(&mut vm);

    // κ should become "Looooooooong" (longest neighbor)
    assert_eq!(vm.grid[5][5], Value::Str("Looooooooong".to_string()));
}
