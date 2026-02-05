#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value, GRID_SIZE};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_thermo_ops() {
        // [ push(50) ignite() thermometer() push(20) chill() thermometer() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(50)] },
            Gene { op: OpCode::Ignite, args: vec![] },
            Gene { op: OpCode::Thermometer, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
            Gene { op: OpCode::Chill, args: vec![] },
            Gene { op: OpCode::Thermometer, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Push 50
        vm.step();
        // Ignite
        vm.step();

        // Thermometer
        vm.step();
        let t1 = match vm.stack.pop() {
            Some(Value::Int(n)) => n,
            _ => panic!("Expected Int from Thermometer"),
        };
        assert!(t1 > 20, "Heat should be > 20 (ambient) after Ignite(50)");

        // Push 20
        vm.step();
        // Chill
        vm.step();

        // Thermometer
        vm.step();
        let t2 = match vm.stack.pop() {
            Some(Value::Int(n)) => n,
            _ => panic!("Expected Int from Thermometer"),
        };

        assert!(t2 < t1, "Heat should decrease after Chill. t1={} t2={}", t1, t2);
    }

    #[test]
    fn test_heat_diffusion() {
        // [ push(100) ignite() ]
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Ignite, args: vec![] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.context_loc = (8, 8);
        vm.step(); // Push
        vm.step(); // Ignite

        crate::vm::nova::diffuse_heat(&mut vm);

        let center = vm.heat_grid[8][8];
        let neighbor = vm.heat_grid[8][9];

        assert!(neighbor > 20, "Heat should diffuse to neighbor (>20)");
        assert!(center < 120, "Center heat should decay/diffuse");
    }

    #[test]
    fn test_high_heat_damage() {
        // [ push(500) ignite() photosynthesize() jump(0) ]
        // Loop to keep VM alive and hot
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(500)] },
            Gene { op: OpCode::Ignite, args: vec![] },
            Gene { op: OpCode::Photosynthesize, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8);
        vm.energy = 1000; // Give plenty of energy

        // Loop until mutation or timeout
        let mut found = false;
        for _ in 0..100 {
            vm.step();
            if vm.output.iter().any(|s| s.contains("OVERHEAT")) {
                found = true;
                break;
            }
        }
        assert!(found, "Should warn about overheating eventually");
    }

    #[test]
    fn test_cryostasis() {
        // Set heat grid to 0 everywhere to ensure stasis
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.context_loc = (8, 8);

        // Manually freeze the world
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                vm.heat_grid[y][x] = 0;
            }
        }

        let ip_before = vm.ip;
        vm.step();
        let ip_after = vm.ip;

        assert_eq!(ip_before, ip_after, "Should be frozen in cryostasis (IP shouldn't move)");
        assert!(vm.output.iter().any(|s| s.contains("CRYOSTASIS")), "Should log cryostasis");
    }
}
