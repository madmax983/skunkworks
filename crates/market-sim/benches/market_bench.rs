use criterion::{black_box, criterion_group, criterion_main, Criterion};
use market_sim::{Grid, Particle};
use rand::Rng;

fn bench_grid_update(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let width = 100;
    let height = 100;

    c.bench_function("grid_update_100x100", |b| {
        b.iter_with_setup(
            || {
                let mut grid = Grid::new(width, height);
                // Populate 20% of the grid with particles
                for _ in 0..(width * height / 5) {
                    let x = rng.gen_range(0..width);
                    let y = rng.gen_range(0..height);
                    let p = if rng.gen_bool(0.5) {
                        Particle::Bid(rng.gen())
                    } else {
                        Particle::Ask(rng.gen())
                    };
                    grid.set(x, y, p);
                }
                grid
            },
            |mut grid| {
                black_box(grid.update());
            },
        )
    });
}

criterion_group!(benches, bench_grid_update);
criterion_main!(benches);
