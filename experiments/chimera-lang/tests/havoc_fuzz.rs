use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use proptest::prelude::*;

// Expanded OpCode Strategy
fn opcode_strategy() -> impl Strategy<Value = OpCode> {
    prop_oneof![
        Just(OpCode::Push),
        Just(OpCode::Add),
        Just(OpCode::Sub),
        Just(OpCode::Mul),
        Just(OpCode::Div),
        Just(OpCode::Dup),
        Just(OpCode::Drop),
        Just(OpCode::Swap),
        Just(OpCode::Jump),
        Just(OpCode::Brz),
        Just(OpCode::Call),
        Just(OpCode::Ret),
        Just(OpCode::Simulate),
        Just(OpCode::Dream),
        Just(OpCode::Prophecy),
        Just(OpCode::Chaos),
        Just(OpCode::HavocRate),
        Just(OpCode::Scavenge),
        Just(OpCode::Digest),
        Just(OpCode::Supernova),
        Just(OpCode::Singularity),
        Just(OpCode::Zip),
        Just(OpCode::Map),
        Just(OpCode::Fold),
        Just(OpCode::Filter),
        Just(OpCode::Compile),
        Just(OpCode::Decompile),
        Just(OpCode::Eval),
        Just(OpCode::LispEval),
        Just(OpCode::Curry),
        Just(OpCode::Chain),
        Just(OpCode::Quote),
        // Nova Genetics
        Just(OpCode::Mitosis),
        Just(OpCode::Apoptosis),
        Just(OpCode::Splice),
        Just(OpCode::Frankenstein),
        Just(OpCode::Crossover),
        Just(OpCode::Recombine),
        Just(OpCode::CrisprScan),
        Just(OpCode::Cas9Cut),
        Just(OpCode::Ligase),
        Just(OpCode::Incubate),
        Just(OpCode::Metamorphosis),
        Just(OpCode::Genesis),
        // Nova Physics
        Just(OpCode::Entropy),
        Just(OpCode::Stabilize),
        Just(OpCode::Disintegrate),
        Just(OpCode::QuantumTunnel),
        Just(OpCode::QuantumJump),
        Just(OpCode::Superpose),
        Just(OpCode::Collapse),
        Just(OpCode::Observe),
        // Nova Grid
        Just(OpCode::GRead),
        Just(OpCode::GWrite),
        Just(OpCode::Radiate),
        Just(OpCode::Siphon),
        Just(OpCode::Virus),
        // Dangerous
        Just(OpCode::HoloInvoke), // Recursive?
        Just(OpCode::BabelCompile),
    ]
}

// Deep Nucleotide Strategy to test Recursion Limits
fn nucleotide_strategy() -> impl Strategy<Value = Nucleotide> {
    let leaf = prop_oneof![
        any::<i64>().prop_map(Nucleotide::Number),
        ".*".prop_map(Nucleotide::String),
        "[a-zA-Z_][a-zA-Z0-9_]*".prop_map(Nucleotide::Identifier),
    ];

    leaf.prop_recursive(
        8,   // Deeper than standard 4
        256, // More nodes
        20,  // Items per collection
        |inner| {
            prop_oneof![prop::collection::vec(inner, 0..20)
                .prop_map(|v| Nucleotide::Junction(JunctionType::Any, v)),]
        },
    )
}

fn gene_strategy() -> impl Strategy<Value = Gene> {
    (
        opcode_strategy(),
        prop::collection::vec(nucleotide_strategy(), 0..5),
    )
        .prop_map(|(op, args)| Gene { op, args })
}

fn strand_strategy() -> impl Strategy<Value = Strand> {
    prop::collection::vec(gene_strategy(), 0..50).prop_map(|genes| Strand { genes })
}

fn dna_strategy() -> impl Strategy<Value = Dna> {
    prop::collection::vec(strand_strategy(), 1..10).prop_map(|strands| Dna {
        evolution_config: None,
        helix: Helix { strands },
    })
}

proptest! {
    // Run 100 cases, allow failures (we want to find them)
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn test_havoc_fuzz(dna in dna_strategy()) {
        let mut vm = ChimeraVM::new(dna);
        vm.chaos_mode = true;
        vm.energy = 10000; // Infinite Power!

        // Run for enough steps to cause trouble
        for _ in 0..500 {
            if vm.halted { break; }
            vm.step();
        }
    }
}
