use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::nova::OrganelleType;
use crate::vm::{ChimeraVM, Value};

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix { strands },
    }
}

#[test]
fn test_void_spawn_and_consume() {
    let strand = Strand {
        genes: vec![Gene {
            op: OpCode::Void,
            args: vec![],
        }],
    };
    let mut vm = ChimeraVM::new(make_dna(vec![strand]));

    // Place some food on the grid
    vm.grid[8][8] = Value::Int(100); // Center where void spawns
    vm.grid[8][9] = Value::Int(200); // Neighbor

    vm.step(); // Execute Void

    // Check if Void spawned
    assert!(!vm.organelles.is_empty());
    let organelle = &vm.organelles[0];
    assert_eq!(organelle.kind, OrganelleType::Void);
    assert_eq!(organelle.context_loc, (8, 8));

    // Verify immediate consumption might happen in next step or tick?
    // OpCode::Void spawns it. It hasn't ticked yet.

    vm.step(); // Tick organelle. Should consume (8,8) if it wasn't cleared by spawn?
               // Wait, spawn doesn't clear. Void tick clears.
               // And Void tick moves randomly.

    // The Void logic:
    // 1. Consume current loc.
    // 2. Move.

    // After 1 step (which runs process_organelles), it should have consumed (8,8).
    assert_eq!(vm.grid[8][8], Value::Int(0));
    // Energy gain?
    // Base 50 - 50 (spawn cost) = 0.
    // +1 (consumed) -1 (tick cost) = 0.
    // Wait, vm.energy is shared? No, organelle has no separate energy?
    // Organelle swaps state with VM.
    // VM energy is modified.
    // Spawn cost is 50.
    // Void adds 1 energy per consumption.
    // Tick costs 1.
    // So net change per step is 0 if consuming.

    // Let's verify grid consumption mainly.
}

#[test]
fn test_supernova() {
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Supernova,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
        ],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0]));

    vm.step(); // push(1)
    vm.step(); // supernova

    // Strand 0 genes should be empty
    assert!(vm.dna.helix.strands[0].genes.is_empty());
    assert!(vm.halted);

    // Check grid for scattered genes
    // "push(1)", "supernova", "push(2)" should be somewhere on the grid
    let mut found_genes = 0;
    for row in &vm.grid {
        for cell in row {
            if let Value::Str(s) = cell {
                if s == "push(1)" || s == "supernova" || s == "push(2)" {
                    found_genes += 1;
                }
            }
        }
    }
    assert!(found_genes > 0, "Supernova should scatter genes to grid");
}

#[test]
fn test_singularity() {
    let strand0 = Strand {
        genes: vec![Gene {
            op: OpCode::Singularity,
            args: vec![],
        }],
    };
    let strand1 = Strand {
        genes: vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        }],
    };

    let mut vm = ChimeraVM::new(make_dna(vec![strand0, strand1]));
    assert_eq!(vm.dna.helix.strands.len(), 2);

    vm.step(); // execute singularity

    assert_eq!(vm.dna.helix.strands.len(), 1);

    // The new strand should contain genes from both
    // singularity, push(42)
    let genes = &vm.dna.helix.strands[0].genes;
    assert_eq!(genes.len(), 2);
    assert_eq!(genes[0].op, OpCode::Singularity);
    assert_eq!(genes[1].op, OpCode::Push);

    // IP should be at (0, 1) because Singularity (at 0) was executed
    // and execution continues to next instruction.
    assert_eq!(vm.ip, (0, 1));
}
