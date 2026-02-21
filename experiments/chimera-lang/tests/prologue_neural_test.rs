#![cfg(feature = "biophysics")]

use chimera_lang::ast::{Dna, Helix, Strand};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![Strand { genes: vec![] }] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_neural_creation() {
    let mut vm = make_vm();

    // Place Neuron Rune
    vm.grid[5][5] = Value::Str("♦".to_string());

    exec_prologue_tick(&mut vm);

    assert!(vm.neurons.contains_key(&(5, 5)), "Neuron should be created at (5,5)");
}

#[test]
fn test_neural_decay() {
    let mut vm = make_vm();

    // Place and create
    vm.grid[5][5] = Value::Str("♦".to_string());
    exec_prologue_tick(&mut vm);
    assert!(vm.neurons.contains_key(&(5, 5)));

    // Remove rune
    vm.grid[5][5] = Value::Int(0);
    exec_prologue_tick(&mut vm);
    assert!(!vm.neurons.contains_key(&(5, 5)), "Neuron should be removed after rune is gone");
}

#[test]
fn test_neural_firing() {
    let mut vm = make_vm();

    // Place Neuron
    vm.grid[5][5] = Value::Str("♦".to_string());
    // Ensure created
    exec_prologue_tick(&mut vm);

    // Inject massive voltage to force spike
    if let Some(neuron) = vm.neurons.get_mut(&(5, 5)) {
        neuron.v = 50.0; // Above threshold
        neuron.last_spike = vm.tick_counter; // Fake a spike *now*
    }

    // We need to simulate the timing.
    // fire_neurons checks: neuron.last_spike == vm.tick_counter.saturating_sub(1)
    // So if we set last_spike = 0, and tick = 1, it should fire.

    vm.tick_counter = 1;
    if let Some(neuron) = vm.neurons.get_mut(&(5, 5)) {
        neuron.last_spike = 0;
    }

    // Run prologue (fires based on last_spike)
    exec_prologue_tick(&mut vm);

    // Check signal grid
    assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(1)), "Neuron should emit signal");
}

#[test]
fn test_neural_integration() {
    let mut vm = make_vm();

    // Circuit: 10 -> ! -> ♦
    // 5,4: 10
    // 5,5: ! (Source) -> emits to 5,5? No, ! reads West, emits Self.
    // So:
    // 5,3: 10
    // 5,4: !
    // 5,5: ♦

    vm.grid[5][3] = Value::Int(10);
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("♦".to_string());

    // Tick 1: Scan & Prepare
    // ! reads 10, emits 10 to signal_grid[5][4].
    // ♦ sees neighbor 10.
    // ♦ integrate -> i_inj += 100.0 (10 * 10.0 scale).

    exec_prologue_tick(&mut vm);

    if let Some(neuron) = vm.neurons.get(&(5, 5)) {
        assert!(neuron.i_inj > 0.0, "Neuron should receive input current");
        assert_eq!(neuron.i_inj, 100.0);
    } else {
        panic!("Neuron missing");
    }
}
