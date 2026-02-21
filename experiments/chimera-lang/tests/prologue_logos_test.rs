use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::opcode::OpCode;

fn setup_signal(vm: &mut ChimeraVM, val: Value, target_y: usize, target_x: usize, from_dir: &str) {
    match from_dir {
        "WEST" => {
            // Signal at (y, x-1)
            // Value at (y, x-2) -> ! at (y, x-1)
            if target_x >= 2 {
                vm.grid[target_y][target_x - 2] = val;
                vm.grid[target_y][target_x - 1] = Value::Str("!".to_string());
            }
        }
        "NORTH" => {
            // Signal at (y-1, x)
            // Value at (y-1, x-1) -> ! at (y-1, x)
            if target_y >= 1 && target_x >= 1 {
                vm.grid[target_y - 1][target_x - 1] = val;
                vm.grid[target_y - 1][target_x] = Value::Str("!".to_string());
            }
        }
        _ => {}
    }
}

#[test]
fn test_logos_grammar_definition() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Define Rule: Γ (Gamma) at (5,5)
    // West: Name "noun"
    // North: Def "dog|cat"
    vm.grid[5][5] = Value::Str("Γ".to_string());

    setup_signal(&mut vm, Value::Str("noun".to_string()), 5, 5, "WEST");
    setup_signal(&mut vm, Value::Str("dog|cat".to_string()), 5, 5, "NORTH");

    exec_prologue_tick(&mut vm);

    // Verify Ack signal at (5,5)
    assert!(vm.prologue_state.signal_grid[5][5].is_some(), "Gamma should emit ack signal");

    // Verify Rule in Engine
    let engine = &vm.prologue_state.logos_engine;
    assert!(engine.rules.contains_key("noun"));

    // Check structure
    let rule = engine.rules.get("noun").unwrap();
    match rule {
        chimera_lang::vm::prologue::logos::GrammarRule::Choice(opts) => {
            assert_eq!(opts.len(), 2);
        },
        _ => panic!("Expected Choice rule"),
    }
}

#[test]
fn test_logos_parsing() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Pre-define rule
    vm.prologue_state.logos_engine.define_rule("S", "A B");
    vm.prologue_state.logos_engine.define_rule("A", "\"hello\"");
    vm.prologue_state.logos_engine.define_rule("B", "\"world\"");

    // Test Parse: « (Left Guillemet) at (5,5)
    // West: Input "hello world"
    // North: Rule "S"
    vm.grid[5][5] = Value::Str("«".to_string());

    setup_signal(&mut vm, Value::Str("hello world".to_string()), 5, 5, "WEST");
    setup_signal(&mut vm, Value::Str("S".to_string()), 5, 5, "NORTH");

    exec_prologue_tick(&mut vm);

    // Verify Output
    // Should be Junction(All, [Str("hello"), Str("world")])
    if let Some(val) = &vm.prologue_state.signal_grid[5][5] {
        match val {
            Value::Junction(chimera_lang::ast::JunctionType::All, list) => {
                assert_eq!(list.len(), 2);
                assert_eq!(list[0], Value::Str("hello".to_string()));
                assert_eq!(list[1], Value::Str("world".to_string()));
            },
            _ => panic!("Expected Junction output, got {:?}", val),
        }
    } else {
        panic!("Pi («) did not emit output");
    }
}

#[test]
fn test_logos_generation() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Pre-define rule
    vm.prologue_state.logos_engine.define_rule("greeting", "\"hi\"");

    // Test Generate: » (Right Guillemet) at (5,5)
    // West: Rule "greeting"
    vm.grid[5][5] = Value::Str("»".to_string());

    setup_signal(&mut vm, Value::Str("greeting".to_string()), 5, 5, "WEST");

    exec_prologue_tick(&mut vm);

    // Verify Output
    if let Some(val) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*val, Value::Str("hi".to_string()));
    } else {
        panic!("Sigma (») did not emit output");
    }
}

#[test]
fn test_logos_dna_definition() {
    // Setup DNA: Strand 0 dummy, Strand 1 has genes [Push("A"), Push("B")]
    // We use Strand 1 because Int(0) is considered Empty Signal by the VM circuit logic.
    let genes = vec![
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("A".to_string())] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::String("B".to_string())] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes: vec![] }, Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Define from DNA: Γ at (5,5)
    // West: Name "gene_rule"
    // North: Strand Index 1
    vm.grid[5][5] = Value::Str("Γ".to_string());

    setup_signal(&mut vm, Value::Str("gene_rule".to_string()), 5, 5, "WEST");
    setup_signal(&mut vm, Value::Int(1), 5, 5, "NORTH");

    exec_prologue_tick(&mut vm);

    // Verify Rule Created
    let engine = &vm.prologue_state.logos_engine;
    assert!(engine.rules.contains_key("gene_rule"), "Rule 'gene_rule' not found in {:?}", engine.rules.keys());

    // Generate from it to verify structure
    if let Ok(gen) = engine.generate("gene_rule") {
        assert_eq!(gen, "A B");
    } else {
        panic!("Failed to generate from DNA-defined rule");
    }
}

#[test]
fn test_logos_weighted_choice() {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Define Weighted Rule: "10:\"Common\" | 1:\"Rare\""
    // Using quoted strings to ensure they are interpreted as Literals, not References.
    vm.grid[5][5] = Value::Str("Γ".to_string());
    setup_signal(&mut vm, Value::Str("loot".to_string()), 5, 5, "WEST");
    setup_signal(&mut vm, Value::Str("10:\"Common\" | 1:\"Rare\"".to_string()), 5, 5, "NORTH");

    exec_prologue_tick(&mut vm);

    // Generate multiple times to statistical check (probabilistic, so looseness required)
    // We just verify it generates *something* valid
    let engine = &vm.prologue_state.logos_engine;
    if let Ok(gen) = engine.generate("loot") {
        assert!(gen == "Common" || gen == "Rare", "Generated unexpected: {}", gen);
    } else {
        panic!("Failed to generate from weighted choice");
    }
}
