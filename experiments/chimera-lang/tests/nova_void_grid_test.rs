use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix { strands: vec![Strand { genes }] },
    }
}

#[test]
fn test_void_scribe_and_read() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("RUNE".to_string())] },
        Gene { op: OpCode::Scribe, args: vec![] },
        Gene { op: OpCode::ReadVoid, args: vec![] },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));
    vm.energy = 100; // Ensure enough energy

    vm.step(); // Push
    vm.step(); // Scribe

    // Check internal state
    let (cy, cx) = vm.context_loc;
    assert_eq!(vm.void_grid[cy][cx], Some("RUNE".to_string()));

    vm.step(); // ReadVoid

    assert_eq!(vm.stack.last(), Some(&Value::Str("RUNE".to_string())));
}

#[test]
fn test_spell_firestorm() {
    // Scribe "Fire" at North (cy-1, cx)
    // Scribe "Storm" at East (cy, cx+1)
    // "FireStorm" triggers OpCode::Storm

    let genes = vec![
        // North (-1, 0)
        // Stack order for Migrate: [dy, dx (top)]
        // Push dy (-1), Push dx (0)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] }, // dy
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dx
        Gene { op: OpCode::Migrate, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Fire".to_string())] },
        Gene { op: OpCode::Scribe, args: vec![] },

        // South (1, 0) - Back to Center
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dy
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dx
        Gene { op: OpCode::Migrate, args: vec![] },

        // East (0, 1)
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dy
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // dx
        Gene { op: OpCode::Migrate, args: vec![] },

        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Storm".to_string())] },
        Gene { op: OpCode::Scribe, args: vec![] },

        // West (0, -1) - Back to Center
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // dy
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-1)] }, // dx
        Gene { op: OpCode::Migrate, args: vec![] },

        // Cast
        Gene { op: OpCode::Spell, args: vec![] },
    ];

    let mut vm = ChimeraVM::new(make_dna(genes));

    // Give enough energy
    vm.energy = 1000;

    // Run loop
    // 1. Push 0
    // 2. Push -1
    // 3. Migrate
    // 4. Push Fire
    // 5. Scribe
    // ...
    // Total genes: 3*4 + 1 = 13.
    // Plus output processing etc.

    for _ in 0..50 {
        vm.step();
    }

    // Check Output for "SPELL: Cast 'Firestorm'"
    let found = vm.output.iter().any(|s| s.contains("SPELL: Cast 'Firestorm'"));

    if !found {
        println!("VM Output:");
        for line in &vm.output {
            println!("{}", line);
        }
    }

    assert!(found, "Spell should have cast Firestorm");
}
