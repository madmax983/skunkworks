use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;
use std::time::Instant;

#[test]
fn test_automaton_jump_performance() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    let y = 5;
    let x = 5;
    vm.grid[y][x] = Value::Str("🤖".to_string());

    // Construct a malicious program:
    // [ ... ]
    // The jump from [ needs to scan to the matching ].
    // We pad with spaces or no-ops.
    let n = 100_000;
    let mut program = String::with_capacity(n + 2);
    program.push('[');
    for _ in 0..n {
        program.push(' ');
    }
    program.push(']');

    let state_str = format!("A:0:1:0:{}", program);
    vm.prologue_state.registers.insert(
        (y, x),
        Value::Str(state_str)
    );

    println!("Starting execution with program length {}", program.len());
    let start = Instant::now();

    // This tick will parse the agent, execute '[', scan to ']', and update state.
    exec_prologue_tick(&mut vm);

    let duration = start.elapsed();
    println!("Execution took {:?}", duration);

    // If it's O(N^2), 20,000^2 = 400,000,000 checks.
    // Plus string overhead.
    // Expectation: > 1 second on unoptimized.
    // Optimized: < 10 ms.

    // We set a lenient threshold of 500ms.
    // If it takes longer, we consider it a DoS vulnerability.
    assert!(duration.as_millis() < 500, "Automaton jump took too long! Potential DoS vulnerability.");
}
