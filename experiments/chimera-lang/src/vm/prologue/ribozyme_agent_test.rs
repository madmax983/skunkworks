use super::super::*;
use crate::ast::{Dna, Helix, JunctionType};
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_ribozyme_agent() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Grid
    // Ribozyme at (5,5)
    vm.grid[5][5] = Value::Str("🛠".to_string());

    // Input West (5,4): 10
    vm.grid[5][4] = Value::Int(10);

    // Enzyme North (4,5): [ "push(2)", "mul" ]
    // We construct the list of OpCodes.
    // The Ribozyme expects a Junction of Strings or Strings directly.
    // Let's use a Junction of Strings.
    let enzyme = Value::Junction(
        JunctionType::Any,
        vec![
            Value::Str("push".to_string()),
            Value::Str("2".to_string()), // Push takes arg from next gene usually, but in raw execution we need to push args manually?
            // Wait, execute_gene_inner handles args if they are passed.
            // But here we are executing opcodes one by one.
            // If we use "push", it expects args.
            // In `ribozyme.rs`:
            // if let Ok(op) = s.parse::<OpCode>() { genes.push(Gene { op, args: vec![] }) }
            // OpCode::Push with empty args usually does nothing or fails if it expects args.

            // Wait, `exec_stack_op` checks args.
            // "Error: Invalid arg for push: None" if empty.

            // So we cannot use `Push` easily via Ribozyme unless we support args parsing.
            // BUT Ribozyme parses strings to OpCodes.
            // It doesn't parse arguments.

            // However, we can use stack manipulation!
            // Input is 10 (on stack).
            // We want to multiply by 2.
            // We need to push 2.
            // How do we push 2 without arguments?
            // We can't with standard Push opcode and empty args.

            // Maybe we can use `Dup` (10, 10) and `Add` (20)?
            // That works! 10 * 2 = 10 + 10.
            Value::Str("dup".to_string()),
            Value::Str("add".to_string()),
        ],
    );
    vm.grid[4][5] = enzyme;

    // 2. Run Tick
    // This calls process_agents -> process_ribozyme_agent
    prologue::exec_prologue_tick(&mut vm);

    // 3. Verify Result
    // Result should be written to East (5,6)
    // 10 -> Dup -> 10, 10 -> Add -> 20.
    assert_eq!(vm.grid[5][6], Value::Int(20));

    // Verify Input Consumed
    assert_eq!(vm.grid[5][4], Value::Int(0));

    // Verify Ribozyme is still there
    // It might have moved if random walk, but we can check if it exists somewhere or if it stayed put because it acted.
    // The implementation says: "Do not move" if it acted.
    assert_eq!(vm.grid[5][5], Value::Str("🛠".to_string()));
}
