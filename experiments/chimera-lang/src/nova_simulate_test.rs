#[cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        helix: Helix { strands },
    }
}

#[cfg(feature = "nova")]
#[test]
fn test_simulate_death() {
    // Strand 0: Main
    // [ push(10) push(1) simulate() ]
    // Expect: [ ..., Status(0), Energy, Result ]
    let main_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Target Strand (Poison)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // Ticks
            },
            Gene {
                op: OpCode::Simulate,
                args: vec![],
            },
        ],
    };

    // Strand 1: Poison
    // [ push(-1000) consume() ]
    // Dies immediately
    let poison_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-1000)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
        ],
    };

    let dna = make_dna(vec![main_strand, poison_strand]);
    let mut vm = ChimeraVM::new(dna);
    // Give initial energy so simulation cost doesn't kill main VM
    vm.energy = 200;

    vm.step(); // push(10)
    vm.step(); // push(1)
    vm.step(); // simulate()

    // Stack should have 3 values pushed by simulate
    // 1. Result (top of stack of sim, probably 0 or leftover from consume?)
    // Consume pops -1000. Stack empty. So 0.
    // 2. Final Energy (should be <= 0)
    // 3. Status (0 = Dead)
    // Pushed in order: Result, Energy, Status (top)

    assert!(vm.stack.len() >= 3);
    let status = vm.stack.pop().unwrap();
    let _energy = vm.stack.pop().unwrap();
    let _result = vm.stack.pop().unwrap();

    if let Value::Int(s) = status {
        assert_eq!(s, 0, "Expected Status 0 (Dead)");
    } else {
        panic!("Status not Int");
    }

    // Original VM should still be alive
    assert!(!vm.halted);
    assert!(vm.energy < 200); // Cost deducted
}

#[cfg(feature = "nova")]
#[test]
fn test_simulate_survival() {
    // Strand 0: Main
    // [ push(1) push(10) simulate() ]
    let main_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // Target Strand (Safe)
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // Ticks
            },
            Gene {
                op: OpCode::Simulate,
                args: vec![],
            },
        ],
    };

    // Strand 1: Safe
    // [ push(42) jump(1) ]
    let safe_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            },
        ],
    };

    let dna = make_dna(vec![main_strand, safe_strand]);
    let mut vm = ChimeraVM::new(dna);
    vm.metamorphism_enabled = false;
    vm.energy = 200;

    vm.step(); // push(10)
    vm.step(); // push(1)
    vm.step(); // simulate()

    // Stack: Result(42), Energy(>0), Status(1)
    let status = vm.stack.pop().unwrap();
    let _energy = vm.stack.pop().unwrap();
    let result = vm.stack.pop().unwrap();

    if let Value::Int(s) = status {
        assert_eq!(s, 1, "Expected Status 1 (Alive)");
    } else {
        panic!("Status not Int");
    }

    if let Value::Int(r) = result {
        assert_eq!(r, 42, "Expected Result 42");
    } else {
        panic!("Result not Int");
    }
}
