use chimera_lang::ast::{Dna, Helix, Strand};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_void_overflow_protection() {
    let dna = Dna { evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // 5,4: 42
    // 5,5: ! (Source) -> Emits 42
    // 5,6: Ø (Void In) -> Consumes 42, Pushes to Buffer

    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("Ø".to_string());

    // Run for 2000 ticks.
    let iterations = 2000;
    for _ in 0..iterations {
        exec_prologue_tick(&mut vm);
    }

    println!("Void Buffer Size: {}", vm.prologue_state.void_buffer.len());

    // Security Limit Check: Should be capped at MAX_VOID_BUFFER (1024)
    assert!(
        vm.prologue_state.void_buffer.len() <= 1024,
        "Buffer overflowed limit! Size: {}",
        vm.prologue_state.void_buffer.len()
    );
    // In fact, since we pushed 2000 times, it should be exactly capped.
    // If it's less, the test setup might be wrong (Source not firing every tick?).
    // But since ! (Source) fires every tick, it should hit the cap.
    assert_eq!(
        vm.prologue_state.void_buffer.len(),
        1024,
        "Buffer was not capped at exact limit"
    );
}
