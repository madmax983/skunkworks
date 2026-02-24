#[cfg(feature = "nova")]
use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
#[cfg(feature = "nova")]
use chimera_lang::opcode::OpCode;
#[cfg(feature = "nova")]
use chimera_lang::vm::prologue::exec_prologue_tick;
#[cfg(feature = "nova")]
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
#[cfg(feature = "nova")]
fn test_prism_disperse() {
    // ▲: List -> N, E, S
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid:
    // . . .
    // . ▲ .
    // . . .
    // ▲ at (5,5). West is (5,4).
    // Inject signal at West.

    vm.grid[5][5] = Value::Str("▲".to_string());

    vm.prologue_state.delayed_signals[5][4] = Some(Value::Junction(
        chimera_lang::ast::JunctionType::Any,
        vec![Value::Int(1), Value::Int(2), Value::Int(3)],
    ));

    exec_prologue_tick(&mut vm);

    // Check North (4,5)
    match &vm.prologue_state.signal_grid[4][5] {
        Some(Value::Int(v)) => assert_eq!(*v, 1),
        _ => panic!("North failed: {:?}", vm.prologue_state.signal_grid[4][5]),
    }

    // Check East (5,6)
    match &vm.prologue_state.signal_grid[5][6] {
        Some(Value::Int(v)) => assert_eq!(*v, 2),
        _ => panic!("East failed: {:?}", vm.prologue_state.signal_grid[5][6]),
    }

    // Check South (6,5)
    match &vm.prologue_state.signal_grid[6][5] {
        Some(Value::Int(v)) => assert_eq!(*v, 3),
        _ => panic!("South failed: {:?}", vm.prologue_state.signal_grid[6][5]),
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_prism_converge() {
    // ▼: N, E, S -> List
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    vm.grid[5][5] = Value::Str("▼".to_string());

    // Inject delayed signals
    vm.prologue_state.delayed_signals[4][5] = Some(Value::Int(1)); // N
    vm.prologue_state.delayed_signals[5][6] = Some(Value::Int(2)); // E
    vm.prologue_state.delayed_signals[6][5] = Some(Value::Int(3)); // S

    exec_prologue_tick(&mut vm);

    // ▼ should output List[1, 2, 3] to Self (5,5)
    // Note: The order depends on implementation: N, E, S
    match &vm.prologue_state.signal_grid[5][5] {
        Some(Value::Junction(_, list)) => {
            // Filter empty if any
            let list: Vec<&Value> = list
                .iter()
                .filter(|v| match v {
                    Value::Int(0) => false,
                    Value::Str(s) => !s.is_empty(),
                    _ => true,
                })
                .collect();

            assert_eq!(list.len(), 3, "List length mismatch: {:?}", list);
            assert_eq!(list[0], &Value::Int(1)); // N
            assert_eq!(list[1], &Value::Int(2)); // E
            assert_eq!(list[2], &Value::Int(3)); // S
        }
        _ => panic!("Converge failed: {:?}", vm.prologue_state.signal_grid[5][5]),
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_prism_sequence() {
    // 🧬: StrandIdx -> List[GeneStr]
    let genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(42)],
        },
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
    ];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    vm.grid[5][5] = Value::Str("🧬".to_string());
    // Inject signal at West (5,4)
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(0)); // Strand 0

    exec_prologue_tick(&mut vm);

    match &vm.prologue_state.signal_grid[5][5] {
        Some(Value::Junction(_, list)) => {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0], Value::Str("push(42)".to_string()));
            assert_eq!(list[1], Value::Str("add".to_string()));
        }
        _ => panic!("Sequence failed: {:?}", vm.prologue_state.signal_grid[5][5]),
    }
}

#[test]
#[cfg(feature = "nova")]
fn test_prism_decompose() {
    // ⚛: "push(42)" -> N="push", S=[42]
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    vm.grid[5][5] = Value::Str("⚛".to_string());
    // Inject signal at West (5,4)
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("push(42)".to_string()));

    exec_prologue_tick(&mut vm);

    // Check North (OpCode)
    match &vm.prologue_state.signal_grid[4][5] {
        Some(Value::Str(s)) => assert_eq!(s, "push"),
        _ => panic!(
            "Decompose OpCode failed: {:?}",
            vm.prologue_state.signal_grid[4][5]
        ),
    }

    // Check South (Args)
    match &vm.prologue_state.signal_grid[6][5] {
        Some(Value::Junction(_, list)) => {
            assert_eq!(list.len(), 1);
            assert_eq!(list[0], Value::Int(42));
        }
        _ => panic!(
            "Decompose Args failed: {:?}",
            vm.prologue_state.signal_grid[6][5]
        ),
    }
}
