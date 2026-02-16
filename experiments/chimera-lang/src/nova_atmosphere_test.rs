#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;
    use crate::value::Value;

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_aeolus_wind() {
        // [ push(5) chronostasis() push(2) push(10) aeolus() sense_wind() ]
        // We use Chronostasis to freeze physics so wind doesn't advect away immediately.
        // Stack: [angle=2, strength=10]
        // Angle 2 = East (0, 1). Strength 10.
        // Expected Wind: (0, 10).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Chronostasis,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // Angle
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // Strength
            Gene {
                op: OpCode::Aeolus,
                args: vec![],
            },
            Gene {
                op: OpCode::SenseWind,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        let (cy, cx) = vm.context_loc;

        vm.step(); // push 5
        vm.step(); // chronostasis
        vm.step(); // push 2
        vm.step(); // push 10
        vm.step(); // aeolus

        let (wy, wx) = vm.wind_grid[cy][cx];
        assert_eq!(wy, 0);
        assert_eq!(wx, 10); // East * 10

        vm.step(); // sense_wind
        let wx_val = vm.stack.pop().unwrap();
        let wy_val = vm.stack.pop().unwrap();
        assert_eq!(wx_val, Value::Int(10));
        assert_eq!(wy_val, Value::Int(0));
    }

    #[test]
    fn test_storm_moisture() {
        // Stack: [intensity=50, radius=2]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            }, // Intensity
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // Radius
            Gene {
                op: OpCode::Storm,
                args: vec![],
            },
            Gene {
                op: OpCode::SenseMoisture,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        let (cy, cx) = vm.context_loc;

        vm.step();
        vm.step();
        vm.step(); // storm

        assert!(vm.moisture_grid[cy][cx] >= 50);

        vm.step(); // sense
        let m_val = vm.stack.pop().unwrap();
        if let Value::Int(m) = m_val {
            // Moisture decays naturally (0.98), so it might be slightly less than 50
            assert!(m >= 40);
        } else {
            panic!("Expected Int");
        }
    }

    #[test]
    fn test_wind_advection() {
        // 1. Set wind to East (0, 10)
        // 2. Secrete hormone at center
        // 3. Run a step (triggers diffusion)
        // 4. Compare East vs West neighbors

        let mut vm = make_vm(vec![]);
        let (cy, cx) = (8, 8);
        vm.context_loc = (cy, cx);

        // Wind East
        // Set wind at Center and neighbors to East
        vm.wind_grid[cy][cx - 1] = (0, 10);
        vm.wind_grid[cy][cx] = (0, 10);
        vm.wind_grid[cy][cx + 1] = (0, 10);

        // Secrete
        vm.hormone_grid[cy][cx][0] = 1000;

        // Run physics manually
        crate::vm::nova::diffuse_hormones(&mut vm);

        let west = vm.hormone_grid[cy][cx - 1][0];
        let east = vm.hormone_grid[cy][cx + 1][0];

        // East should receive more because wind blows West -> East
        assert!(
            east > west,
            "East {} should be > West {} with East wind",
            east,
            west
        );
    }

    #[test]
    fn test_cloud_shadow() {
        let mut vm = make_vm(vec![]);
        let (cy, cx) = (8, 8);
        vm.context_loc = (cy, cx);

        // Case 1: Clear Sky
        vm.light_grid[cy][cx] = 1000;
        // Run light diffusion
        crate::vm::nova::diffuse_light(&mut vm);
        let clear_val = vm.light_grid[cy][cx];

        // Case 2: Cloudy
        let mut vm2 = make_vm(vec![]);
        vm2.context_loc = (cy, cx);
        vm2.light_grid[cy][cx] = 1000;
        vm2.moisture_grid[cy][cx] = 50; // Max opacity
        crate::vm::nova::diffuse_light(&mut vm2);
        let cloudy_val = vm2.light_grid[cy][cx];

        assert!(
            cloudy_val < clear_val,
            "Cloudy {} should be < Clear {}",
            cloudy_val,
            clear_val
        );
    }
}
