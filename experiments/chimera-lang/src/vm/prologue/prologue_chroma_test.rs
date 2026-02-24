use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::ChimeraVM;
use crate::vm::Value;

#[test]
fn test_chroma_palette() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 255 -> ! -> (West) -> 🎨 -> (Self)
    // We need 3 inputs for Palette: W(R), N(G), E(B).
    // Let's set R=255, G=0, B=0.
    // 255 ! 🎨
    // This connects 255 (as ! source) to W of Palette.
    // N and E are empty (0).

    vm.grid[5][4] = Value::Int(255);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("🎨".to_string());

    exec_prologue_tick(&mut vm);

    // Check Self Output of Palette at 5,6
    if let Some(val) = &vm.prologue_state.signal_grid[5][6] {
        if let Value::Color(r, g, b) = val {
            assert_eq!(*r, 255);
            assert_eq!(*g, 0);
            assert_eq!(*b, 0);
        } else {
            panic!("Expected Color, got {:?}", val);
        }
    } else {
        panic!("Palette did not emit signal");
    }
}

#[test]
fn test_chroma_extraction() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 👁 (Eye) reads Green -> emits Color -> ~ -> 🟢 (Extract Green) -> Output Int
    // But Eye reads Chroma Grid.
    // Let's use Palette to generate Color, then wire to Extractor.
    // 255 ! 🎨 ~ 🟢
    // Palette emits (255, 0, 0) (Red).
    // Let's Extract Red using 🔴.

    vm.grid[5][4] = Value::Int(255);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("🎨".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());
    vm.grid[5][8] = Value::Str("🔴".to_string());

    exec_prologue_tick(&mut vm);

    // 🔴 outputs to North. So check (4, 8).
    if let Some(val) = &vm.prologue_state.signal_grid[4][8] {
        if let Value::Int(n) = val {
            assert_eq!(*n, 255);
        } else {
            panic!("Expected Int, got {:?}", val);
        }
    } else {
        panic!("Extractor did not emit signal to North");
    }
}

#[test]
fn test_chroma_brush() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 255 ! 🎨 🖌
    // Palette creates Red. Brush paints South (at 6, 7).

    vm.grid[5][4] = Value::Int(255);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("🎨".to_string());
    vm.grid[5][7] = Value::Str("🖌".to_string());

    exec_prologue_tick(&mut vm);

    // Check Chroma Grid at (6, 7)
    let cell = vm.chroma_grid[6][7];
    if let Some((r, g, b)) = cell.fg {
        assert_eq!(r, 255);
        assert_eq!(g, 0);
        assert_eq!(b, 0);
    } else {
        panic!("Brush did not paint Chroma Grid");
    }
}

#[test]
fn test_chroma_eye() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Chroma Grid manually at (5, 5) with Blue
    vm.chroma_grid[5][5].fg = Some((0, 0, 255));

    // Place Eye at (5, 5)
    vm.grid[5][5] = Value::Str("👁".to_string());

    exec_prologue_tick(&mut vm);

    // Eye should emit Color(0, 0, 255) to Self (5, 5)
    if let Some(val) = &vm.prologue_state.signal_grid[5][5] {
        if let Value::Color(r, g, b) = val {
            assert_eq!(*r, 0);
            assert_eq!(*g, 0);
            assert_eq!(*b, 255);
        } else {
            panic!("Expected Color, got {:?}", val);
        }
    } else {
        panic!("Eye did not emit signal");
    }
}
