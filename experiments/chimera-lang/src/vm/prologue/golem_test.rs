use crate::ast::{Dna, Helix};
use crate::vm::prologue::{exec_prologue_tick, MAT_BRICK, MAT_HEART, MAT_WALL};
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_golem_movement() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Golem
    // . . .
    // . 🗿 .
    // . 🧱 .
    let center_y = 5;
    let center_x = 5;
    vm.grid[center_y][center_x] = Value::Str(MAT_HEART.to_string());
    vm.grid[center_y + 1][center_x] = Value::Str(MAT_BRICK.to_string());

    // Signal to Move East (1)
    // Signal must be at Heart pos
    // We use delayed_signals because prepare_signals clears signal_grid at start of tick
    vm.prologue_state.delayed_signals[center_y][center_x] = Some(Value::Int(1));

    exec_prologue_tick(&mut vm);

    // Check Heart moved East
    assert_eq!(
        vm.grid[center_y][center_x + 1],
        Value::Str(MAT_HEART.to_string())
    );
    // Check Brick moved East
    assert_eq!(
        vm.grid[center_y + 1][center_x + 1],
        Value::Str(MAT_BRICK.to_string())
    );

    // Check Old spots empty
    assert_eq!(vm.grid[center_y][center_x], Value::Int(0));
    assert_eq!(vm.grid[center_y + 1][center_x], Value::Int(0));
}

#[test]
fn test_golem_collision() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup Golem moving East
    // . . #
    // . 🗿 .
    // . 🧱 .
    // Wall at (5, 7) - Should block movement to (5,6)?? No, Heart is at (5,5), moving to (5,6).
    // Let's put a wall at (5, 6) directly blocking the heart.
    let center_y = 5;
    let center_x = 5;
    vm.grid[center_y][center_x] = Value::Str(MAT_HEART.to_string());
    vm.grid[center_y + 1][center_x] = Value::Str(MAT_BRICK.to_string());

    // Obstacle (Must NOT be a Golem material, otherwise it becomes part of the body!)
    // Using "X" as an immovable object/obstacle
    vm.grid[center_y][center_x + 1] = Value::Str("X".to_string());

    // Signal to Move East
    vm.prologue_state.delayed_signals[center_y][center_x] = Some(Value::Int(1));

    exec_prologue_tick(&mut vm);

    // Should NOT move
    assert_eq!(
        vm.grid[center_y][center_x],
        Value::Str(MAT_HEART.to_string())
    );
    assert_eq!(
        vm.grid[center_y + 1][center_x],
        Value::Str(MAT_BRICK.to_string())
    );
}

#[test]
fn test_golem_complex_shape() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Shape:
    //   🧱
    // 🧱🗿🧱
    //   🧱
    let cy = 10;
    let cx = 10;

    vm.grid[cy][cx] = Value::Str(MAT_HEART.to_string());
    vm.grid[cy - 1][cx] = Value::Str(MAT_BRICK.to_string()); // Top
    vm.grid[cy + 1][cx] = Value::Str(MAT_BRICK.to_string()); // Bottom
    vm.grid[cy][cx - 1] = Value::Str(MAT_BRICK.to_string()); // Left
    vm.grid[cy][cx + 1] = Value::Str(MAT_BRICK.to_string()); // Right

    // Signal South (2)
    vm.prologue_state.delayed_signals[cy][cx] = Some(Value::Int(2));

    exec_prologue_tick(&mut vm);

    // Expected New Center (11, 10)
    assert_eq!(vm.grid[cy + 1][cx], Value::Str(MAT_HEART.to_string()));

    // Top brick should be at (10, 10) - where heart was
    assert_eq!(vm.grid[cy][cx], Value::Str(MAT_BRICK.to_string()));

    // Bottom brick should be at (12, 10)
    assert_eq!(vm.grid[cy + 2][cx], Value::Str(MAT_BRICK.to_string()));
}
