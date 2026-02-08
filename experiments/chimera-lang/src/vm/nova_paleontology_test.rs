#![cfg(feature = "nova")]

use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};

fn make_dna(genes: Vec<Gene>) -> Dna {
    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[test]
fn test_fossilize_and_unearth() {
    // 1. Create a strand that will be fossilized
    // Strand 0: [ push(0) fossilize() ]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Fossilize,
            args: vec![],
        },
    ];
    let mut vm = ChimeraVM::new(make_dna(genes));

    // Execute Fossilize
    vm.step(); // push(0) - Strand Index to fossilize
    vm.step(); // fossilize() - Pops 0

    // Check grid at context_loc (default 8,8)
    let (cy, cx) = vm.context_loc;
    if let Value::Str(s) = &vm.grid[cy][cx] {
        assert!(s.starts_with("Fossil:"));
        println!("Fossil created: {}", s);
    } else {
        panic!("Grid cell not a string");
    }

    // 2. Unearth it
    // We manually clear the stack and try to unearth
    vm.stack.clear();
    // Stack for Unearth: y, x
    vm.stack.push(Value::Int(cy as i64));
    vm.stack.push(Value::Int(cx as i64));

    // We can't easily inject instructions without modifying DNA or using Virus.
    // Let's just call the internal function or assume we have an "Archeologist" strand.
    // Or we can manually execute the OpCode using a mock environment?
    // Easier: Just modify the VM DNA to include a new strand for unearthing.

    // But since we are inside the test module which is part of the crate, we can call exec function?
    // No, exec functions are in nova_paleontology which is private/pub restricted?
    // It's pub mod in vm, so crate::vm::nova_paleontology::exec_unearth is visible if pub.
    // Yes, I made them pub.

    crate::vm::nova_paleontology::exec_unearth(&mut vm);

    // Check results
    // Stack should have new_strand_idx
    assert_eq!(vm.stack.len(), 1);
    let new_idx = if let Value::Int(i) = vm.stack[0] { i } else { -1 };
    assert!(new_idx > 0);

    // Verify the new strand matches the old one
    let recovered = &vm.dna.helix.strands[new_idx as usize];
    assert_eq!(recovered.genes.len(), 2);
    assert_eq!(recovered.genes[0].op, OpCode::Push);
    // Note: The second gene was Fossilize. The serialization captures the *static* DNA.
}

#[test]
fn test_carbon_date() {
    let genes = vec![];
    let mut vm = ChimeraVM::new(make_dna(genes));
    let (cy, cx) = vm.context_loc;

    // Manually place a fossil from tick 10
    vm.grid[cy][cx] = Value::Str("Fossil:10:hash:dna".to_string());

    // Advance time to 100
    vm.tick_counter = 100;

    // Stack: y, x
    vm.stack.push(Value::Int(cy as i64));
    vm.stack.push(Value::Int(cx as i64));

    crate::vm::nova_paleontology::exec_carbon_date(&mut vm);

    if let Some(Value::Int(age)) = vm.stack.pop() {
        assert_eq!(age, 90);
    } else {
        panic!("Expected integer age");
    }
}
