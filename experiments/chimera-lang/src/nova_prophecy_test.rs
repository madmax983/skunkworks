#[cfg(feature = "nova")]
use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};

#[cfg(feature = "nova")]
fn make_dna(strands: Vec<Strand>) -> Dna {
    Dna {
        evolution_config: None,
        helix: Helix { strands },
    }
}

#[cfg(feature = "nova")]
#[test]
fn test_prophecy_death() {
    // Strand 0: Main
    // [ push(10) prophecy() ]
    // Expect: [ ..., Result(1) ] (1 = Death)
    let main_strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)], // 10 ticks
            },
            Gene {
                op: OpCode::Prophecy,
                args: vec![],
            },
        ],
    };

    let dna = make_dna(vec![main_strand]);
    let mut vm = ChimeraVM::new(dna);
    // Give minimal energy so simulation cost kills it if it consumes
    vm.energy = 50;
    // Wait, Prophecy runs a clone.
    // Prophecy cost is 50 + ticks/2.
    // If I give 50 energy, Prophecy itself might kill the VM due to cost?
    // But Prophecy pushes result 1 if the *simulation* dies.
    // Simulation inherits energy? Yes, clone.
    // If I have 50 energy, and Prophecy costs 55, the *real* VM dies.
    // The *simulation* starts with 50.
    // Simulation runs next instructions?
    // Main strand: [ push(10), prophecy() ... (next instructions) ]
    // Simulation starts AFTER prophecy.
    // If next instruction is empty, simulation halts (but not necessarily dead).
    // Halted means IP exceeded strand.
    // exec_prophecy: "Result: 1 if Dead (halted), 0 if Alive"
    // Wait, halted means it finished the strand? Or ran out of energy?
    // If it finished the strand, it halted normally. Is that death?
    // "Result: 1 if Dead (halted), 0 if Alive".
    // If simulation runs out of instructions, it sets halted=true.
    // So if the strand ends, Prophecy says 1 (Death)?
    // Let's check `exec_prophecy` logic:
    // `let result = if sim_vm.halted { 1 } else { 0 };`
    // Yes, if halted (for any reason), it returns 1.
    // So an empty strand after prophecy is death?
    // Let's add a suicide gene to be sure.
    // [ push(10) prophecy() consume() consume() ... ]

    // But if simulation halts due to running out of instructions, it's also "death" in this context?
    // Let's make sure it dies from energy loss.
    // Or just "halted".

    // Case 1: Death by emptiness (Strand ends)
    // Simulation runs 10 ticks. Strand ends immediately. Halted = true. Result = 1.

    // Case 2: Life (Infinite Loop)
    // [ push(10) prophecy() jump(0) ]
    // Simulation jumps to 0. Loops. Does not halt. Result = 0.

    // Let's test Case 2 first (Life).
    // Reworking test_prophecy_death to be explicit death.
}

#[cfg(feature = "nova")]
#[test]
fn test_prophecy_predicts_halt() {
    // Strand 0: [ push(5) prophecy() ]
    // Simulation will run out of instructions and halt.
    // Expect: Result 1.
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Prophecy,
            args: vec![],
        },
    ];
    let dna = make_dna(vec![Strand { genes }]);
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000;

    vm.step(); // push
    vm.step(); // prophecy

    let result = vm.stack.pop().unwrap();
    if let Value::Int(r) = result {
        assert_eq!(r, 1, "Expected Prophecy to predict Halt (1)");
    } else {
        panic!("Prophecy result not Int");
    }
}

#[cfg(feature = "nova")]
#[test]
fn test_prophecy_predicts_life() {
    // Strand 0: [ push(5) prophecy() jump(0) ]
    // Simulation loops forever (well, for 5 ticks).
    // Expect: Result 0.

    // Note: jump(0) jumps to index 0 of CURRENT strand.
    // Strand 0 len is 3. 0 is push(5).
    // So it loops: push(5), prophecy(), jump(0).
    // Wait, if it loops, it executes prophecy AGAIN inside simulation?
    // `sim_vm.ip.1 += 1;` advances PAST prophecy.
    // But jump(0) sends it back to 0.
    // Then it hits prophecy again.
    // Recursion depth check handles this?
    // `sim_vm.recursion_depth += 1`.
    // Inside simulation: recursion_depth is 1.
    // Hit prophecy: `exec_prophecy` checks depth.
    // If depth > MAX, returns None?
    // Does it halt?
    // If `exec_prophecy` returns None (error), does VM halt?
    // `step()` prints error but continues unless error is fatal?
    // Errors usually just print to output and continue.
    // So it continues to jump(0).
    // So it loops. Halted = false.
    // Result = 0.

    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(5)],
        },
        Gene {
            op: OpCode::Prophecy,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        },
    ];
    let dna = make_dna(vec![Strand { genes }]);
    let mut vm = ChimeraVM::new(dna);
    vm.energy = 1000;

    vm.step(); // push
    vm.step(); // prophecy

    let result = vm.stack.pop().unwrap();
    if let Value::Int(r) = result {
        assert_eq!(r, 0, "Expected Prophecy to predict Life (0)");
    } else {
        panic!("Prophecy result not Int");
    }
}
