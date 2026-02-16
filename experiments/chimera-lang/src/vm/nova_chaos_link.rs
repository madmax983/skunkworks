use super::{ChimeraVM, nova_biome::Biome};

pub fn sync_biomes_from_chaos(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    for y in 0..size {
        for x in 0..size {
            let chaos_val = vm.chaos_struct.grid[y][x];
            let r_val = vm.chaos_struct.r_grid[y][x];

            let new_biome = if r_val > 3.8 && chaos_val > 0.6 {
                // High chaos parameters -> Glitch
                Biome::Glitch
            } else if chaos_val < 0.2 {
                // Low activity -> Tundra (Frozen)
                Biome::Tundra
            } else if chaos_val > 0.8 {
                // High activity -> Volcanic
                Biome::Volcanic
            } else if r_val < 3.5 {
                // Stable regime -> Garden
                Biome::Garden
            } else {
                // Normal
                Biome::Plains
            };

            // Only update if different to avoid constant flickering if we had hysteresis (not implemented yet)
            // But simple mapping is fine for now.
            vm.biome_grid[y][x] = new_biome;
        }
    }
}
