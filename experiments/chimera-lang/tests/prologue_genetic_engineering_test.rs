use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;

#[test]
fn test_genetic_engineering_compose() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Test: Compose "add" and [5, 3] -> "add(5, 3)"
    // Circuit:
    // Args (4,4) -> ! (4,5) -> ⚒ (5,5) reads North
    // Op (5,3)   -> ! (5,4) -> ⚒ (5,5) reads West

    // Args: 4,4
    vm.grid[4][4] = Value::Junction(JunctionType::Any, vec![Value::Int(5), Value::Int(3)]);
    // !: 4,5
    vm.grid[4][5] = Value::Str("!".to_string());

    // OpCode: 5,3
    vm.grid[5][3] = Value::Str("add".to_string());
    // !: 5,4
    vm.grid[5][4] = Value::Str("!".to_string());

    // Rune: 5,5
    vm.grid[5][5] = Value::Str("⚒".to_string());

    exec_prologue_tick(&mut vm);

    let sig = &vm.prologue_state.signal_grid[5][5];
    assert!(sig.is_some(), "Signal not produced");
    if let Some(Value::Str(s)) = sig {
        assert_eq!(s, "add(5, 3)");
    } else {
        panic!("Expected String signal, got {:?}", sig);
    }
}

#[test]
fn test_genetic_engineering_synthesize() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Test: Synthesize ["push(5)", "add"] -> New Strand
    // Circuit:
    // List -> ! -> 🧶

    let gene_list = vec![
        Value::Str("push(5)".to_string()),
        Value::Str("add".to_string())
    ];

    // 5,4: List
    vm.grid[5][4] = Value::Junction(JunctionType::Any, gene_list);

    // 5,5: Source !
    vm.grid[5][5] = Value::Str("!".to_string());

    // 5,6: Synthesize 🧶
    vm.grid[5][6] = Value::Str("🧶".to_string());

    exec_prologue_tick(&mut vm);

    // Check if new strand was added
    assert_eq!(vm.dna.helix.strands.len(), 1);

    let strand = &vm.dna.helix.strands[0];
    assert_eq!(strand.genes.len(), 2);
    assert_eq!(strand.genes[0].op.to_string(), "push");
    assert_eq!(strand.genes[1].op.to_string(), "add");

    // Check output index at South of 🧶 (6,6)
    if let Value::Int(idx) = vm.grid[6][6] {
        assert_eq!(idx, 0);
    } else {
        panic!("Expected strand index at 6,6, got {:?}", vm.grid[6][6]);
    }
}

#[test]
fn test_genetic_engineering_excise() {
    // Start with 3 strands (0, 1, 2)
    let mut dna = Dna { helix: Helix { strands: vec![] } };
    // Add dummy strands
    use chimera_lang::ast::Strand;
    dna.helix.strands.push(Strand { genes: vec![] }); // Index 0
    dna.helix.strands.push(Strand { genes: vec![] }); // Index 1
    dna.helix.strands.push(Strand { genes: vec![] }); // Index 2

    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: Remove Index 1
    // 1 -> ! -> ✂

    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[5][6] = Value::Str("✂".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.dna.helix.strands.len(), 2);
}

#[test]
fn test_genetic_engineering_splice() {
    let mut dna = Dna { helix: Helix { strands: vec![] } };
    use chimera_lang::ast::Strand;
    dna.helix.strands.push(Strand { genes: vec![] }); // 0
    dna.helix.strands.push(Strand { genes: vec![] }); // 1

    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: Splice strand 1 at index 1

    // West Input: Strand to insert (1)
    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());

    // North Input: Target Index (1)
    vm.grid[4][5] = Value::Int(1);
    vm.grid[4][6] = Value::Str("!".to_string());

    // Rune: 5,6
    vm.grid[5][6] = Value::Str("💉".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.dna.helix.strands.len(), 3);
}
