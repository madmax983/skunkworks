#[cfg(test)]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(test)]
use crate::opcode::OpCode;
#[cfg(test)]
use crate::vm::ChimeraVM;

#[cfg(test)]
fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_membrane_blocking() {
    // 1. Build a wall to the East (Mask 4) at (8,8).
    // 2. Try to migrate East (0, 1). Should fail.

    let genes = vec![
        // Build East Wall (Mask 4)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        },
        Gene {
            op: OpCode::Membrane,
            args: vec![],
        },
        // Try to migrate East (dy=0, dx=1)
        // Stack order: dy, dx (top)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // dy
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // dx
        },
        Gene {
            op: OpCode::Migrate,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.context_loc = (8, 8); // Start at center

    // Step 1: Push 4
    vm.step();
    // Step 2: Membrane
    vm.step();

    // Verify wall exists at (8,8) bit 4
    #[cfg(feature = "nova")]
    {
        assert_eq!(vm.membranes[8][8] & 4, 4, "East wall should be set");
        // Verify reciprocal West wall at (8,9) bit 8
        assert_eq!(
            vm.membranes[8][9] & 8,
            8,
            "Reciprocal West wall should be set"
        );
    }

    // Step 3: Push 1
    vm.step();
    // Step 4: Push 0
    vm.step();
    // Step 5: Migrate
    vm.step();

    // Should still be at (8,8)
    assert_eq!(
        vm.context_loc,
        (8, 8),
        "Migration should be blocked by membrane"
    );
}

#[test]
fn test_osmosis_permeability() {
    // 1. Build a wall to the East.
    // 2. Use Osmosis to move East. Should succeed.

    let genes = vec![
        // Build East Wall (Mask 4)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        },
        Gene {
            op: OpCode::Membrane,
            args: vec![],
        },
        // Osmosis East (dy=0, dx=1)
        // Stack order: dy, dx (top)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // dy
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // dx
        },
        Gene {
            op: OpCode::Osmosis,
            args: vec![],
        },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.context_loc = (8, 8);

    // Run until finished
    for _ in 0..10 {
        if vm.halted {
            break;
        }
        vm.step();
    }

    // Should be at (8,9)
    assert_eq!(vm.context_loc, (8, 9), "Osmosis should penetrate membrane");
}
