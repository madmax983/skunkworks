use chimera_lang::prologue_compiler;
use chimera_lang::vm::Value;
use std::path::Path;

#[test]
fn test_prologue_compilation() {
    let source = r#"
config {
    mode: Orca
}

definitions {
    Z: {
        "Custom Rune Z" print
    }
}

grid {
    . . . .
    . Z . .
    . . . .
    . . . .
}

dna {
    strand boot {
        "Booting" print
    }
}
"#;

    let result = prologue_compiler::compile(source, None);
    assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

    let (dna, grid, orca_mode, custom_runes) = result.unwrap();

    // Check DNA
    assert_eq!(
        dna.helix.strands.len(),
        2,
        "Should have 2 strands (1 main + 1 custom rune)"
    );

    // Check main strand
    // Note: The order depends on implementation. Usually appended.
    // 'boot' strand should be there.
    let boot_strand = dna.helix.strands.iter().find(|s| {
        // We can't easily check name as it's compiled away, but we can check content
        // "Booting" print -> Push("Booting"), Print
        if s.genes.len() >= 2 {
            format!("{}", s.genes[0].op) == "push"
        } else {
            false
        }
    });
    assert!(boot_strand.is_some(), "Main strand 'boot' not found");

    // Check custom rune strand
    let rune_strand_idx = custom_runes.get("Z");
    assert!(rune_strand_idx.is_some(), "Custom rune 'Z' not registered");

    // Check config
    assert_eq!(orca_mode, Some(true), "Orca mode should be enabled");

    // Check grid
    assert!(grid.is_some());
    let g = grid.unwrap();
    // Row 1, Col 1 should be "Z" (Value::Str("Z"))
    // Grid parsing logic: . . . . -> 0, 0, 0, 0
    // . Z . . -> 0, "Z", 0, 0
    // Note: parser handles whitespace.
    // Let's verify a cell.
    // Indexing: grid[y][x]
    // The provided grid string has newlines.
    // Row 0: . . . .
    // Row 1: . Z . .
    assert!(matches!(g[1][1], Value::Str(ref s) if s == "Z"));
}
