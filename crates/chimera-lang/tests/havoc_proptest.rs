use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use proptest::prelude::*;

fn opcode_strategy() -> impl Strategy<Value = OpCode> {
    prop_oneof![
        Just(OpCode::Push),
        Just(OpCode::Add),
        Just(OpCode::Sub),
        Just(OpCode::Dup),
        Just(OpCode::Drop),
        Just(OpCode::Jump),
        Just(OpCode::Brz),
        Just(OpCode::Simulate),
        Just(OpCode::Dream),
        Just(OpCode::HavocRate),
        Just(OpCode::Photosynthesize),
        Just(OpCode::Consume),
        Just(OpCode::GWrite),
        Just(OpCode::GRead),
        Just(OpCode::Meme),
        Just(OpCode::Poly),
        Just(OpCode::AkashicWrite),
        Just(OpCode::AkashicRead),
        Just(OpCode::Scavenge), // 😈
    ]
}

fn nucleotide_strategy() -> impl Strategy<Value = Nucleotide> {
    let leaf = prop_oneof![
        any::<i64>().prop_map(Nucleotide::Number),
        "[a-zA-Z0-9_]+".prop_map(Nucleotide::String),
        "[a-zA-Z_][a-zA-Z0-9_]*".prop_map(Nucleotide::Identifier),
    ];

    leaf.prop_recursive(
        4,  // levels deep
        64, // max size nodes
        10, // items per collection
        |inner| {
            prop_oneof![prop::collection::vec(inner, 0..10)
                .prop_map(|v| Nucleotide::Junction(JunctionType::Any, v)),]
        },
    )
}

fn gene_strategy() -> impl Strategy<Value = Gene> {
    (
        opcode_strategy(),
        prop::collection::vec(nucleotide_strategy(), 0..3),
    )
        .prop_map(|(op, args)| Gene { op, args })
}

fn strand_strategy() -> impl Strategy<Value = Strand> {
    prop::collection::vec(gene_strategy(), 0..20).prop_map(|genes| Strand { genes })
}

fn dna_strategy() -> impl Strategy<Value = Dna> {
    prop::collection::vec(strand_strategy(), 1..5).prop_map(|strands| Dna { evolution_config: None,
        helix: Helix { strands },
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(50))]

    #[test]
    fn test_vm_fuzz(dna in dna_strategy()) {
        let mut vm = ChimeraVM::new(dna);
        vm.chaos_mode = true; // Engage Entropy

        // Give it some juice
        vm.energy = 1000;

        for _ in 0..100 {
            if vm.halted { break; }
            vm.step();
        }
    }
}
