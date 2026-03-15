use chimera_lang::prolouge_compiler;
use chimera_lang::vm::Value;

#[test]
fn test_prolouge_compilation() {
    let source = r#"
config {
    mode: Orca
}

grid {
    . . . .
    . 5 . .
    . ! . .
    . . . .
}

dna {
    strand main {
        "Hello" print
        10
        dup
        add
        fact("chimera", 1, 2)
        query(is_cool(?X))
        >>+<<
        [+]
        splice(main, main)
    }
}
"#;

    let result = prolouge_compiler::compile(source, None);
    assert!(result.is_ok(), "Prolouge compilation failed: {:?}", result.err());

    let (dna, grid, orca_mode, _) = result.unwrap();

    // Check config
    assert_eq!(orca_mode, Some(true), "Orca mode should be enabled");

    // Check grid
    assert!(grid.is_some(), "Grid should be parsed");
    let g = grid.unwrap();
    for (r, row) in g.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell != Value::Int(0) {
                println!("Grid[{}][{}] = {:?}", r, c, cell);
            }
        }
    }
    assert_eq!(g[1][1], Value::Int(5), "Grid cell at (1,1) should be 5");
    assert!(matches!(g[2][1], Value::Str(ref s) if s == "!"), "Grid cell at (2,1) should be '!'");

    // Check DNA
    assert_eq!(dna.helix.strands.len(), 1, "Should have 1 strand");
    let strand = &dna.helix.strands[0];

    let ops: Vec<String> = strand.genes.iter().map(|g| g.op.to_string()).collect();

    assert!(ops.contains(&"push".to_string()));
    assert!(ops.contains(&"print".to_string()));
    assert!(ops.contains(&"dup".to_string()));
    assert!(ops.contains(&"add".to_string()));
    assert!(ops.contains(&"assert".to_string()));

    #[cfg(feature = "oracle")]
    assert!(ops.contains(&"prolog_call".to_string()));

    #[cfg(feature = "nova")]
    {
        assert!(ops.contains(&"map".to_string())); // From hyper_op
        assert!(ops.contains(&"fold".to_string())); // From reduce_op
        assert!(ops.contains(&"splice".to_string())); // From genetic_splice
    }
}
