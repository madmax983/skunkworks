#[cfg(all(test, feature = "elektra", feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::prelude::*;
    use crate::vm::ChimeraVM;
    use crate::vm::VisualEffect;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_tesla_coil_visual_effects() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::TeslaCoil, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];
        let mut vm = make_vm(genes);

        vm.voltage_grid[8][8] = 150.0;
        vm.resistance_grid[8][8] = -1.0;
        vm.context_loc = (8, 8);

        // Step 1: Push
        vm.step();
        // Step 2: Push
        vm.step();
        // Step 3: TeslaCoil -> Adds effect (TTL 3)
        vm.step();

        assert!(!vm.visual_effects.is_empty(), "Expected visual effects");

        let mut has_lightning = false;
        for effect in &vm.visual_effects {
            if let VisualEffect::Lightning { from, ttl, .. } = effect {
                assert_eq!(*from, (8, 8));
                assert_eq!(*ttl, 3);
                has_lightning = true;
            }
        }
        assert!(has_lightning, "Expected Lightning effect");

        // Step 4: Jump(0) -> TTL 3->2
        vm.step();

        if let Some(VisualEffect::Lightning { ttl, .. }) = vm.visual_effects.first() {
             assert_eq!(*ttl, 2, "TTL should be 2");
        }

        // Step 5: Push (loop) -> TTL 2->1
        vm.step();
        if let Some(VisualEffect::Lightning { ttl, .. }) = vm.visual_effects.first() {
             assert_eq!(*ttl, 1, "TTL should be 1");
        }

        // Step 6: Push (loop) -> TTL 1->0
        vm.step();
        assert!(vm.visual_effects.is_empty(), "Effects should be cleared");
    }

    #[test]
    fn test_galvanize_visual_effect() {
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Galvanize, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];
        let mut vm = make_vm(genes);
        vm.voltage_grid[8][8] = 120.0;
        vm.resistance_grid[8][8] = -1.0;
        vm.context_loc = (8, 8);
        vm.graveyard.push(Strand { genes: vec![] });

        // Step 1: Push
        vm.step();
        // Step 2: Galvanize -> Adds Spark (TTL 5)
        vm.step();

        assert!(!vm.visual_effects.is_empty(), "Expected visual effects");

        if let VisualEffect::Spark { loc, ttl, .. } = &vm.visual_effects[0] {
            assert_eq!(*loc, (8, 8));
            assert_eq!(*ttl, 5);
        }

        // Step 3: Jump -> TTL 4
        vm.step();
        if let VisualEffect::Spark { ttl, .. } = &vm.visual_effects[0] {
            assert_eq!(*ttl, 4);
        }
    }
}
