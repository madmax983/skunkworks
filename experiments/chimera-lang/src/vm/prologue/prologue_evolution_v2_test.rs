use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_incubate_grid() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // 3 -> ! -> ~ -> G -> "add" -> 5 -> "print"
    // West signal to G is 3.
    // G reads 3 cells East: "add", 5, "print"

    vm.grid[5][2] = Value::Int(3);
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("~".to_string());
    vm.grid[5][5] = Value::Str("G".to_string());

    // Cells to read
    vm.grid[5][6] = Value::Str("add".to_string());
    vm.grid[5][7] = Value::Int(5);
    vm.grid[5][8] = Value::Str("print".to_string());

    // Run multiple ticks to ensure signal propagation
    // Tick 1: ! emits 3.
    // Tick 2: ~ carries 3.
    // Tick 3: G receives 3.
    // Since we keep emitting 3, we expect multiple strands if we run for 10 ticks.
    // Let's run for 5 ticks, which should be enough for at least 1-2 creations.
    for _ in 0..5 {
        exec_prologue_tick(&mut vm);
    }

    // Expect new strand
    assert!(vm.dna.helix.strands.len() >= 1, "Should create at least 1 strand");
    let strand = &vm.dna.helix.strands[0];

    // Check genes
    // Note: The order of genes depends on compiler output for "add\n5\nprint\n".
    // "add" -> OpCode::Add
    // "5" -> OpCode::Push(5)
    // "print" -> OpCode::Print

    // Debug output if fails
    for g in &strand.genes {
        println!("{:?}", g);
    }

    assert_eq!(strand.genes.len(), 3);
    assert_eq!(strand.genes[0].op.to_string(), "add");
    // 5 becomes Push(5)
    assert_eq!(strand.genes[1].op.to_string(), "push");
    let arg = &strand.genes[1].args[0];
    if let crate::ast::Nucleotide::Number(n) = arg {
        assert_eq!(*n, 5);
    } else {
        panic!("Expected Number(5)");
    }
    assert_eq!(strand.genes[2].op.to_string(), "print");
}
