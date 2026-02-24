use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_weaver_reverse_translation() {
    let mut vm = make_vm();

    // Add a strand to DNA
    let strand = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ],
    };
    // Strand 0
    vm.dna.helix.strands.push(strand);

    // Setup Weaver at (5,5)
    vm.grid[5][5] = Value::Str("🕷".to_string());

    // Inject delayed signal at West (5,4) -> Int(0)
    // We use delayed_signals so it's promoted to signal_grid in prepare_signals
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Int(0));

    // Run tick
    exec_prologue_tick(&mut vm);

    // Check East of Weaver (5,6) and (5,7)
    // (5,6) should be Push(42) -> Int(42)
    assert_eq!(vm.grid[5][6], Value::Int(42));

    // (5,7) should be Add -> Str("A")
    assert_eq!(vm.grid[5][7], Value::Str("A".to_string()));
}

#[test]
fn test_splicer_split() {
    let mut vm = make_vm();

    // Setup Splicer at (5,5)
    vm.grid[5][5] = Value::Str("✂".to_string());

    // Setup List at West (5,4)
    vm.grid[5][4] = Value::Junction(
        JunctionType::Any,
        vec![Value::Int(1), Value::Int(2), Value::Int(3)],
    );

    // Run tick
    exec_prologue_tick(&mut vm);

    // Check North (4,5): Head -> 1
    assert_eq!(vm.grid[4][5], Value::Int(1));

    // Check South (6,5): Tail -> [2, 3]
    match &vm.grid[6][5] {
        Value::Junction(_, list) => {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0], Value::Int(2));
            assert_eq!(list[1], Value::Int(3));
        }
        _ => panic!("Expected Junction at South"),
    }

    // Check West (5,4): Cleared
    assert_eq!(vm.grid[5][4], Value::Int(0));
}

#[test]
fn test_ligase_join() {
    let mut vm = make_vm();

    // Setup Ligase at (5,5)
    vm.grid[5][5] = Value::Str("🔗".to_string());

    // Setup North (4,5) and South (6,5)
    vm.grid[4][5] = Value::Int(10);
    vm.grid[6][5] = Value::Int(20);

    // Run tick
    exec_prologue_tick(&mut vm);

    // Check East (5,6): [10, 20]
    match &vm.grid[5][6] {
        Value::Junction(_, list) => {
            assert_eq!(list.len(), 2);
            assert_eq!(list[0], Value::Int(10));
            assert_eq!(list[1], Value::Int(20));
        }
        _ => panic!("Expected Junction at East"),
    }

    // Check Inputs cleared
    assert_eq!(vm.grid[4][5], Value::Int(0));
    assert_eq!(vm.grid[6][5], Value::Int(0));
}

#[test]
fn test_chromatin_reprogramming() {
    let mut vm = make_vm();

    // Setup Chromatin at (5,5)
    vm.grid[5][5] = Value::Str("χ".to_string());

    // Setup Configuration List at North (4,5)
    // [Function(0), Target("X")]
    vm.grid[4][5] = Value::Junction(
        JunctionType::Any,
        vec![Value::Int(0), Value::Str("X".to_string())],
    );

    // Run tick
    exec_prologue_tick(&mut vm);

    // Check Agent State
    // The agent might have moved, so we need to find it in the list
    let agent = vm
        .prologue_state
        .agents
        .iter()
        .find(|a| {
            // Approximate check: Was at 5,5, moved to neighbor or stayed
            let dy = (a.y as i64 - 5).abs();
            let dx = (a.x as i64 - 5).abs();
            dy <= 1 && dx <= 1
        })
        .expect("Agent should exist");

    if let Value::Junction(_, list) = &agent.state {
        // Function 0 (Int), Target "X" (Str)
        assert_eq!(list[0], Value::Int(0));
        assert_eq!(list[1], Value::Str("X".to_string()));
    } else {
        panic!(
            "Agent state not updated or invalid format: {:?}",
            agent.state
        );
    }
}
