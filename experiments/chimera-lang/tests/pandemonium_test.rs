use chimera_lang::ast::JunctionType;
use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_pandemonium_gamble() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 10 -> ! -> ¿ -> ? (Check Grid South of ¿)
    // 10 at 5,4
    // ! at 5,5 (Emits 10 to 5,5)
    // ¿ at 5,6 (Reads 5,5. Output to 6,6)

    vm.grid[5][4] = Value::Int(10);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("¿".to_string());

    exec_prologue_tick(&mut vm);

    // ¿ output should be at South (6,6) in SIGNAL GRID.
    // Wait, apply_pandemonium_runes writes to `next_signals`.
    // exec_prologue_tick swaps buffers.
    // So after tick, the signal should be in vm.prologue_state.signal_grid[6][6].

    if let Some(val) = &vm.prologue_state.signal_grid[6][6] {
        match val {
            Value::Int(n) => assert!(*n == 20 || *n == 0, "Gamble produced {}", n),
            _ => panic!("Gamble produced non-int"),
        }
    } else {
        panic!("Gamble produced no signal at South");
    }
}

#[test]
fn test_pandemonium_flux() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 10 -> ! -> ≈
    // 10 at 5,4
    // ! at 5,5
    // ≈ at 5,6 (Reads 5,5)

    vm.grid[5][4] = Value::Int(10);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("≈".to_string());

    exec_prologue_tick(&mut vm);

    // Check neighbors of ≈ (5,6)
    // N(4,6), S(6,6), W(5,5), E(5,7)
    let neighbors = [(4, 6), (6, 6), (5, 5), (5, 7)];
    let mut found = false;
    for (y, x) in neighbors {
        if let Some(val) = &vm.prologue_state.signal_grid[y][x] {
            if let Value::Int(n) = val {
                if *n == 10 {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "Flux did not output to any neighbor");
}

#[test]
fn test_pandemonium_scramble() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // [1,2,3] -> ! -> ¡
    // [1,2,3] at 5,4
    // ! at 5,5
    // ¡ at 5,6 (Reads 5,5. Output South 6,6)

    let list = Value::Junction(
        JunctionType::Any,
        vec![Value::Int(1), Value::Int(2), Value::Int(3)],
    );
    vm.grid[5][4] = list;
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("¡".to_string());

    exec_prologue_tick(&mut vm);

    if let Some(val) = &vm.prologue_state.signal_grid[6][6] {
        if let Value::Junction(_, items) = val {
            assert_eq!(items.len(), 3);
            let sums: i64 = items
                .iter()
                .map(|v| if let Value::Int(n) = v { *n } else { 0 })
                .sum();
            assert_eq!(sums, 6); // 1+2+3
        } else {
            panic!("Scramble did not produce a list");
        }
    } else {
        panic!("Scramble produced no signal");
    }
}
