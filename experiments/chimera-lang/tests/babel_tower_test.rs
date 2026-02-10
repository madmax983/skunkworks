use chimera_lang::prelude::*;
use chimera_lang::opcode::OpCode;
use chimera_lang::ast::{Gene, Nucleotide, Strand, Dna, Helix};
use chimera_lang::vm::Value;

#[test]
fn test_babel_tower_arithmetic_inversion() {
    // Strand 0: Setup Tower at (8,8)
    // 1. Move to (8,8) - Start context is (8,8) by default in new()
    // 2. Babel(5) - Radius 5
    // 3. Tongue("Sub", "Add") - Map Add -> Sub
    // Note: Tongue pops [to, from]. So we push "Add" (from) then "Sub" (to).
    // Wait, my implementation:
    // let to_val = pop() // Top
    // let from_val = pop() // Second
    // So if stack is [ "Add", "Sub" ], to="Sub", from="Add".
    // Map is insert(from, to). So Add -> Sub. Correct.

    let setup_genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Babel, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("add".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("sub".to_string())] },
        Gene { op: OpCode::Tongue, args: vec![] },
    ];

    // Strand 1: Test Subject at (8,8)
    // Push(10) Push(5) Add
    // Should result in 5 (Sub) instead of 15 (Add)
    let test_genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Add, args: vec![] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![
                Strand { genes: setup_genes },
                Strand { genes: test_genes },
            ],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Step Strand 0 (Setup)
    // 5 genes.
    for _ in 0..5 {
        vm.step();
    }

    // Verify Tower exists
    assert!(vm.towers.contains_key(&(8, 8)));
    let tower = vm.towers.get(&(8, 8)).unwrap();
    assert_eq!(tower.radius, 5);
    assert_eq!(tower.dialect.get(&OpCode::Add), Some(&OpCode::Sub));

    // Step Strand 1 (Test)
    // Switch to strand 1 manually or let loop continue?
    // VM steps strand 0 then strand 1 if both have genes.
    // Strand 0 is done (5 genes executed). IP -> (0, 5).
    // Next step will try strand 0, find it done, increment strand -> 1.

    // We need to ensure Strand 1 executes at (8,8).
    // vm.context_loc defaults to (8,8).
    // execute_gene uses context_loc.
    // So it should work.

    // Step 1: Switch to Strand 1 (handled by VM loop logic usually, but let's manual step)
    // Current IP (0, 5).
    // Step -> (1, 0)
    vm.step(); // IP moves to (1,0) effectively
    assert_eq!(vm.ip, (1, 0));

    // Execute Push(10)
    vm.step();
    // Execute Push(5)
    vm.step();
    // Execute Add (mapped to Sub)
    vm.step();

    // Check Stack
    assert_eq!(vm.stack.len(), 1);
    match vm.stack[0] {
        Value::Int(n) => assert_eq!(n, 5, "Expected 10 - 5 = 5, got {}", n),
        _ => panic!("Expected Int"),
    }
}

#[test]
fn test_babel_tower_range() {
    // Setup Tower at (8,8) radius 1.
    // Test at (8,8) -> Affected
    // Test at (8,10) -> Unaffected (Dist 2 > 1)

    let setup_genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Radius 1
        Gene { op: OpCode::Babel, args: vec![] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("add".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("sub".to_string())] },
        Gene { op: OpCode::Tongue, args: vec![] },
    ];

    let test_genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(5)] },
        Gene { op: OpCode::Add, args: vec![] },
    ];

    let dna = Dna {
        helix: Helix {
            strands: vec![
                Strand { genes: setup_genes },
                Strand { genes: test_genes.clone() },
                Strand { genes: test_genes },
            ],
        },
    };

    let mut vm = ChimeraVM::new(dna);

    // Run Setup
    for _ in 0..5 {
        vm.step();
    }

    // Strand 1: At (8,8)
    vm.context_loc = (8, 8);
    // Move IP to Strand 1
    vm.ip = (1, 0);

    vm.step(); vm.step(); vm.step();
    assert_eq!(vm.stack.pop(), Some(Value::Int(5))); // Affected

    // Strand 2: At (8,10)
    vm.context_loc = (8, 10);
    vm.ip = (2, 0);

    vm.step(); vm.step(); vm.step();
    assert_eq!(vm.stack.pop(), Some(Value::Int(15))); // Unaffected
}
