use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_mycelium_growth() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // (5,5): 🍄 (Spore)
    // (5,4): 100 (Nutrient)
    // (5,6): 0 (Empty)

    vm.grid[5][5] = Value::Str("🍄".to_string());
    vm.grid[5][4] = Value::Int(100);

    // Initial Scan
    exec_prologue_tick(&mut vm);

    // Expectation:
    // 1. Nutrient (100) should be consumed (replaced with 0 or smaller).
    // 2. A new 🍄 should appear in an adjacent empty cell (e.g., (5,6), (4,5), (6,5)).

    let nutrient_eaten = match vm.grid[5][4] {
        Value::Int(n) => n < 100,
        _ => true, // Consumed entirely
    };
    assert!(nutrient_eaten, "Nutrient was not consumed");

    // Check for new Spore
    let neighbors = [(4, 5), (6, 5), (5, 6)]; // (5,4) was nutrient
    let mut found_new_spore = false;
    for (ny, nx) in neighbors {
        if let Value::Str(s) = &vm.grid[ny][nx] {
            if s == "🍄" {
                found_new_spore = true;
                break;
            }
        }
    }
    assert!(found_new_spore, "No new Spore grew");
}
