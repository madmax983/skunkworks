use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::ChimeraVM;
use crate::vm::Value;

#[test]
fn test_biolum_emitter() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 100 -> ! -> ~ -> Φ
    // ! reads West (4,3).
    // ! (4,4) emits to self.
    // ~ (4,5) reads West (4,4).
    // Φ (4,6) reads West (4,5).

    vm.grid[4][3] = Value::Int(100);
    vm.grid[4][4] = Value::Str("!".to_string());
    vm.grid[4][5] = Value::Str("~".to_string());
    vm.grid[4][6] = Value::Str("Φ".to_string());

    exec_prologue_tick(&mut vm);

    // Check vm.light_grid[4][6]
    // Intensity should be 100
    assert_eq!(vm.light_grid[4][6], 100);

    // Check color (default White if no N/E/S inputs)
    assert_eq!(vm.light_color_grid[4][6], (255, 255, 255));
}

#[test]
fn test_biolum_sensor() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: Λ -> ?
    // (5,5) = Λ
    // (6,5) = ?

    vm.grid[5][5] = Value::Str("Λ".to_string());
    vm.grid[6][5] = Value::Str("?".to_string());

    // Set light at 5,5
    vm.light_grid[5][5] = 50;

    exec_prologue_tick(&mut vm);

    // Λ should emit signal 50 to Self
    // ? should read signal from North (Self of Λ)

    // Check if ? received signal
    // Output should contain log
    let output = vm.output.join("\n");
    assert!(output.contains("PROLOGUE: Sink at 5,6 received Int(50)"));
}

#[test]
fn test_biolum_absorber() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 50 -> ! -> ~ -> Ω
    // ! reads West (4,3).

    vm.grid[4][3] = Value::Int(50);
    vm.grid[4][4] = Value::Str("!".to_string());
    vm.grid[4][5] = Value::Str("~".to_string());
    vm.grid[4][6] = Value::Str("Ω".to_string());

    // Initial light at Ω (4,6)
    vm.light_grid[4][6] = 100;

    exec_prologue_tick(&mut vm);

    // Should absorb 50. Result 50.
    assert_eq!(vm.light_grid[4][6], 50);
}
