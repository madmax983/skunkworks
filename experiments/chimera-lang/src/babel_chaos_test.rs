#![cfg(all(test, feature = "nova"))]

use crate::prelude::*;
use crate::vm::babel_chaos;

#[test]
fn test_babel_chaos_integrity() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);

    // Initial state
    assert_eq!(vm.babel_state.integrity, 1.0);

    // Apply chaos
    vm.stack.push(Value::Int(10)); // Degradation amount
    babel_chaos::exec_babel_chaos_op(&mut vm, OpCode::Glossolalia, &[]);
    assert!((vm.babel_state.integrity - 0.9).abs() < 0.001);

    // Apply clarity
    vm.stack.push(Value::Int(5));
    babel_chaos::exec_babel_chaos_op(&mut vm, OpCode::Clarify, &[]);
    assert!((vm.babel_state.integrity - 0.95).abs() < 0.001);
}

#[test]
fn test_babel_chaos_map() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);

    assert!(vm.babel_state.chaos_map.is_empty());

    babel_chaos::exec_babel_chaos_op(&mut vm, OpCode::Confuse, &[]);

    // Might be empty if Ops list is empty or rng unlucky, but unlikely with 10 iterations.
    // Ops list comes from OpCode::iter(), which should have many items.
    assert!(!vm.babel_state.chaos_map.is_empty());
}
