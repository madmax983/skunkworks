use chimera_lang::prelude::*;

#[test]
fn test_simplified_api() {
    // 1. Nucleotide conversion
    let n1: Nucleotide = 42.into();
    assert_eq!(n1, Nucleotide::Number(42));

    let n2: Nucleotide = "test".into();
    assert_eq!(n2, Nucleotide::String("test".to_string()));

    // 2. Gene construction
    let g1 = Gene::new(OpCode::Push, vec![n1]);
    assert_eq!(g1.op, OpCode::Push);
    assert_eq!(g1.args.len(), 1);

    let g2: Gene = OpCode::Add.into();
    assert_eq!(g2.op, OpCode::Add);
    assert_eq!(g2.args.len(), 0);

    // 3. DNA construction
    let genes = vec![g1, g2];
    let dna = Dna::from_genes(genes);

    assert_eq!(dna.helix.strands.len(), 1);
    assert_eq!(dna.helix.strands[0].genes.len(), 2);
}
