use super::*;
use crate::ast::{Dna, Helix, JunctionType};
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_architect_rotate() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Blueprint: [[1, 2], [3, 4]]
    let row1 = Value::Junction(
        JunctionType::Dish,
        vec![Value::Int(1), Value::Int(2)],
    );
    let row2 = Value::Junction(
        JunctionType::Dish,
        vec![Value::Int(3), Value::Int(4)],
    );
    let blueprint = Value::Junction(JunctionType::Dish, vec![row1, row2]);

    // Setup:
    // (5, 4) = Blueprint
    // (5, 5) = Rotate Rune ⟳
    // (5, 6) = Result (Self)
    // Wait, Rotate is a propagation rune. It reads West and outputs to Self.
    // So if (5,5) is ⟳, it reads (5,4).
    // The result is stored in signal_grid[5][5].

    // We manually inject the signal because propagation depends on previous signals.
    // Or we can use Source (!) to emit it.
    // Let's use Source.

    // Grid Setup:
    // 5,3: Value(Blueprint)
    // 5,4: !
    // 5,5: ⟳

    // But blueprint is complex value.
    // Let's just inject into signal_grid to simulate propagation step.

    vm.grid[5][5] = Value::Str("⟳".to_string());
    vm.prologue_state.runes.insert((5, 5));

    // Inject signal at West (5, 4)
    vm.prologue_state.signal_grid[5][4] = Some(blueprint);

    // Run propagation (we need to call apply_architect_runes manually or run tick)
    // Let's run full tick to be safe, but we need to ensure signal persists.
    // Since we injected into signal_grid, and tick clears signal_grid at start of prepare_signals...
    // We should use delayed signals or just call the function directly for unit testing logic.

    // Calling function directly is better for unit test.
    let mut next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    architect::apply_architect_runes(
        "⟳",
        5,
        5,
        &vm.prologue_state.signal_grid,
        &mut next_signals,
    );

    let result = next_signals[5][5].clone();
    assert!(result.is_some());

    if let Some(Value::Junction(JunctionType::Dish, rows)) = result {
        // Expected: [[3, 1], [4, 2]]
        assert_eq!(rows.len(), 2);

        if let Value::Junction(JunctionType::Dish, r1) = &rows[0] {
            assert_eq!(r1[0], Value::Int(3));
            assert_eq!(r1[1], Value::Int(1));
        } else { panic!("Row 1 format error"); }

        if let Value::Junction(JunctionType::Dish, r2) = &rows[1] {
            assert_eq!(r2[0], Value::Int(4));
            assert_eq!(r2[1], Value::Int(2));
        } else { panic!("Row 2 format error"); }
    } else {
        panic!("Result is not a Blueprint");
    }
}

#[test]
fn test_architect_life() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);

    // Blinker (Vertical):
    // 0 1 0
    // 0 1 0
    // 0 1 0
    let row1 = Value::Junction(JunctionType::Dish, vec![Value::Int(0), Value::Int(1), Value::Int(0)]);
    let row2 = Value::Junction(JunctionType::Dish, vec![Value::Int(0), Value::Int(1), Value::Int(0)]);
    let row3 = Value::Junction(JunctionType::Dish, vec![Value::Int(0), Value::Int(1), Value::Int(0)]);
    let blueprint = Value::Junction(JunctionType::Dish, vec![row1, row2, row3]);

    vm.grid[5][5] = Value::Str("▓".to_string());
    vm.prologue_state.signal_grid[5][4] = Some(blueprint);

    let mut next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    architect::apply_architect_runes(
        "▓",
        5,
        5,
        &vm.prologue_state.signal_grid,
        &mut next_signals,
    );

    let result = next_signals[5][5].clone();
    assert!(result.is_some());

    if let Some(Value::Junction(JunctionType::Dish, rows)) = result {
        // Expected Blinker (Horizontal):
        // 0 0 0
        // 1 1 1
        // 0 0 0

        if let Value::Junction(JunctionType::Dish, r1) = &rows[0] {
            assert_eq!(r1[1], Value::Int(0));
        }
        if let Value::Junction(JunctionType::Dish, r2) = &rows[1] {
            assert_eq!(r2[0], Value::Int(1));
            assert_eq!(r2[1], Value::Int(1));
            assert_eq!(r2[2], Value::Int(1));
        }
        if let Value::Junction(JunctionType::Dish, r3) = &rows[2] {
            assert_eq!(r3[1], Value::Int(0));
        }
    } else {
        panic!("Result is not a Blueprint");
    }
}
