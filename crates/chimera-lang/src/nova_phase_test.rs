use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_ethereal_movement() {
    // [ membrane(1) phase_shift(1) migrate(-1, 0) ]
    // 1. Build North Wall (Membrane 1).
    // 2. Shift to Ethereal.
    // 3. Migrate North (dy=-1, dx=0). Should succeed.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Membrane,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // Ethereal
        Gene {
            op: OpCode::PhaseShift,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(-1)],
        }, // dy
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }, // dx
        Gene {
            op: OpCode::Migrate,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.energy = 1000;

    // Default context is (8,8)

    vm.step(); // Push 1
    vm.step(); // Membrane (Build wall at North of 8,8)

    // Verify membrane at (8,8) has bit 1 set
    assert_eq!(vm.membranes[8][8] & 1, 1);

    vm.step(); // Push 1
    vm.step(); // PhaseShift -> Ethereal

    vm.step(); // Push -1
    vm.step(); // Push 0
    vm.step(); // Migrate

    // Should have moved to (7, 8) despite wall
    assert_eq!(vm.context_loc, (7, 8));
}

#[test]
fn test_crystalline_immunity() {
    // [ phase_shift(2) mutate() ]
    // Shift to Crystalline.
    // Trigger Mutate manually.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }, // Crystalline
        Gene {
            op: OpCode::PhaseShift,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.energy = 1000;

    vm.step(); // Push
    vm.step(); // PhaseShift

    let original_gene_op = vm.dna.helix.strands[0].genes[1].op.clone();

    // Force mutate many times
    for _ in 0..100 {
        vm.mutate();
    }

    let new_gene_op = vm.dna.helix.strands[0].genes[1].op.clone();
    assert_eq!(original_gene_op, new_gene_op);
}

#[test]
fn test_flux_speed() {
    // [ phase_shift(3) push(1) push(2) ]
    // Shift to Flux.
    // Next step should execute push(1) AND push(2).
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(3)],
        }, // Flux
        Gene {
            op: OpCode::PhaseShift,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.energy = 1000;

    vm.step(); // Push 3
    vm.step(); // PhaseShift. Consumes tick.

    // Now in Flux.
    // Next step() should execute 2 instructions.
    // push(1) and push(2).

    vm.step();

    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0], Value::Int(1));
    assert_eq!(vm.stack[1], Value::Int(2));
}

#[test]
fn test_ethereal_gwrite_fail() {
    // [ phase_shift(1) push(99) push(0) push(0) g_write() ]
    // Should fail to write.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::PhaseShift,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(99)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.energy = 1000;

    vm.step(); // Push 1
    vm.step(); // PhaseShift
    vm.step(); // Push 99
    vm.step(); // Push 0
    vm.step(); // Push 0
    vm.step(); // GWrite

    // Grid should still be 0
    assert_eq!(vm.grid[0][0], Value::Int(0));
    assert!(vm.output.last().unwrap().contains("Ethereal"));
}
