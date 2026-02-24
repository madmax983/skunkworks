#[cfg(test)]
mod tests {
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::ast::{Dna, Helix, Strand};
    use chimera_lang::vm::nova_fluid;

    #[test]
    fn test_wind_accumulation_overflow() {
        let dna = Dna { evolution_config: None, helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);

        // Target cell: (8, 8)
        let target_y = 8;
        let target_x = 8;

        // Set up multiple cells to blow wind towards (8, 8)
        // Max wind is 10.
        // We can use cells where (y + dy) == 8 and (x + dx) == 8
        // So dy = 8 - y, dx = 8 - x.
        // And |dy| <= 10, |dx| <= 10.

        // Let's iterate over the grid and set wind if possible
        for y in 0..16 {
            for x in 0..16 {
                if y == target_y && x == target_x {
                    continue;
                }

                let dy = target_y as i64 - y as i64;
                let dx = target_x as i64 - x as i64;

                if dy.abs() <= 10 && dx.abs() <= 10 {
                    // Set wind to point to (8, 8)
                    // But wait, calculate_decay multiplies by 0.95.
                    // If we set wind to 10, decayed is 9.
                    // If we set wind to -10, decayed is -9.

                    // We need to ensure we don't accidentally exceed MAX_WIND in input?
                    // No, input is clamped to MAX_WIND when updating grid,
                    // but we can manually set grid here for testing state.
                    // process_fluid reads vm.wind_grid.

                    // Let's strictly use valid winds (<= 10)
                    vm.wind_grid[y][x] = (dy as i8, dx as i8);
                }
            }
        }

        // Now run process_fluid.
        // Many cells will add their decayed wind to new_wind[8][8].
        // This should panic in debug mode due to overflow if += is unchecked/default.
        nova_fluid::process_fluid(&mut vm);
    }
}
