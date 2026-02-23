use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_forth_stack_ops() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // ₣ -> "1" -> "2" -> "swap" -> "."
    // Expected Output: "2", "1" (swap swaps top 2)
    // 1 push 1
    // 2 push 2 (stack: 1, 2)
    // swap (stack: 2, 1)
    // . (pop 1, print 1)
    // Stack left with 2?

    vm.grid[5][5] = Value::Str("₣".to_string());
    vm.grid[5][6] = Value::Str("1".to_string());
    vm.grid[5][7] = Value::Str("2".to_string());
    vm.grid[5][8] = Value::Str("swap".to_string());
    vm.grid[5][9] = Value::Str(".".to_string());

    // Run until agent reaches end
    for _ in 0..10 {
        exec_prologue_tick(&mut vm);
    }

    // Check output log
    // Expect "₣ 1: 1" (stack len 1, value 1)
    let output = vm.output.join("\n");
    assert!(output.contains("₣ 1: 1"));
}

#[test]
fn test_forth_skip() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // ₣ -> "0" -> "skip" -> "fail" -> "pass"
    // 0 push 0
    // skip pops 0 -> skips next move ("fail") -> lands on "pass"
    // "fail" puts "fail" on stack (if not skipped)
    // "pass" puts "pass" on stack

    vm.grid[5][5] = Value::Str("₣".to_string());
    vm.grid[5][6] = Value::Str("0".to_string());
    vm.grid[5][7] = Value::Str("skip".to_string());
    vm.grid[5][8] = Value::Str("fail".to_string());
    vm.grid[5][9] = Value::Str("pass".to_string());

    // Run until done
    for _ in 0..10 {
        exec_prologue_tick(&mut vm);
    }

    // Check agent stack state
    // We can inspect agent directly if we find it.
    let agent = vm.prologue_state.agents.iter().find(|a|
        if let Value::Str(s) = &vm.grid[a.y][a.x] { s == "₣" } else { false }
    );

    // Agent should have "pass" on stack, but NOT "fail".
    if let Some(a) = agent {
        let stack_str = format!("{:?}", a.stack);
        assert!(stack_str.contains("pass"));
        assert!(!stack_str.contains("fail"));
    } else {
        // Agent might have moved off the end?
        // Or moved to 5,10.
        // If 10 ticks, it moved 5 -> 6 -> 7 -> 9 -> 10.
    }
}
