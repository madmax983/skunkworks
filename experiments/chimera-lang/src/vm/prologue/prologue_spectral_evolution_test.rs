use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::ChimeraVM;
use crate::vm::Value;

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

// Helper to setup a circuit with West and East inputs
// West Input: 10
// East Input: 5
// Rune at (5, 6)
// Output at (6, 6) (South)
fn setup_circuit(vm: &mut ChimeraVM, rune: &str, color: (u8, u8, u8)) {
    // Clear grid area
    for y in 0..16 {
        for x in 0..16 {
            vm.grid[y][x] = Value::Int(0);
        }
    }

    // West Input Chain
    vm.grid[5][4] = Value::Int(10);
    vm.grid[5][5] = Value::Str("!".to_string());

    // East Input Chain (Via North detour)
    vm.grid[4][6] = Value::Int(5);
    vm.grid[4][7] = Value::Str("!".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());

    // Rune
    vm.grid[5][6] = Value::Str(rune.to_string());
    vm.chroma_grid[5][6].fg = Some(color);
}

// Helper to setup Green Sub inputs (Hello, l)
fn setup_circuit_strings(vm: &mut ChimeraVM, rune: &str, w: &str, e: &str, color: (u8, u8, u8)) {
    for y in 0..16 {
        for x in 0..16 {
            vm.grid[y][x] = Value::Int(0);
        }
    }
    vm.grid[5][4] = Value::Str(w.to_string());
    vm.grid[5][5] = Value::Str("!".to_string());

    vm.grid[4][6] = Value::Str(e.to_string());
    vm.grid[4][7] = Value::Str("!".to_string());
    vm.grid[5][7] = Value::Str("~".to_string());

    vm.grid[5][6] = Value::Str(rune.to_string());
    vm.chroma_grid[5][6].fg = Some(color);
}

#[test]
fn test_spectral_red_logic() {
    let mut vm = setup_vm();
    let red = (255, 0, 0);

    // 1. Red Sub (-) -> Clamped Sub (10 - 5 = 5)
    setup_circuit(&mut vm, "-", red);
    exec_prologue_tick(&mut vm);
    if let Some(Value::Int(res)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*res, 5, "Red Sub failed");
    } else {
        panic!(
            "Red Sub no signal: {:?}",
            vm.prologue_state.signal_grid[6][6]
        );
    }

    // 2. Red XOR (^) -> Annihilation (10 + 5 = 15)
    setup_circuit(&mut vm, "^", red);
    exec_prologue_tick(&mut vm);
    if let Some(Value::Int(res)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*res, 15, "Red XOR failed");
    } else {
        panic!("Red XOR no signal");
    }
}

#[test]
fn test_spectral_green_logic() {
    let mut vm = setup_vm();
    let green = (0, 255, 0);

    // 1. Green Sub (-) -> Pruning
    setup_circuit_strings(&mut vm, "-", "Hello", "l", green);
    exec_prologue_tick(&mut vm);
    if let Some(Value::Str(res)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(res, "Heo", "Green Sub failed");
    } else {
        panic!("Green Sub no signal");
    }

    // 2. Green Div (/) -> Mitosis (10 / 2 = 5 N, 5 S)
    // Setup inputs: W=10, E=2. Use setup_circuit but overwrite E.
    setup_circuit(&mut vm, "/", green);
    vm.grid[4][6] = Value::Int(2); // Set East input to 2
    exec_prologue_tick(&mut vm);

    // North (4, 6)
    // Note: Rune at (5,6) outputs to (4,6). But (4,6) was input value '2'.
    // signal_grid[4][6] should contain the output.
    if let Some(Value::Int(n)) = &vm.prologue_state.signal_grid[4][6] {
        assert_eq!(*n, 5, "Green Div North failed");
    } else {
        panic!("Green Div North no signal");
    }
    // South (6, 6)
    if let Some(Value::Int(s)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*s, 5, "Green Div South failed");
    } else {
        panic!("Green Div South no signal");
    }
}

#[test]
fn test_spectral_blue_logic() {
    let mut vm = setup_vm();
    let blue = (0, 0, 255);

    // 1. Blue Sub (-) -> Inverse (E - W) -> (5 - 10 = -5)
    setup_circuit(&mut vm, "-", blue);
    exec_prologue_tick(&mut vm);
    if let Some(Value::Int(res)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*res, -5, "Blue Sub failed");
    } else {
        panic!("Blue Sub no signal");
    }

    // 2. Blue OR (|) -> NAND
    // Inputs: W=1, E=1.
    setup_circuit(&mut vm, "|", blue);
    vm.grid[5][4] = Value::Int(1);
    vm.grid[4][6] = Value::Int(1);
    exec_prologue_tick(&mut vm);

    if let Some(Value::Int(res)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*res, 0, "Blue NAND failed");
    } else {
        panic!("Blue NAND no signal");
    }
}
