#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;


    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    #[cfg(feature = "elektra")]
    fn test_voltage_patch() {
        // 1. Create a battery at (0,0) with 100V
        // 2. Patch (0,0) to EnergyRegen (Target 0)
        // 3. Step and verify energy increases

        let genes = vec![
            // Battery(100, 0, 0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // V
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // X
            Gene {
                op: OpCode::Battery,
                args: vec![],
            },
            // Patch(0, 0, 0, 0) -> Voltage(0) at (0,0) to Energy(0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Source Type (Voltage)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // X
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Target (EnergyRegen)
            Gene {
                op: OpCode::Patch,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Initial execution to setup battery and patch
        for _ in 0..10 {
            vm.step();
        }

        // Verify patch exists
        assert!(vm.patch_bay.contains_key(&(0, 0)));
        assert_eq!(vm.voltage_grid[0][0], 100.0);

        // Record energy
        let energy_before = vm.energy;

        // Step once
        vm.step();

        // Expected gain: 100V * 0.1 = 10 Energy.
        // Cost per step: 1 (or variable).
        // Net change approx +9.

        let energy_after = vm.energy;
        assert!(
            energy_after > energy_before,
            "Energy should increase due to patch ({} -> {})",
            energy_before,
            energy_after
        );
    }
}
