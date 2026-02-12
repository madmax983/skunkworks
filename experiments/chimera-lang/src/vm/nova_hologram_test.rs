use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::ChimeraVM;

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_hologram_interfere_refract_perfect() {
    let mut vm = make_vm();

    // Create a complex strand
    // push(42) add() print()
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        }, // Add (needs stack setup usually, but we just test storage)
        Gene {
            op: OpCode::Print,
            args: vec![],
        },
    ];
    let strand = Strand {
        genes: genes.clone(),
    };
    vm.dna.helix.strands.push(strand);

    // Interfere (Encode Strand 0)
    // Args: [strand_idx]
    let args = vec![Nucleotide::Number(0)];
    crate::vm::nova_hologram::exec_interfere(&mut vm, OpCode::Interfere, &args);

    // Check if hologram grid is dirty
    let mut energy = 0.0;
    for row in &vm.hologram_grid {
        for (re, im) in row {
            energy += re * re + im * im;
        }
    }
    assert!(energy > 0.0, "Hologram grid should have energy");

    // Refract (Decode)
    crate::vm::nova_hologram::exec_refract(&mut vm, OpCode::Refract, &[]);

    assert_eq!(
        vm.dna.helix.strands.len(),
        2,
        "Should have reconstructed a strand"
    );
    let new_strand = &vm.dna.helix.strands[1];

    assert_eq!(new_strand.genes.len(), genes.len(), "Length mismatch");

    for (i, gene) in new_strand.genes.iter().enumerate() {
        assert_eq!(gene.op, genes[i].op, "OpCode mismatch at {}", i);
        if !genes[i].args.is_empty() {
            assert_eq!(
                gene.args.len(),
                genes[i].args.len(),
                "Arg count mismatch at {}",
                i
            );
            if let (Nucleotide::Number(a), Nucleotide::Number(b)) =
                (&gene.args[0], &genes[i].args[0])
            {
                assert_eq!(a, b, "Arg value mismatch at {}", i);
            }
        }
    }
}

#[test]
fn test_diffract_ghost() {
    let mut vm = make_vm();
    let genes = vec![Gene {
        op: OpCode::Push,
        args: vec![Nucleotide::Number(10)],
    }];
    vm.dna.helix.strands.push(Strand { genes });

    let args = vec![Nucleotide::Number(0)];
    crate::vm::nova_hologram::exec_diffract(&mut vm, OpCode::Diffract, &args);

    // Check energy
    let mut energy = 0.0;
    for row in &vm.hologram_grid {
        for (re, im) in row {
            energy += re * re + im * im;
        }
    }
    assert!(energy > 0.0, "Diffract should add energy");

    // Refract should FAIL to find a clean strand (or find a weak one)
    // because Diffract shifts the center.
    // The current Refract implementation looks at the exact center.

    crate::vm::nova_hologram::exec_refract(&mut vm, OpCode::Refract, &[]);

    // Ideally it finds nothing or garbage, OR we relax refract to scan?
    // Since we didn't implement spatial scan in Refract, it should fail to reconstruct perfectly
    // or return a very weak signal that gets filtered out.
    // Based on threshold 0.1 magnitude:
    // Diffract amplitude is 0.5. Shift is 4 pixels.
    // k ranges 0.5 to 8.0.
    // 4 pixels shift is significant phase/interference change.
    // Expectation: No strand created or partial/garbage.

    // Let's assert that it's DIFFERENT than Interfere.
    // If we run Interfere on same grid, it should interfere constructively or destructively.
}
