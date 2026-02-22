#![cfg(all(feature = "nova", feature = "oracle"))]

use chimera_lang::ast::{Dna, Helix, JunctionType, Strand};
use chimera_lang::vm::nova_signals::process_signals;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.orca_mode = true;
    vm
}

#[test]
fn test_logic_pi_assert() {
    let mut vm = make_vm();

    // Setup Π (Define) at (5, 5)
    // N: "likes"
    // E: "cat"
    // W: "milk"

    vm.grid[5][5] = Value::Str("Π".to_string());
    vm.grid[4][5] = Value::Str("likes".to_string()); // Predicate
    vm.grid[5][6] = Value::Str("cat".to_string()); // Subject
    vm.grid[5][4] = Value::Str("milk".to_string()); // Object

    // Fire the signal at (5,5)
    vm.signal_grid[5][5] = 1;

    // Process
    process_signals(&mut vm);

    // Check KB
    // Expected: Junction(Any, ["likes", "cat", "milk"])
    // Note: The implementation of Π usually structures facts.
    // If we implement it as [Pred, Subj, Obj], it should be there.

    let expected = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("likes".to_string()),
            Value::Str("cat".to_string()),
            Value::Str("milk".to_string()),
        ],
    );

    assert!(
        vm.knowledge_base.contains(&expected),
        "KB should contain the fact"
    );
}

#[test]
fn test_logic_lambda_query() {
    let mut vm = make_vm();

    // Pre-populate KB with likes(cat, milk)
    let fact = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("likes".to_string()),
            Value::Str("cat".to_string()),
            Value::Str("milk".to_string()),
        ],
    );
    vm.knowledge_base.push(fact);

    // Setup λ (Query) at (5, 5)
    // N: "likes"
    // E: "cat"
    // W: "milk"

    vm.grid[5][5] = Value::Str("λ".to_string());
    vm.grid[4][5] = Value::Str("likes".to_string()); // Predicate
    vm.grid[5][6] = Value::Str("cat".to_string()); // Subject
    vm.grid[5][4] = Value::Str("milk".to_string()); // Object

    // Fire signal
    vm.signal_grid[5][5] = 1;

    // Process
    process_signals(&mut vm);

    // Check Output at South (6, 5)
    let result = &vm.grid[6][5];

    // Should be "1" (True)
    match result {
        Value::Str(s) => assert_eq!(s, "1"),
        Value::Int(n) => assert_eq!(*n, 1),
        _ => panic!("Expected '1' output, got {:?}", result),
    }
}

#[test]
fn test_logic_lambda_query_fail() {
    let mut vm = make_vm();

    // KB is empty

    // Setup λ (Query) at (5, 5)
    // N: "likes"
    // E: "dog"
    // W: "sushi"

    vm.grid[5][5] = Value::Str("λ".to_string());
    vm.grid[4][5] = Value::Str("likes".to_string());
    vm.grid[5][6] = Value::Str("dog".to_string());
    vm.grid[5][4] = Value::Str("sushi".to_string());

    vm.signal_grid[5][5] = 1;

    process_signals(&mut vm);

    // Check Output at South (6, 5)
    let result = &vm.grid[6][5];

    // Should be "0" (False) or unchanged (if we only write on success? Usually standard is 0/1)
    // Let's assume we implement it to write 0 on fail.
    match result {
        Value::Str(s) => assert_eq!(s, "0"),
        Value::Int(n) => assert_eq!(*n, 0),
        _ => panic!("Expected '0' output, got {:?}", result),
    }
}
