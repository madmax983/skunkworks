#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;
    use crate::vm::nova_constellation::ConstellationShape;
    use crate::vm::Value;

    fn make_gene(op: OpCode, args: Vec<Nucleotide>) -> Gene {
        Gene { op, args }
    }

    fn empty_dna() -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        }
    }

    #[test]
    fn test_constellation_triangulum() {
        // Strand 0: Jump to 1
        let s0 = Strand {
            genes: vec![
                make_gene(OpCode::Jump, vec![Nucleotide::Number(1)]),
            ],
        };
        // Strand 1: Jump to 2
        let s1 = Strand {
            genes: vec![
                make_gene(OpCode::Jump, vec![Nucleotide::Number(2)]),
            ],
        };
        // Strand 2: Jump to 0 (Completes cycle)
        let s2 = Strand {
            genes: vec![
                make_gene(OpCode::Jump, vec![Nucleotide::Number(0)]),
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![s0, s1, s2],
            },
        };

        let mut vm = ChimeraVM::new(dna);

        // Step 1: 0 -> 1
        vm.step();
        assert_eq!(vm.star_map.history[0], 0);

        // Step 2: 1 -> 2
        vm.step();

        // Step 3: 2 -> 0
        vm.step();

        // Step 4: 0 -> 1 (Cycle 0-1-2-0 detected)
        vm.step();

        assert_eq!(vm.star_map.active_constellations.len(), 1, "Constellation should be detected");
        let c = &vm.star_map.active_constellations[0];
        assert_eq!(c.shape, ConstellationShape::Triangulum);

        let energy_before = vm.energy;
        vm.step();
        // Step 5: 1 -> 2 (Cycle 1-2-0-1 detected)
        // Now we have 2 active constellations (Triangulums).
        // Gain from C1: +1.
        // Gain from C2: +1.
        // Cost: -1.
        // Net: +1.

        // So energy should INCREASE.
        assert!(vm.energy > energy_before, "Energy should increase due to resonance (stacking constellations). Before: {}, After: {}", energy_before, vm.energy);
    }

    #[test]
    fn test_stargaze_opcode() {
        let mut vm = ChimeraVM::new(empty_dna());

        // Manually inject a constellation
        vm.star_map.active_constellations.push(crate::vm::nova_constellation::Constellation {
            shape: ConstellationShape::Triangulum,
            duration: 10,
            strands: vec![0, 1, 2],
        });

        // Inject Stargaze gene
        vm.inject_genes(vec![make_gene(OpCode::Stargaze, vec![])]);

        vm.step();

        assert_eq!(vm.stack.pop(), Some(Value::Int(1)));
    }

    #[test]
    fn test_zenith_opcode() {
        let mut vm = ChimeraVM::new(empty_dna());

        // Manually inject a Triangulum (+15 bonus)
        vm.star_map.active_constellations.push(crate::vm::nova_constellation::Constellation {
            shape: ConstellationShape::Triangulum,
            duration: 10,
            strands: vec![0, 1, 2],
        });

        // Inject Zenith gene
        vm.inject_genes(vec![make_gene(OpCode::Zenith, vec![])]);

        let start_energy = vm.energy; // 50
        vm.step();

        // Sequence:
        // 1. Cost -1 (49)
        // 2. Tick +1 (50) (Constellation still active)
        // 3. Zenith consumes (+25) (75)

        // Base bonus (15) + Count bonus (10) = 25.
        // Net change = +25.

        assert_eq!(vm.energy, start_energy + 25);
        assert!(vm.star_map.active_constellations.is_empty());
        assert_eq!(vm.stack.pop(), Some(Value::Int(25)));
    }
}
