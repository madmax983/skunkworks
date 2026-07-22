use chimera_lang::ast::Nucleotide;
use chimera_lang::opcode::OpCode;
use chimera_lang::tapestry_compiler::compile;

#[test]
fn test_tapestry_basic_compilation() {
    let source = "
| Forth | Math | Logic |
|-------|------|-------|
| 10    | 20   | foo   |
| 5     | 4    | p     |
| +     | *    |       |
| p     | p    |       |
";
    let source = source.trim();
    let dna = compile(source).unwrap();
    assert_eq!(dna.helix.strands.len(), 3);

    let forth_strand = &dna.helix.strands[0];
    assert_eq!(forth_strand.genes[0].op, OpCode::Push);
    assert_eq!(forth_strand.genes[0].args, vec![Nucleotide::Number(10)]);
    assert_eq!(forth_strand.genes[1].op, OpCode::Push);
    assert_eq!(forth_strand.genes[1].args, vec![Nucleotide::Number(5)]);
    assert_eq!(forth_strand.genes[2].op, OpCode::Add);
    assert_eq!(forth_strand.genes[3].op, OpCode::Print);
}
