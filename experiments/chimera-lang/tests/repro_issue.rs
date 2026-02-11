#[cfg(feature = "nova")]
#[test]
fn test_add_strand_consistency() {
    use chimera_lang::prelude::*;

    // 1. Create VM with 1 natural strand
    let genes = vec![
        Gene { op: OpCode::Photosynthesize, args: vec![] },
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];
    let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
    let mut vm = ChimeraVM::new(dna);

    // 2. Add artificial strand using the new method
    let artificial_strand = Strand {
        genes: vec![
            Gene { op: OpCode::Photosynthesize, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(1)] }, // Jump to self
        ],
    };
    let idx = vm.add_strand(artificial_strand);
    assert_eq!(idx, 1);

    // Give energy
    vm.energy = 1000;

    // 3. Run artificial strand
    vm.ip = (1, 0);

    // We expect it to die after ~100 ticks (50 telomeres, -1 every 2 ticks)
    let mut steps = 0;
    while steps < 120 && !vm.halted {
        vm.step();
        if vm.ip.0 != 1 {
            // Switched strand (senescence)
            break;
        }
        steps += 1;
    }

    assert!(steps < 110, "Strand lived too long (Immortal)! Steps: {}", steps);
    assert!(steps > 90, "Strand died too early! Steps: {}", steps);
}
