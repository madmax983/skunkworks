use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::ChimeraVM;
use crate::vm::Value;
use crate::vm::prologue::exec_prologue_tick;

#[test]
fn test_prologue_resonance_note() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 60 -> ! -> ~ -> ♪
    // (5,4)=60
    // (5,5)=!
    // (6,5)=~
    // (7,5)=♪
    // Note: Rune scan happens first. ! at 5,5 reads West 5,4 (60). Emits to 5,5.
    // ~ at 6,5 reads North (5,5).
    // ♪ at 7,5 reads West?
    // Wait. My circuit construction might be off.

    // In scan_grid_rules: ! reads West.
    // ! at 5,5 reads 60 at 5,4.

    // In propagation: ~ at 6,5 needs signal.
    // ! emits signal to 5,5.
    // ~ at 6,5 is adjacent to 5,5.
    // If wire connects, it propagates.
    // ~ conducts in all directions.

    // ♪ at 7,5 reads WEST.
    // So 7,4 must have signal.

    // My circuit puts ~ at 6,5 (North of ♪).
    // ♪ does NOT read North. It reads West.

    // So I need wire at 7,4.

    // Correct Circuit:
    // 60 -> ! -> ~ -> ♪
    // (7,2)=60
    // (7,3)=!  (Emits to 7,3)
    // (7,4)=~  (Conducts from 7,3 to 7,4)
    // (7,5)=♪  (Reads West 7,4)

    vm.grid[7][2] = Value::Int(60);
    vm.grid[7][3] = Value::Str("!".to_string());
    vm.grid[7][4] = Value::Str("~".to_string());
    vm.grid[7][5] = Value::Str("♪".to_string());

    exec_prologue_tick(&mut vm);

    // Verify Output
    let output = vm.output.join("\n");
    // "RESONANCE: Note 60 (261.63Hz) at 5,7"
    assert!(output.contains("RESONANCE: Note 60"), "Output was: {}", output);
}

#[test]
fn test_prologue_resonance_chord() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Rune at 7,5 = ♫
    // West (7,4) needs signal (Root).
    // North (6,5) needs signal (Type).

    // West Path
    vm.grid[7][2] = Value::Int(60);
    vm.grid[7][3] = Value::Str("!".to_string());
    vm.grid[7][4] = Value::Str("~".to_string());

    // North Path
    // Source (!) at 5,5 reads from West (5,4)
    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("~".to_string());

    // Rune
    vm.grid[7][5] = Value::Str("♫".to_string());

    exec_prologue_tick(&mut vm);

    let output = vm.output.join("\n");
    assert!(output.contains("RESONANCE: Chord 60 (Type 1)"), "Output was: {}", output);
}

#[test]
fn test_prologue_resonance_drum() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 1 -> ! -> ~ -> 🥁
    // Use horizontal layout
    vm.grid[7][2] = Value::Int(1);
    vm.grid[7][3] = Value::Str("!".to_string());
    vm.grid[7][4] = Value::Str("~".to_string());
    vm.grid[7][5] = Value::Str("🥁".to_string());

    exec_prologue_tick(&mut vm);

    let output = vm.output.join("\n");
    assert!(output.contains("RESONANCE: Drum at 5,7"), "Output was: {}", output);
}
