#[cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::ChimeraVM;

#[cfg(feature = "nova")]
fn make_vm_with_genes(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000; // ample energy
    vm
}

#[cfg(feature = "nova")]
#[test]
fn test_ignite() {
    // [ push(50) push(2) ignite() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Ignite,
            args: vec![],
        },
    ];
    let mut vm = make_vm_with_genes(genes);
    vm.context_loc = (8, 8);

    vm.step(); // push
    vm.step(); // push
    vm.step(); // ignite

    // process_environment runs BEFORE ignite. So heat should be exactly 50.
    assert_eq!(vm.heat_grid[8][8], 50);
    // Radius 2 check
    assert_eq!(vm.heat_grid[8][10], 50); // (8, 8+2)
    assert_eq!(vm.heat_grid[8][11], 0); // (8, 8+3) outside
}

#[cfg(feature = "nova")]
#[test]
fn test_freeze() {
    // [ push(50) push(2) ignite() push(20) push(2) freeze() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(50)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Ignite,
            args: vec![],
        },
        // Immediate freeze next step
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(20)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        },
        Gene {
            op: OpCode::Freeze,
            args: vec![],
        },
    ];
    let mut vm = make_vm_with_genes(genes);
    vm.context_loc = (8, 8);

    vm.step(); // push
    vm.step(); // push
    vm.step(); // ignite (Heat -> 50)

    vm.step(); // push (Env runs: 50 diffuses. Center < 50)
    vm.step(); // push (Env runs: Center < prev)
    vm.step(); // freeze (Env runs. Freeze removes 20)

    // Heat should be > 0 but < 30 (due to diffusion)
    let heat = vm.heat_grid[8][8];
    println!("Heat after freeze: {}", heat);
    assert!(heat < 30);
    assert!(heat >= 0);
}

#[cfg(feature = "nova")]
#[test]
fn test_diffusion() {
    // [ push(100) push(0) ignite() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(100)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Ignite,
            args: vec![],
        },
    ];
    let mut vm = make_vm_with_genes(genes);
    vm.context_loc = (8, 8);

    vm.step();
    vm.step();
    vm.step();

    // Ignite ran.
    assert_eq!(vm.heat_grid[8][8], 100);
    assert_eq!(vm.heat_grid[8][9], 0);

    // Run one step to trigger diffusion
    vm.dna.helix.strands[0].genes = vec![Gene {
        op: OpCode::Drop,
        args: vec![],
    }];
    vm.ip = (0, 0);

    vm.step(); // Diffusion tick

    let center = vm.heat_grid[8][8];
    let neighbor = vm.heat_grid[8][9];

    println!("Tick 1: Center {}, Neighbor {}", center, neighbor);
    // Center should decay/diffuse
    assert!(center < 100);
    // Neighbor should receive heat
    assert!(neighbor > 0);
}

#[cfg(feature = "nova")]
#[test]
fn test_thermal_mutation() {
    // [ push(200) push(0) ignite() ... loop ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(200)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Ignite,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let mut vm = make_vm_with_genes(genes.clone());
    vm.context_loc = (8, 8);

    // Initial state
    vm.step();
    vm.step();
    vm.step(); // Ignite -> 200

    // We need to keep heat high for mutation to happen.
    // But heat diffuses rapidly!
    // Fix: Ignite massive heat and Re-ignite every loop.

    let genes_loop = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1000)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Ignite,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    vm = make_vm_with_genes(genes_loop);
    vm.context_loc = (8, 8);

    let initial_genes = vm.dna.helix.strands[0].genes.clone();
    let mut mutated = false;
    for _ in 0..1000 {
        vm.step();

        // Check if ANY gene changed
        if vm.dna.helix.strands[0].genes != initial_genes {
            mutated = true;
            break;
        }
    }

    assert!(mutated, "High heat (sustained) failed to trigger mutation");
}
