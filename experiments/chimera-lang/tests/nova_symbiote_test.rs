use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_spawn_symbiote() {
    let genes = vec![Gene {
        op: OpCode::SpawnSymbiote,
        args: vec![],
    }];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.step();
    assert!(vm.symbiote_entity.is_some());
    assert_eq!(vm.symbiote_entity.as_ref().unwrap().active, true);
    assert_eq!(vm.symbiote_entity.as_ref().unwrap().segments.len(), 1);
}

#[test]
fn test_feed_symbiote() {
    let genes = vec![
        Gene {
            op: OpCode::SpawnSymbiote,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(10)],
        },
        Gene {
            op: OpCode::FeedSymbiote,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    // Step 1: Spawn
    vm.step();
    assert!(vm.symbiote_entity.is_some());

    // Each step decreases symbiote energy by 1
    // Initial: 100
    // After Spawn step: 100 (it wasn't ticked during its spawn step)

    // Step 2: Push 10
    vm.step();
    // After Push: Symbiote energy is ticked to 99

    // Step 3: Feed 10
    vm.step();
    // During Feed: Symbiote is fed +10 (109), then ticked to 108.

    assert_eq!(
        vm.symbiote_entity.as_ref().unwrap().energy,
        107
    );
}

#[test]
fn test_symbiote_op() {
    // Op 0: Move Symbiote Head (dx, dy)
    let genes = vec![
        Gene {
            op: OpCode::SpawnSymbiote,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)], // dy
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // dx
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)], // Op 0: Move
        },
        Gene {
            op: OpCode::SymbioteOp,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Spawn
    vm.step();
    let initial_y = vm.symbiote_entity.as_ref().unwrap().segments[0].y;

    // Push args
    vm.step();
    vm.step();
    vm.step();

    // Execute SymbioteOp
    vm.step();

    // Check updated position
    assert_eq!(
        vm.symbiote_entity.as_ref().unwrap().segments[0].y,
        (initial_y + 1) % chimera_lang::vm::GRID_SIZE
    );
}
