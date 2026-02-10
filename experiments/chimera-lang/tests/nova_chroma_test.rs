#![cfg(feature = "nova")]

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

#[test]
fn test_chroma_red_fury() {
    let genes = vec![
        Gene { op: OpCode::ChromaShift, args: vec![] },
        // Pigment(r=255, g=0, b=0, y=8, x=8)
        // Stack order needed: [r, g, b, y, x] (Top is x)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(255)] }, // r
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // g
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // b
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // x
        Gene { op: OpCode::Pigment, args: vec![] },
        // Wait one tick
        Gene { op: OpCode::Nop, args: vec![] },
        Gene { op: OpCode::Nop, args: vec![] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };

    let mut vm = ChimeraVM::new(dna);
    vm.context_loc = (8, 8);

    // Run
    // Step 1: ChromaShift ON
    vm.step();
    assert!(vm.chroma_shift_mode);

    // Step 2-7: Push args and Pigment
    // Push x 5
    vm.step(); vm.step(); vm.step(); vm.step(); vm.step();
    // Pigment
    vm.step();

    // Verify grid color
    let cell = &vm.chroma_grid[8][8];
    assert_eq!(cell.fg, Some((255, 0, 0)), "Pigment failed to paint red");

    // Step 8: Nop. Should trigger Chroma Interaction.
    vm.step();

    assert!(vm.output.iter().any(|s| s.contains("CHROMA: Fury (Red)")), "Did not detect Red Fury");
}

#[test]
fn test_chroma_green_growth() {
    let genes = vec![
        Gene { op: OpCode::ChromaShift, args: vec![] },
        // Pigment(r=0, g=255, b=0, y=8, x=8)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // r
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(255)] }, // g
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // b
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // x
        Gene { op: OpCode::Pigment, args: vec![] },
        Gene { op: OpCode::Nop, args: vec![] },
    ];

    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);
    vm.context_loc = (8, 8);

    // Run until Pigment
    for _ in 0..7 {
        vm.step();
    }

    assert!(vm.chroma_shift_mode);
    assert_eq!(vm.chroma_grid[8][8].fg, Some((0, 255, 0)), "Pigment failed to paint green");

    let energy_before = vm.energy;
    vm.step(); // Nop + Chroma Green

    // Green: +1 Energy. Base cost: -1. Net: 0 change.
    assert_eq!(vm.energy, energy_before, "Energy should balance out");
    assert!(vm.output.iter().any(|s| s.contains("CHROMA: Growth (Green)")));
}

#[test]
fn test_chroma_blue_stasis() {
    let genes = vec![
        Gene { op: OpCode::ChromaShift, args: vec![] },
        // Pigment(r=0, g=0, b=255, y=8, x=8)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // r
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },   // g
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(255)] }, // b
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },   // x
        Gene { op: OpCode::Pigment, args: vec![] },
        Gene { op: OpCode::Nop, args: vec![] },
    ];

    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);
    vm.context_loc = (8, 8);
    // reduce telomere to verify restoration
    vm.telomeres[0] = 10;

    // Run until Pigment
    for _ in 0..7 {
        vm.step();
    }

    vm.step(); // Nop + Chroma Blue

    assert!(vm.output.iter().any(|s| s.contains("CHROMA: Stasis (Blue)")));
    // Logic: 10 -> -1 (check) +1 (stasis) = 10.
    assert!(vm.telomeres[0] >= 10, "Telomeres should be preserved");
}
