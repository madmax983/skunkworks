#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::nova_biome::Biome;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_terraform_and_sense() {
        let mut vm = make_vm();

        // Terraform center (8,8) radius 1 to Swamp (1)
        // Stack: [ ..., biome_id, radius ]
        vm.stack.push(Value::Int(1)); // Swamp
        vm.stack.push(Value::Int(1)); // Radius

        let op = OpCode::Terraform;
        chimera_lang::vm::nova::exec_nova_op(&mut vm, op, &[]);

        // Verify via SenseBiome
        let op_sense = OpCode::SenseBiome;
        chimera_lang::vm::nova::exec_nova_op(&mut vm, op_sense, &[]);

        if let Some(Value::Int(id)) = vm.stack.pop() {
            assert_eq!(id, 1, "Expected Swamp (1)");
        } else {
            panic!("SenseBiome failed");
        }

        // Verify Neighbor is Swamp too (radius 1)
        vm.context_loc = (8, 9);
        chimera_lang::vm::nova::exec_nova_op(&mut vm, OpCode::SenseBiome, &[]);
        if let Some(Value::Int(id)) = vm.stack.pop() {
            assert_eq!(id, 1, "Expected Swamp (1) at neighbor");
        } else {
            panic!("SenseBiome failed at neighbor");
        }

        // Verify Far is Plains (0)
        vm.context_loc = (8, 12);
        chimera_lang::vm::nova::exec_nova_op(&mut vm, OpCode::SenseBiome, &[]);
        if let Some(Value::Int(id)) = vm.stack.pop() {
            assert_eq!(id, 0, "Expected Plains (0) at far");
        } else {
            panic!("SenseBiome failed at far");
        }
    }

    #[test]
    fn test_diffusion_biome_effect() {
        // Plains
        let mut vm_plains = make_vm();
        vm_plains.hormone_grid[8][8][0] = 100;

        // Swamp
        let mut vm_swamp = make_vm();
        vm_swamp.biome_grid[8][8] = Biome::Swamp;
        // Neighbors must also be swamp for fair comparison of "environment"?
        // Or just center.
        // Logic: avg = (self * inertia + sum_neighbors) / (inertia + neighbor_count)
        // Neighbors are 0 initially.
        // So we just need center to have different inertia.
        vm_swamp.hormone_grid[8][8][0] = 100;

        chimera_lang::vm::nova::diffuse_hormones(&mut vm_plains);
        chimera_lang::vm::nova::diffuse_hormones(&mut vm_swamp);

        let plains_val = vm_plains.hormone_grid[8][8][0];
        let swamp_val = vm_swamp.hormone_grid[8][8][0];

        println!("Plains: {}, Swamp: {}", plains_val, swamp_val);

        // Plains (Inertia 4): (100*4 + 0)/8 = 50
        // Swamp (Inertia 12): (100*12 + 0)/16 = 75

        assert!(
            swamp_val > plains_val,
            "Swamp should retain more hormone due to stagnation"
        );
        assert_eq!(plains_val, 50);
        assert_eq!(swamp_val, 75);
    }
}
