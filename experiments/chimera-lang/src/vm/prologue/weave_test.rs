use super::super::*;
use crate::ast::{Dna, Helix};
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_weave_circuit() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Warp at (2, 6) with value 10
    vm.grid[2][6] = Value::Str("║".to_string());
    vm.prologue_state.registers.insert((2, 6), Value::Int(10));

    // 2. Setup Operator '+' at (5, 6)
    vm.grid[5][6] = Value::Str("+".to_string());

    // 3. Setup Shuttle 'ð' at (5, 5)
    vm.grid[5][5] = Value::Str("ð".to_string());

    // 4. Tick 1
    // Shuttle moves from (5, 5) to (5, 6).
    // Interaction at (5, 5) is with "ð" (start pos, technically overwrites itself in init?).
    // Actually, when scanning, we find "ð" at (5, 5).
    // process_agents:
    //   reads "ð". state=[0, 1, 0, 0].
    //   interacts with underfoot (0). No op.
    //   moves to (5, 6).
    //   restores underfoot (0) to (5, 5).
    //   captures new underfoot "+" from (5, 6).
    //   updates state=[0, 1, 0, "+"].
    //   writes "ð" to (5, 6).
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 6)
    assert_eq!(vm.grid[5][6], Value::Str("ð".to_string()));
    assert_eq!(vm.grid[5][5], Value::Int(0)); // Cleared

    // 5. Tick 2
    // Shuttle at (5, 6). Underfoot is "+".
    // process_agents:
    //   interacts with underfoot "+".
    //   scans col 6, finds "║" at (2, 6). Value 10.
    //   payload 0 + 10 = 10.
    //   moves to (5, 7).
    //   restores underfoot "+" to (5, 6).
    //   captures new underfoot (0) from (5, 7).
    //   updates state=[0, 1, 10, 0].
    //   writes "ð" to (5, 7).
    prologue::exec_prologue_tick(&mut vm);

    // Verify Shuttle at (5, 7)
    assert_eq!(vm.grid[5][7], Value::Str("ð".to_string()));
    // Verify "+" restored at (5, 6)
    assert_eq!(vm.grid[5][6], Value::Str("+".to_string()));

    // Verify Payload
    let reg = vm
        .prologue_state
        .registers
        .get(&(5, 7))
        .expect("Agent not in registers");
    // Unpack: Junction(All, [dy, dx, Payload, Underfoot, Tension])
    if let Value::Junction(_, list) = reg {
        assert_eq!(list.len(), 5);
        assert_eq!(list[2], Value::Int(10), "Payload should be 10");
    } else {
        panic!("Invalid agent state format: {:?}", reg);
    }
}
