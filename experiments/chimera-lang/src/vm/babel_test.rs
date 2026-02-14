#![cfg(test)]
#[cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
fn make_vm() -> ChimeraVM {
    let genes = vec![];
    let dna = Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[cfg(feature = "nova")]
#[test]
fn test_babel_compile_simple() {
    let mut vm = make_vm();

    // 1. Define Handler Strand (Index 1)
    // Handler just prints everything it sees.
    // Expects: [ ..., val1, val2, type, count ]
    // We want to verify it was called.
    // Let's make it print "Handler Called".
    // And drop the args to clean stack.
    let handler_genes = vec![
        Gene { op: OpCode::Drop, args: vec![] }, // Drop Count
        Gene { op: OpCode::Drop, args: vec![] }, // Drop Type
        Gene { op: OpCode::Print, args: vec![] }, // Print Val2
        Gene { op: OpCode::Print, args: vec![] }, // Print Val1
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("Handler Done".to_string())] },
        Gene { op: OpCode::Print, args: vec![] },
        Gene { op: OpCode::Ret, args: vec![] }, // Return to caller
    ];
    vm.dna.helix.strands.push(Strand { genes: handler_genes });
    let handler_idx = 1;

    // 2. Create CST: Junction(All, [Int(10), Int(20)])
    let cst = Value::Junction(
        crate::ast::JunctionType::All,
        vec![Value::Int(10), Value::Int(20)],
    );

    // 3. Setup Stack for BabelCompile
    // [ cst, handler_idx ]
    vm.stack.push(cst);
    vm.stack.push(Value::Int(handler_idx as i64));

    // 4. Execute BabelCompile
    crate::vm::babel::exec_babel_op(&mut vm, OpCode::BabelCompile, &[]);

    // 5. Verify New Strand Created
    assert_eq!(vm.stack.len(), 1);
    let new_strand_idx = match vm.stack.pop().unwrap() {
        Value::Int(i) => i as usize,
        _ => panic!("Expected strand index"),
    };
    assert_eq!(new_strand_idx, 2); // 0=Main, 1=Handler, 2=Compiled

    // 6. Execute New Strand
    // Manually jump to it
    vm.ip = (new_strand_idx, 0);
    // Step until halted or done (strand 2 has finite length)
    // We know exactly how many steps:
    // Push(10), Push(20), Push("All"), Push(2), Call(1) -> Handler
    // Handler: Drop, Drop, Print, Print, Push, Print -> 6 steps
    // Total approx 11 steps.
    for _ in 0..50 {
        vm.step();
        if vm.ip.0 > new_strand_idx && vm.call_stack.is_empty() {
            break;
        }
    }

    // 7. Verify Output
    println!("VM Output: {:?}", vm.output);
    // Handler should have printed:
    // 20
    // 10
    // "Handler Done"
    assert!(vm.output.contains(&"20".to_string()), "Output missing 20");
    assert!(vm.output.contains(&"10".to_string()), "Output missing 10");
    // Value::Str is printed with quotes
    assert!(vm.output.contains(&"\"Handler Done\"".to_string()), "Output missing Handler Done");
}

#[cfg(feature = "nova")]
#[test]
fn test_babel_compile_nested() {
    let mut vm = make_vm();

    // Handler: Simply drops 2 items (Type, Count)
    // We rely on leaves being pushed to stack.
    // [ ..., leaf, type, count ] -> Drop, Drop -> [ ..., leaf ]
    let handler_genes = vec![
        Gene { op: OpCode::Drop, args: vec![] }, // Drop Count
        Gene { op: OpCode::Drop, args: vec![] }, // Drop Type
        Gene { op: OpCode::Ret, args: vec![] },
    ];
    vm.dna.helix.strands.push(Strand { genes: handler_genes });
    let handler_idx = 1;

    // CST: [ [ 42 ] ] (Nested)
    let inner = Value::Junction(
        crate::ast::JunctionType::Any,
        vec![Value::Int(42)],
    );
    let outer = Value::Junction(
        crate::ast::JunctionType::All,
        vec![inner],
    );

    vm.stack.push(outer);
    vm.stack.push(Value::Int(handler_idx as i64));

    crate::vm::babel::exec_babel_op(&mut vm, OpCode::BabelCompile, &[]);
    let new_idx = match vm.stack.pop().unwrap() {
        Value::Int(i) => i as usize,
        _ => panic!("Expected index"),
    };

    vm.ip = (new_idx, 0);
    for _ in 0..50 {
        vm.step();
        if vm.ip.0 > new_idx { break; }
    }

    // Expected Stack state:
    // 1. Push 42
    // 2. Push "Any", Push 1, Call Handler (Drops 2) -> Stack: [42]
    // 3. Push "All", Push 1, Call Handler (Drops 2) -> Stack: [42]
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], Value::Int(42));
}
