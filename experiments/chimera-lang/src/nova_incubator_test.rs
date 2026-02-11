#![cfg(test)]
#![cfg(feature = "nova")]

use crate::prelude::*;
use crate::vm::nova_genetics::Incubator;

#[test]
fn test_incubator_initialization() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Photosynthesize, args: vec![] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let vm = ChimeraVM::new(dna);

    let incubator = Incubator::new(&vm, 0, 8);
    assert_eq!(incubator.population.len(), 8);
    assert_eq!(incubator.generation, 0);
    assert!(!incubator.running);
}

#[test]
fn test_incubator_evolution() {
    let genes = vec![
        Gene { op: OpCode::Photosynthesize, args: vec![] },
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let vm = ChimeraVM::new(dna);

    let mut incubator = Incubator::new(&vm, 0, 16);
    incubator.running = true;
    incubator.max_ticks_per_gen = 10;

    // Run for 1 generation
    for _ in 0..15 { // 10 ticks + buffer
        incubator.tick();
    }

    assert!(incubator.generation >= 1);

    // Check if fitness history is populated
    assert!(!incubator.history.is_empty());

    // Check if population is still full
    assert_eq!(incubator.population.len(), 16);
}

#[test]
fn test_incubator_selection() {
    // Create a VM where one strand is clearly better if mutated manually (simulated)
    // But Incubator clones the SAME strand initially.
    // So all start equal.
    // Random mutations should create divergence.

    let genes = vec![
        Gene { op: OpCode::Nop, args: vec![] }, // Does nothing
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let vm = ChimeraVM::new(dna);

    let mut incubator = Incubator::new(&vm, 0, 20);
    incubator.running = true;
    incubator.max_ticks_per_gen = 5;

    // Run for multiple generations
    for _ in 0..50 {
        incubator.tick();
    }

    assert!(incubator.generation >= 5);

    // We can't guarantee fitness increase because random mutation might fail,
    // but we can check that we have a best fitness >= initial (0 energy + 10 alive bonus).
    // Actually, Nop+Jump costs 1 energy per tick (metabolism).
    // So energy decreases.
    // Initial: 50. 5 ticks -> 45. Fitness ~ 55.

    assert!(incubator.best_fitness > 0);

    // Check best strand retrieval
    let best = incubator.best_strand();
    assert!(best.is_some());
}
