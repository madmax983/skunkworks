#![cfg(feature = "nova")]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};

fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

fn fill_grid(vm: &mut ChimeraVM, val: i64) {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            vm.grid[y][x] = Value::Int(val);
        }
    }
}

#[test]
fn test_rain() {
    // [ push(10) push(2) rain() ] -> Rain intensity 10 radius 2
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] }, // intensity
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },  // radius
        Gene { op: OpCode::Rain, args: vec![] },
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (8, 8); // Center

    vm.step(); // push
    vm.step(); // push
    vm.step(); // rain

    // Radius 2 circle should have moisture
    // Center
    assert_eq!(vm.moisture_grid[8][8], 10);
    // Edge (8, 9)
    assert_eq!(vm.moisture_grid[8][9], 10);
    // Outside (8, 11)
    assert_eq!(vm.moisture_grid[8][11], 0);
}

#[test]
fn test_flow() {
    // Setup slope: (8,8)=100, (8,9)=50
    // Add moisture to (8,8)
    // Flow()
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // iterations
        Gene { op: OpCode::Flow, args: vec![] },
    ];
    let mut vm = make_vm(genes);

    // Fill grid with 100 to prevent flow to default 0s
    fill_grid(&mut vm, 100);

    vm.grid[8][8] = Value::Int(100);
    vm.grid[8][9] = Value::Int(50); // Lowest neighbor
    vm.moisture_grid[8][8] = 100;

    vm.step(); // push
    vm.step(); // flow

    // Moisture should move from 8,8 to 8,9 (lowest neighbor)
    let m_source = vm.moisture_grid[8][8];
    let m_target = vm.moisture_grid[8][9];

    assert!(m_source < 100, "Moisture should leave source");
    assert!(m_target > 0, "Moisture should reach target");

    // Erosion/Deposition
    let h_source = match vm.grid[8][8] { Value::Int(h) => h, _ => 0 };
    let h_target = match vm.grid[8][9] { Value::Int(h) => h, _ => 0 };

    assert!(h_source < 100, "Source should erode");
    assert!(h_target > 50, "Target should receive sediment");
}

#[test]
fn test_river() {
    // [ push(5) push(10) river() ] -> Length 10, Width 5
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] }, // length
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },  // width
        Gene { op: OpCode::River, args: vec![] },
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (5, 5);

    // Fill grid with very high value to create a canyon wall
    fill_grid(&mut vm, 200);

    // Set a slope to guide river: 5,5 high -> 5,15 low
    // Make slope steep (5 per step) to prevent backflow from erosion
    for x in 0..16 {
        vm.grid[5][x] = Value::Int(100 - (x as i64 * 5));
    }

    vm.step(); // push
    vm.step(); // push
    vm.step(); // river

    // Check start point modified
    if let Value::Int(h) = vm.grid[5][5] {
        assert!(h < 75, "River start should be carved deep (Started at 75)");
    }
    assert!(vm.moisture_grid[5][5] > 0, "River should have water");

    // Check downstream
    // It should have moved roughly towards increasing x (lower height)
    let mut wet_cells = 0;
    for x in 5..16 {
        if vm.moisture_grid[5][x] > 0 {
            wet_cells += 1;
        }
    }

    // Debug info
    if wet_cells <= 1 {
        println!("Grid Row 5 Heights:");
        for x in 0..16 {
            print!("{} ", match vm.grid[5][x] { Value::Int(h) => h, _ => 0 });
        }
        println!("\nMoisture Row 5:");
        for x in 0..16 {
            print!("{} ", vm.moisture_grid[5][x]);
        }
        println!("\nVM Output: {:?}", vm.output);
    }

    assert!(wet_cells > 1, "River should flow downstream (at least 1 step)");
}

#[test]
fn test_aquifer() {
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
        Gene { op: OpCode::Aquifer, args: vec![] }, // Pump
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(-20)] },
        Gene { op: OpCode::Aquifer, args: vec![] }, // Drain
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (0, 0);

    vm.step(); // push
    vm.step(); // pump
    assert_eq!(vm.moisture_grid[0][0], 50);

    vm.step(); // push
    vm.step(); // drain
    assert_eq!(vm.moisture_grid[0][0], 30);

    // Check stack for drained amount
    if let Value::Int(drained) = vm.stack.last().unwrap() {
        assert_eq!(*drained, 20);
    } else {
        panic!("Aquifer drain should return amount");
    }
}

#[test]
fn test_sense_flow() {
    // Slope (0,0)=10 -> (0,1)=0
    let genes = vec![
        Gene { op: OpCode::SenseFlow, args: vec![] },
    ];
    let mut vm = make_vm(genes);
    vm.context_loc = (0, 0);

    fill_grid(&mut vm, 10); // Fill with 10

    vm.grid[0][0] = Value::Int(10);
    vm.grid[0][1] = Value::Int(0); // East is lower

    vm.step();

    // Stack should have dy, dx
    let dx = vm.stack.pop().unwrap();
    let dy = vm.stack.pop().unwrap();

    assert_eq!(dx, Value::Int(1)); // East
    assert_eq!(dy, Value::Int(0));
}
