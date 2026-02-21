use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::vm::prologue::exec_prologue_tick;

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
