use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use crate::vm::{ChimeraVM, Value};
use crate::opcode::OpCode;

fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[test]
fn test_spawn_organelle() {
    // Strand 0: [ push(1) spawn() jump(2) ] -> Spawns Strand 1, then jumps to gene 2 (infinite wait loop to avoid executing Strand 1)
    // Actually simpler: [ push(1) spawn() ] then let it roll.

    // Strand 0: [ push(1) push(0) spawn() push(999) ]
    let strand0 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Type: Worker
            Gene { op: OpCode::Spawn, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(999)] }, // Just to keep main busy
        ],
    };

    // Strand 1: [ push(42) push(5) push(5) g_write() ]
    // Stack order: bottom [42, 5, 5] top.
    // g_write pops x (5), y (5), val (42).
    let strand1 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(42)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
            Gene { op: OpCode::GWrite, args: vec![] },
        ],
    };

    let dna = make_dna(vec![strand0, strand1]);
    let mut vm = ChimeraVM::new(dna);

    // Step 1: push(1)
    vm.step();

    // Step 2: push(0)
    vm.step();

    // Step 3: spawn()
    vm.step();

    // Verify spawn
    assert_eq!(vm.organelles.len(), 1, "Organelle should be spawned");
    // Organelle executes immediately in the same tick, so it has already executed instruction 0 (push(5))
    // IP should be at (1, 1)
    assert_eq!(vm.organelles[0].ip, (1, 1), "Organelle should have executed one step");

    // Run enough steps for Organelle to execute the rest of Strand 1
    // Needs 3 more steps.
    for _ in 0..5 {
        vm.step();
    }

    // Check Grid
    match vm.grid[5][5] {
        Value::Int(42) => (),
        _ => panic!("Expected 42 at (5,5), found {:?}", vm.grid[5][5]),
    }

    // Organelle should have finished and halted (removed from list)
    // Strand 1 has 4 genes. After 4 steps, it hits end.
    // My logic: if ip.1 >= strand_len, halted=true.
    // Next iteration filters it out.
    // So list should be empty eventually.

    // Run a bit more to clear it
    for _ in 0..5 {
        vm.step();
    }

    assert_eq!(vm.organelles.len(), 0, "Organelle should finish and be removed");
}

#[test]
fn test_spawn_energy_cost() {
    let strand0 = Strand {
        genes: vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Spawn, args: vec![] },
        ],
    };
    let strand1 = Strand { genes: vec![] }; // Empty target

    let dna = make_dna(vec![strand0, strand1]);
    let mut vm = ChimeraVM::new(dna);

    let initial_energy = vm.energy;
    vm.step(); // push
    vm.step(); // push
    vm.step(); // spawn

    // Cost of spawn is 20. Cost of steps is 1 each.
    // Total cost = 3 (Main metabolism) + 20 (Spawn cost) + 1 (Organelle metabolism for 1st step) = 24.
    assert_eq!(vm.energy, initial_energy - 24);
}
