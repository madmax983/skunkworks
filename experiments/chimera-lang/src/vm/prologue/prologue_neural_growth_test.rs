use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value};
use crate::vm::prologue::exec_prologue_tick;

#[test]
fn test_neural_growth() {
    let dna = Dna { evolution_config: None, helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Define Grammar Rule
    // "Grow": Move, Neuron, Move, Synapse, Move, Neuron
    // F N F S F N
    // Note: Logos requires literals to be quoted, otherwise they are treated as references.
    vm.prologue_state.logos_engine.define_rule("Grow", "\"F\" \"N\" \"F\" \"S\" \"F\" \"N\"");

    // Setup Grid
    // (5, 3): "Grow" (Rule Name)
    // (5, 4): "!" (Source)
    // (5, 5): "🌱" (Neural Growth Sink)
    vm.grid[5][3] = Value::Str("Grow".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("🌱".to_string());

    // Step 1: Scan & Propagate
    // Source emits "Grow" to (5, 4).
    // Sink 🌱 reads (5, 4), generates string, executes L-System.
    exec_prologue_tick(&mut vm);

    // Verify L-System Execution
    // Start at (5, 5) facing East.
    // F: Move to (5, 6). Draw ~.
    // N: Place ♦ at (5, 6). Overwrites ~.
    // F: Move to (5, 7). Draw ~.
    // S: Place • at (5, 7). Overwrites ~.
    // F: Move to (5, 8). Draw ~.
    // N: Place ♦ at (5, 8). Overwrites ~.

    let cell_6 = &vm.grid[5][6];
    let cell_7 = &vm.grid[5][7];
    let cell_8 = &vm.grid[5][8];

    assert_eq!(*cell_6, Value::Str("♦".to_string()), "Expected Neuron at (5, 6)");
    assert_eq!(*cell_7, Value::Str("•".to_string()), "Expected Synapse at (5, 7)");
    assert_eq!(*cell_8, Value::Str("♦".to_string()), "Expected Neuron at (5, 8)");

    // Verify Neurons were registered in the VM
    assert!(vm.neurons.contains_key(&(5, 6)), "Neuron not registered at (5, 6)");
    assert!(vm.neurons.contains_key(&(5, 8)), "Neuron not registered at (5, 8)");
}
