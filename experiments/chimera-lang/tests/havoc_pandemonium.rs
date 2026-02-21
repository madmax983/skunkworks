use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::ast::{Dna, Helix};

#[test]
fn test_fuse_explosion_capped() {
    // Regression test for unbounded string growth in Alchemy/Pandemonium.
    // Verifies that string length is capped at MAX_STRING_LEN (65536).

    // Setup VM
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Place Fuse Rune at (5,5)
    vm.grid[5][5] = Value::Str("f".to_string());

    // Setup Sources for perpendicular injection
    // Input 1 (West) -> (5,4)
    vm.grid[5][4] = Value::Str("!".to_string());

    // Input 2 (East) via wire path
    vm.grid[4][7] = Value::Str("!".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());
    vm.grid[5][6] = Value::Str("~".to_string());

    // Initial Input
    let initial_val = Value::Str("A".to_string());
    vm.grid[5][3] = initial_val.clone();
    vm.grid[4][6] = initial_val.clone();

    // Run for 20 ticks. Without cap, 2^20 = 1,048,576 > 65536.
    for _ in 0..20 {
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        if let Some(output) = &vm.prologue_state.signal_grid[5][5] {
            if let Value::Str(_) = output {
                // Feedback
                vm.grid[5][3] = output.clone();
                vm.grid[4][6] = output.clone();
            }
        }
    }

    // Assert limit
    if let Value::Str(final_s) = &vm.grid[5][3] {
        assert!(final_s.len() <= 65536, "String exceeded limit! Len: {}", final_s.len());
        // Also assert it grew large enough to hit the limit (unless it started huge)
        assert!(final_s.len() >= 65536, "String didn't reach limit (growth failure?)");
    } else {
        panic!("Final result was not a string");
    }
}
