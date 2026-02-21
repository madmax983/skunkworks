use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_echo_record_playback() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Setup Record: '(' at 5,5
    vm.grid[5][5] = Value::Str("(".to_string());

    // 2. Toggle Record ON (Signal North)
    vm.tick_counter += 1;
    vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(1));
    exec_prologue_tick(&mut vm);

    // Verify recording is ON
    assert!(
        vm.prologue_state
            .echoes
            .get(&(5, 5))
            .expect("Echo buffer missing")
            .recording
    );

    // 3. Record Sequence (Signal West)
    let sequence = vec![10, 20, 30];
    for &val in &sequence {
        vm.tick_counter += 1;
        vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(val));
        exec_prologue_tick(&mut vm);
    }

    // Verify buffer
    let buffer = &vm
        .prologue_state
        .echoes
        .get(&(5, 5))
        .expect("Echo buffer missing")
        .buffer;
    assert_eq!(buffer.len(), 3);
    assert_eq!(buffer[0], Value::Int(10));
    assert_eq!(buffer[2], Value::Int(30));

    // 4. Toggle Record OFF (Signal North)
    vm.tick_counter += 1;
    vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(1));
    exec_prologue_tick(&mut vm);
    assert!(!vm.prologue_state.echoes.get(&(5, 5)).unwrap().recording);

    // 5. Setup Play: ')' at 5,5
    vm.grid[5][5] = Value::Str(")".to_string());

    // 6. Toggle Play ON (Signal West) - This tick outputs first value!
    vm.tick_counter += 1;
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(1));
    exec_prologue_tick(&mut vm);
    assert!(vm.prologue_state.echoes.get(&(5, 5)).unwrap().playing);
    // Should output 10 immediately
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(10)));

    // 7. Playback (Output South)
    // Tick 2: Should output 20
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(20)));

    // Tick 3: Should output 30
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(30)));

    // Tick 4: Loop back to 10
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(10)));
}

#[test]
fn test_echo_reverse() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Manually inject a buffer [1, 2, 3]
    let mut echo = crate::vm::prologue::echo::EchoBuffer::default();
    echo.buffer = vec![Value::Int(1), Value::Int(2), Value::Int(3)];
    echo.playing = true;
    vm.prologue_state.echoes.insert((5, 5), echo);

    // Place ')' Play
    vm.grid[5][5] = Value::Str(")".to_string());

    // Play Forward: 1 (Tick 1)
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(1)));
    // Index 0 -> 1

    // Trigger Reverse: Overwrite with '{' and trigger West
    vm.grid[5][5] = Value::Str("{".to_string());
    vm.tick_counter += 1;
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(1)); // Trigger
    exec_prologue_tick(&mut vm);

    assert!(vm.prologue_state.echoes.get(&(5, 5)).unwrap().reversed);
    // Index stays 1

    // Switch back to Play
    vm.grid[5][5] = Value::Str(")".to_string());

    // Resume Playback (Tick 3)
    // Index increments to 2.
    // Logic: len(3) - 1 - (idx(2-1=1) % 3) = 1.
    // Buffer[1] = 2.
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(2)));

    // Tick 4: Index 3. 2 (Mapped 0). Buffer[0] = 1.
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(1)));

    // Tick 5: Index 4. 0 (Mapped 2). Buffer[2] = 3.
    vm.tick_counter += 1;
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(3)));
}
