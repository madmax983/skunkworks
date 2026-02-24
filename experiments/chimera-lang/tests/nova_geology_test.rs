use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_vm_with_genes(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_quake() {
    // Fill grid column 0 with 1s
    let mut vm = make_vm_with_genes(vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // Intensity
        Gene {
            op: OpCode::Quake,
            args: vec![],
        },
    ]);

    // Setup initial state: Diagonal line
    for i in 0..16 {
        vm.grid[i][i] = Value::Int(1);
    }

    vm.step(); // Push
    vm.step(); // Quake

    // Check if grid changed (it's random, but likely with intensity 5)
    let mut changed = false;
    for i in 0..16 {
        if vm.grid[i][i] != Value::Int(1) {
            changed = true;
            break;
        }
    }
    // Note: there's a small chance it randomly shifts back or shifts empty rows/cols.
    // But with intensity 5, it should change.
    assert!(changed, "Quake should modify the grid");
}

#[test]
fn test_erode() {
    let mut vm = make_vm_with_genes(vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }, // Radius
        Gene {
            op: OpCode::Erode,
            args: vec![],
        },
    ]);

    // Set center to 10
    vm.grid[8][8] = Value::Int(10);
    // Set neighbor to 10
    vm.grid[8][9] = Value::Int(10);
    // Set far to 10
    vm.grid[0][0] = Value::Int(10);

    vm.step(); // Push
    vm.step(); // Erode

    // Center should be 9
    assert_eq!(vm.grid[8][8], Value::Int(9));
    assert_eq!(vm.grid[8][9], Value::Int(9));
    // Far should be 10 (untouched)
    assert_eq!(vm.grid[0][0], Value::Int(10));
}

#[test]
fn test_sediment() {
    let mut vm = make_vm_with_genes(vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(2)],
        }, // Radius
        Gene {
            op: OpCode::Sediment,
            args: vec![],
        },
    ]);

    vm.grid[8][8] = Value::Int(10);

    vm.step(); // Push
    vm.step(); // Sediment

    assert_eq!(vm.grid[8][8], Value::Int(11));
}

#[test]
fn test_tectonics() {
    // Stack expected: [dy, dx, h, w] (w is top)
    let mut vm = make_vm_with_genes(vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // dy
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        }, // dx
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        }, // h
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(4)],
        }, // w
        Gene {
            op: OpCode::Tectonics,
            args: vec![],
        },
    ]);

    // Plate center is 8,8. 4x4 around it.
    // 8,8 should move to 9,9.
    vm.grid[8][8] = Value::Int(99);

    for _ in 0..5 {
        vm.step();
    }

    assert_eq!(vm.grid[8][8], Value::Int(0)); // Moved
    assert_eq!(vm.grid[9][9], Value::Int(99)); // Arrived
}

#[test]
fn test_volcano() {
    let mut vm = make_vm_with_genes(vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        }, // Power
        Gene {
            op: OpCode::Volcano,
            args: vec![],
        },
    ]);

    vm.step();
    vm.step();

    assert_eq!(vm.grid[8][8], Value::Int(99));
    // Check neighbors have something (random > 50)
    // We can't deterministically check specific cells due to randomness,
    // but we can check if *some* cells changed.
    let mut lava_found = false;
    for y in 0..16 {
        for x in 0..16 {
            if y != 8 || x != 8 {
                if let Value::Int(v) = vm.grid[y][x] {
                    if v > 0 {
                        lava_found = true;
                    }
                }
            }
        }
    }
    assert!(lava_found, "Volcano should spew lava");
}
