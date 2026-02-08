#![cfg(test)]
#![cfg(feature = "nova")]

use super::ast::{Dna, Helix, Strand};
use super::vm::ChimeraVM;
use super::vm::Value;

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_resonate_updates_grid() {
    let mut vm = make_vm();

    // [ push(440), push(100), resonate() ]
    // Stack: frequency, amplitude
    vm.stack.push(Value::Int(440)); // Frequency
    vm.stack.push(Value::Int(100)); // Amplitude

    // Set location to (8, 8)
    vm.context_loc = (8, 8);

    // Exec resonate
    // We access via crate::vm because super::vm inside test module might be tricky?
    // Wait, super::vm works if we are in src/nova_resonance_war_test.rs which is a module of lib.rs.
    // But exec_resonate is in vm::nova_resonance_war.

    crate::vm::nova_resonance_war::exec_resonate(&mut vm);

    // Check grid at (8, 8)
    let (freq, amp) = vm.resonance_grid[8][8];
    assert_eq!(freq, 440.0);
    assert_eq!(amp, 100.0);

    // Check neighbor diffusion (8, 9)
    let (n_freq, n_amp) = vm.resonance_grid[8][9];
    assert_eq!(n_freq, 440.0);
    assert_eq!(n_amp, 50.0); // 50% diffusion
}

#[test]
fn test_sonic_claim_success() {
    let mut vm = make_vm();
    let owner = 0;

    // Setup grid: 440Hz @ 20.0 (Sufficient)
    vm.context_loc = (5, 5);
    vm.resonance_grid[5][5] = (440.0, 20.0);

    // [ push(440), sonic_claim() ]
    vm.stack.push(Value::Int(440));

    crate::vm::nova_resonance_war::exec_sonic_claim(&mut vm);

    assert_eq!(vm.sovereignty_grid[5][5], Some(owner));
    assert!(vm.output.last().unwrap().contains("claimed"));
}

#[test]
fn test_sonic_claim_fail_freq() {
    let mut vm = make_vm();

    // Setup grid: 400Hz @ 20.0 (Mismatch)
    vm.context_loc = (5, 5);
    vm.resonance_grid[5][5] = (400.0, 20.0);

    // [ push(440), sonic_claim() ]
    vm.stack.push(Value::Int(440));

    crate::vm::nova_resonance_war::exec_sonic_claim(&mut vm);

    assert_eq!(vm.sovereignty_grid[5][5], None);
    assert!(vm.output.last().unwrap().contains("Frequency mismatch"));
}

#[test]
fn test_sonic_claim_fail_amp() {
    let mut vm = make_vm();

    // Setup grid: 440Hz @ 5.0 (Too weak)
    vm.context_loc = (5, 5);
    vm.resonance_grid[5][5] = (440.0, 5.0);

    // [ push(440), sonic_claim() ]
    vm.stack.push(Value::Int(440));

    crate::vm::nova_resonance_war::exec_sonic_claim(&mut vm);

    assert_eq!(vm.sovereignty_grid[5][5], None);
    assert!(vm.output.last().unwrap().contains("Signal too weak"));
}

#[test]
fn test_dampen() {
    let mut vm = make_vm();

    // Setup grid
    vm.context_loc = (5, 5);
    vm.resonance_grid[5][5] = (440.0, 100.0);

    // [ push(1), push(50), dampen() ] -> Radius 1, Amount 50
    vm.stack.push(Value::Int(1)); // Radius
    vm.stack.push(Value::Int(50)); // Amount

    crate::vm::nova_resonance_war::exec_dampen(&mut vm);

    let (_, amp) = vm.resonance_grid[5][5];
    assert_eq!(amp, 50.0);
}
