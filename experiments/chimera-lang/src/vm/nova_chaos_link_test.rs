#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_biome::Biome;
    use crate::vm::nova_chaos_link;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_chaos_sync_low() {
        let mut vm = make_vm();
        vm.chaos_struct.grid[0][0] = 0.1; // Low chaos
        vm.chaos_struct.r_grid[0][0] = 3.2; // Stable r

        nova_chaos_link::sync_biomes_from_chaos(&mut vm);

        assert_eq!(vm.biome_grid[0][0], Biome::Tundra);
    }

    #[test]
    fn test_chaos_sync_high() {
        let mut vm = make_vm();
        vm.chaos_struct.grid[0][0] = 0.9; // High chaos
        vm.chaos_struct.r_grid[0][0] = 3.2; // Stable r

        nova_chaos_link::sync_biomes_from_chaos(&mut vm);

        assert_eq!(vm.biome_grid[0][0], Biome::Volcanic);
    }

    #[test]
    fn test_chaos_sync_glitch() {
        let mut vm = make_vm();
        vm.chaos_struct.grid[0][0] = 0.7; // Medium-High
        vm.chaos_struct.r_grid[0][0] = 3.9; // Chaotic r

        nova_chaos_link::sync_biomes_from_chaos(&mut vm);

        assert_eq!(vm.biome_grid[0][0], Biome::Glitch);
    }

    #[test]
    fn test_chaos_sync_garden() {
        let mut vm = make_vm();
        vm.chaos_struct.grid[0][0] = 0.5; // Medium
        vm.chaos_struct.r_grid[0][0] = 3.4; // Stable r

        nova_chaos_link::sync_biomes_from_chaos(&mut vm);

        assert_eq!(vm.biome_grid[0][0], Biome::Garden);
    }
}
