use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_oracle_runes() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Test 1: Assert (¶)
    // "fact" -> ! -> ¶
    vm.grid[5][3] = Value::Str("fact".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("¶".to_string());

    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    {
        assert!(vm.knowledge_base.contains(&Value::Str("fact".to_string())));
    }

    // Test 2: Query (λ)
    // "fact" -> ! -> λ -> South
    // Query needs to run AFTER Assert. Assert ran in previous tick?
    // exec_prologue_tick calls process_sinks. Assert is a sink.
    // So after first tick, "fact" is in KB.
    // Now we setup Query.

    vm.grid[7][3] = Value::Str("fact".to_string());
    vm.grid[7][4] = Value::Str("!".to_string());
    vm.grid[7][5] = Value::Str("λ".to_string());

    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    {
        // Check grid south (8,5)
        // Note: process_sinks writes to grid.
        match &vm.grid[8][5] {
            Value::Int(1) => (), // Success
            v => panic!("Expected Int(1) at 8,5, got {:?}", v),
        }
    }
}
