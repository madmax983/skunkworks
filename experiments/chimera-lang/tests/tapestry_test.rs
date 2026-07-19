use chimera_lang::tapestry_compiler::compile_tapestry;

#[test]
fn test_tapestry_compilation() {
    let source = r#"
| Column1 | Column2 |
|---------|---------|
| 1       | 2       |
| 3       | 4       |
"#;
    let dna = compile_tapestry(source).unwrap();
    assert_eq!(dna.helix.strands.len(), 2);
    assert_eq!(dna.helix.strands[0].genes.len(), 2);
    assert_eq!(dna.helix.strands[1].genes.len(), 2);
}
