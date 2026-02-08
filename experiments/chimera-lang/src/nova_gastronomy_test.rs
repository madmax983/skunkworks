#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand, JunctionType};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_cooking_and_savoring() {
        // [ push(10) push(20) push(2) cook() savor() ]
        // Should gain 30 energy.
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Cook, args: vec![] },
            Gene { op: OpCode::Savor, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 50; // Reset

        vm.step(); // Push 10
        vm.step(); // Push 20
        vm.step(); // Push 2
        vm.step(); // Cook -> Dish(10, 20)

        if let Some(Value::Junction(JunctionType::Dish, _)) = vm.stack.last() {
            // Good
        } else {
            // It might have executed step 4.
            // Check stack
        }

        vm.step(); // Savor

        // Initial 50.
        // Steps 1-3 cost 3. (47)
        // Cook costs 5. (42)
        // Savor costs 1 but gains 30. (71)
        // But step() also decrements 1 per tick.
        assert!(vm.energy > 60, "Energy: {}", vm.energy);
    }

    #[test]
    fn test_spicing() {
        // [ push(10) push(1) cook() push("Spicy") spice() savor() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Cook, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("Spicy".to_string())] },
            Gene { op: OpCode::Spice, args: vec![] },
            Gene { op: OpCode::Savor, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        for _ in 0..6 {
            vm.step();
        }

        assert!(vm.buffs.contains_key("Spicy"));
    }

    #[test]
    fn test_sweet_buff() {
        // [ push(10) push(1) cook() push("Sweet") spice() savor() ]
        // Sweet doubles energy. 10 * 2 = 20.
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
            Gene { op: OpCode::Cook, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::String("Sweet".to_string())] },
            Gene { op: OpCode::Spice, args: vec![] },
            Gene { op: OpCode::Savor, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 100;

        for _ in 0..6 {
            vm.step();
        }

        assert!(vm.energy > 100);
        assert!(vm.buffs.contains_key("Sweet"));
    }

    #[test]
    fn test_cultivate() {
        // [ push(10) push(8) push(8) g_write() cultivate() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(8)] },
            Gene { op: OpCode::GWrite, args: vec![] },
            Gene { op: OpCode::Cultivate, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        // We need to set context_loc explicitly if we rely on GWrite setting it?
        // GWrite doesn't set context_loc?
        // Virus sets context_loc. Migrate sets context_loc.
        // GWrite just writes to grid.
        // Cultivate uses context_loc.
        // We need to move there first or set it manually.
        // Let's use Migrate.
        // [ push(8) push(8) migrate() push(10) push(8) push(8) g_write() cultivate() ]
        // Or just force it in test.
        vm.context_loc = (8, 8);

        for _ in 0..5 {
            vm.step();
        }

        // After 5 steps: 3 pushes, gwrite, cultivate.
        // GWrite puts 10 at 8,8.
        // Cultivate increments 10 -> 11.

        if let Value::Int(n) = vm.grid[8][8] {
            assert_eq!(n, 11);
        } else {
            panic!("Expected Int");
        }
    }

    #[test]
    fn test_banquet() {
        // Stack: [radius, amount] -> [2, 50]
        // Should distribute 50 energy to neighbors in radius 2
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
            Gene { op: OpCode::Banquet, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 100;
        vm.context_loc = (8, 8);

        vm.step(); // Push 2
        vm.step(); // Push 50
        vm.step(); // Banquet

        // Radius 2 includes 13 cells (1 center + 4 neighbors + 8 diag/dist 2).
        // Wait, get_circular_coords implementation:
        // r_sq = 4.
        // dx^2 + dy^2 <= 4.
        // (0,0): 0<=4 (Y)
        // (0,1): 1<=4 (Y)
        // (0,2): 4<=4 (Y)
        // (1,1): 2<=4 (Y)
        // (1,2): 5<=4 (N)
        // Count: Center(1) + 4*(1,0) + 4*(2,0) + 4*(1,1) = 1 + 4 + 4 + 4 = 13.
        // 50 / 13 = 3 per cell.

        if let Value::Int(n) = vm.grid[8][8] {
            assert_eq!(n, 3);
        } else {
            panic!("Expected Int at center");
        }

        if let Value::Int(n) = vm.grid[8][9] {
            assert_eq!(n, 3);
        }

        // Energy reduced: 100 - 3 (steps) - 50 (banquet) = 47.
        assert!(vm.energy < 50);
    }
}
