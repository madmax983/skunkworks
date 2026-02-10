#![cfg(all(test, feature = "nova"))]

use crate::ast::{Dna, Helix, Strand};
use crate::opcode::OpCode;
use crate::vm::nova_fluid;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let genes = vec![];
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_storm_and_aeolus() {
    let mut vm = make_vm();
    let (cy, cx) = vm.context_loc;

    // Test Storm (Add Moisture)
    // Stack: [ intensity, radius ] (Bottom -> Top)
    // exec pops: radius, then intensity.
    // So push intensity first, then radius.
    vm.stack.push(Value::Int(100)); // Intensity
    vm.stack.push(Value::Int(2)); // Radius

    nova_fluid::exec_storm(&mut vm, OpCode::Storm, &[]);

    // Check center
    assert_eq!(
        vm.moisture_grid[cy][cx], 100,
        "Storm failed to add moisture"
    );
    // Check neighbor
    if cy > 0 {
        assert_eq!(vm.moisture_grid[cy - 1][cx], 100, "Storm radius failed");
    }

    // Test Aeolus (Set Wind)
    // Stack: [ angle, strength ]
    vm.stack.push(Value::Int(2)); // Angle 2 (East)
    vm.stack.push(Value::Int(5)); // Strength

    nova_fluid::exec_aeolus(&mut vm, OpCode::Aeolus, &[]);

    assert_eq!(vm.wind_grid[cy][cx], (0, 5), "Aeolus failed to set wind");
}

#[test]
fn test_process_fluid_advection() {
    let mut vm = make_vm();
    let (cy, cx) = vm.context_loc;

    // Setup: Moisture at center, Wind blowing East (0, 5)
    vm.moisture_grid[cy][cx] = 1000;
    vm.wind_grid[cy][cx] = (0, 5); // Strong wind East

    // Step fluid simulation
    nova_fluid::process_fluid(&mut vm);

    let target_x = cx + 5;
    if target_x < 16 {
        assert!(
            vm.moisture_grid[cy][target_x] > 0,
            "Moisture should advect to target"
        );
        assert!(
            vm.moisture_grid[cy][cx] < 1000,
            "Source should lose moisture"
        );
    }
}

#[test]
fn test_tsunami() {
    let mut vm = make_vm();
    let (cy, cx) = vm.context_loc;

    // Tsunami East (0)
    // Stack: [ direction, power ]
    vm.stack.push(Value::Int(0)); // Dir 0 (East)
    vm.stack.push(Value::Int(10)); // Power

    nova_fluid::exec_tsunami(&mut vm, OpCode::Tsunami, &[]);

    // Check line of effect
    // Should set wind to (0, 10) for 8 cells east
    for i in 0..8 {
        if cx + i < 16 {
            assert_eq!(
                vm.wind_grid[cy][cx + i],
                (0, 10),
                "Wind not set at index {}",
                i
            );
            assert_eq!(
                vm.moisture_grid[cy][cx + i],
                50,
                "Moisture not added at index {}",
                i
            );
        }
    }
}

#[test]
fn test_dry() {
    let mut vm = make_vm();
    let (cy, cx) = vm.context_loc;

    // Add moisture
    vm.moisture_grid[cy][cx] = 100;

    // Dry
    // Stack: [ radius ]
    vm.stack.push(Value::Int(5));

    nova_fluid::exec_dry(&mut vm, OpCode::Dry, &[]);

    assert_eq!(vm.moisture_grid[cy][cx], 0, "Dry failed to remove moisture");
}
