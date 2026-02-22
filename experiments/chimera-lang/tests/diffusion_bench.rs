#[cfg(test)]
mod tests {
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::nova;
    use std::time::Instant;

    #[test]
    fn bench_diffusion() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);

        // Setup some state
        for y in 0..16 {
            for x in 0..16 {
                vm.hormone_grid[y][x] = [100, 200, 300];
                vm.waste_grid[y][x] = 100;
            }
        }

        let start = Instant::now();
        for _ in 0..10_000 {
            nova::diffuse_hormones(&mut vm);
            nova::diffuse_waste(&mut vm);
        }
        let duration = start.elapsed();
        println!("Diffusion 10k iters took: {:?}", duration);
    }
}
