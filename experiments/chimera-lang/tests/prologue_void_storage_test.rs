use chimera_lang::ast::{Dna, Helix, Strand};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes: vec![] }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_void_storage() {
    let mut vm = make_vm();

    // 1. Push 42 to Void
    // 5,4: 42
    // 5,5: ! (Source) -> 5,5 Signal
    // 5,6: Ø (Void In) -> Reads 5,5 Signal

    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("Ø".to_string());

    exec_prologue_tick(&mut vm);

    // Verify Void Buffer
    assert_eq!(vm.prologue_state.void_buffer.len(), 1);
    assert_eq!(vm.prologue_state.void_buffer[0], Value::Int(42));

    // Verify Ø lit up
    assert_eq!(vm.prologue_state.signal_grid[5][6], Some(Value::Int(1)));

    // 2. Pop from Void
    // 6,6: § (Void Out)

    vm.grid[6][6] = Value::Str("§".to_string());

    // Stop the source to prevent double pushing
    vm.grid[5][5] = Value::Int(0);

    exec_prologue_tick(&mut vm);

    // Buffer should be empty (popped)
    assert_eq!(vm.prologue_state.void_buffer.len(), 0);

    // § should output 42
    assert_eq!(vm.prologue_state.signal_grid[6][6], Some(Value::Int(42)));
}
