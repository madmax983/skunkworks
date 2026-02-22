use chimera_lang::ast::{Dna, Helix};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::nova::exec_nova_op;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_superpose_depth_limit_enforced() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);

    let mut deep = Value::Int(0);
    for _ in 0..100 {
        deep = Value::Junction(chimera_lang::ast::JunctionType::Any, vec![deep]);
    }

    vm.stack.push(deep.clone());
    vm.stack.push(deep);

    exec_nova_op(&mut vm, OpCode::Superpose, &[]);

    // Debugging
    println!("Stack len: {}", vm.stack.len());
    println!("Output: {:?}", vm.output);

    // If check passed (bug), stack len is 1.
    // If check failed (correct), stack len is 0.
    if vm.stack.len() == 1 {
        panic!(
            "Superpose allowed deep value creation! Stack depth: {}",
            vm.stack[0].depth()
        );
    }

    let output = vm.output.last().cloned().unwrap_or_default();
    assert!(
        output.contains("depth limit exceeded"),
        "Expected depth limit error, got: '{}'",
        output
    );
}
