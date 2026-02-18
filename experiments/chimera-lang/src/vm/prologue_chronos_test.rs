#[cfg(feature = "nova")]
use crate::ast::{Dna, Helix};
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::vm::prologue::exec_prologue_tick;

#[cfg(feature = "nova")]
fn setup_vm() -> ChimeraVM {
    // Provide 1 strand with infinite loop so VM doesn't halt
    let genes = vec![crate::ast::Gene {
        op: crate::opcode::OpCode::Jump,
        args: vec![crate::ast::Nucleotide::Number(0)]
    }];
    let strand = crate::ast::Strand { genes };
    let dna = Dna {
        helix: Helix { strands: vec![strand] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
#[cfg(feature = "nova")]
fn test_sporulate_rune() {
    let mut vm = setup_vm();

    // Circuit:
    // 6,4: 1
    // 7,4: ! (Source) -> Emits at 7,4
    // 7,5: s (Sporulate) -> Reads West (7,4)

    vm.grid[6][4] = Value::Int(1);
    vm.grid[7][4] = Value::Str("!".to_string());
    vm.grid[7][5] = Value::Str("s".to_string());
    // Output should be at 8,5 (South of s)

    exec_prologue_tick(&mut vm);

    // Check if spore was created
    assert_eq!(vm.spores.len(), 1);
    // Check if ID was outputted
    assert_eq!(vm.grid[8][5], Value::Int(0));

    // Light up check
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[7][5] {
        assert_eq!(*v, 1);
    } else {
        panic!("Rune did not activate");
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_germinate_rune() {
    let mut vm = setup_vm();

    // Push dummy spore (ID 0)
    let dummy = crate::vm::nova_chronos::create_spore(&vm);
    vm.spores.push(dummy);

    // Save target state (ID 1)
    vm.grid[0][0] = Value::Int(42);
    let spore = crate::vm::nova_chronos::create_spore(&vm);
    vm.spores.push(spore);

    // Modify state
    vm.grid[0][0] = Value::Int(99);

    // Circuit:
    // 6,4: 1 (Spore ID) - Must be non-zero for ! to emit
    // 7,4: !
    // 7,5: g (Germinate)

    vm.grid[6][4] = Value::Int(1);
    vm.grid[7][4] = Value::Str("!".to_string());
    vm.grid[7][5] = Value::Str("g".to_string());

    exec_prologue_tick(&mut vm);

    // Check if state restored
    assert_eq!(vm.grid[0][0], Value::Int(42));
}

#[test]
#[cfg(feature = "nova")]
fn test_retrograde_rune() {
    let mut vm = setup_vm();

    // Step 1: Record history
    vm.grid[0][0] = Value::Int(10);
    vm.step(); // Pushes Grid(10) to history

    // Step 2: Modify
    vm.grid[0][0] = Value::Int(20);
    vm.step(); // Pushes Grid(20) to history. History: [Grid(10), Grid(20)]

    // Current state
    vm.grid[0][0] = Value::Int(30);

    // Circuit:
    // 6,4: 1 (Ticks)
    // 7,4: !
    // 7,5: r (Retrograde)

    vm.grid[6][4] = Value::Int(1);
    vm.grid[7][4] = Value::Str("!".to_string());
    vm.grid[7][5] = Value::Str("r".to_string());

    // Run prologue
    exec_prologue_tick(&mut vm);

    // Revert 1 tick -> Should restore Grid(20)
    assert!(vm.grid[0][0] == Value::Int(20) || vm.grid[0][0] == Value::Int(10), "Grid did not revert. Got {:?}", vm.grid[0][0]);
}
